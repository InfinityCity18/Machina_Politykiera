use nix::{
    sys::ptrace::{attach, detach},
    sys::wait::waitpid,
    unistd::Pid,
};
use std::{
    error::Error,
    fs::File,
    io::{Read, Seek, Write},
    rc::Rc,
};

use crate::app::scansettings::ScanValue;
use crate::app::scansettings::ScanValueType;
use procfs::process::Process;
#[derive(Clone, Debug)]
pub struct MemoryAddress {
    pub process: Rc<Process>,
    pub address: usize,
    pub val_type: ScanValueType,
}

impl MemoryAddress {
    pub fn new(process: Rc<Process>, address: usize, val_type: ScanValueType) -> Self {
        Self {
            process,
            address,
            val_type,
        }
    }

    pub fn matches(&self, pat: &[u8], file: &mut File) -> Result<bool, Box<dyn Error>> {
        let mut buf = vec![0; pat.len()];
        file.seek(std::io::SeekFrom::Start(self.address as u64))?;
        if file.read(&mut buf)? == pat.len() {
            return Err(String::from("Read count didn't match pattern length").into());
        }
        Ok(buf == pat)
    }

    pub fn read_value(&self) -> Result<ScanValue, Box<dyn Error>> {
        attach(Pid::from_raw(self.process.pid()))?;
        waitpid(Pid::from_raw(self.process.pid()), None)?;
        let mut file = File::open(format!("/proc/{}/mem", self.process.pid()))?;
        file.seek(std::io::SeekFrom::Start(self.address as u64))?;
        let mut bytes = Vec::with_capacity(self.val_type.len());
        std::io::Read::by_ref(&mut file)
            .take(self.val_type.len() as u64)
            .read_to_end(&mut bytes)?;
        detach(Pid::from_raw(self.process.pid()), None)?;
        Ok(ScanValue::convert_from_bytes(&bytes, self.val_type)?)
    }

    pub fn set_value(&self, val_type: ScanValue) -> Result<(), Box<dyn Error>> {
        if let ScanValue::String(_) = val_type && matches!(self.val_type, ScanValueType::String(_)) {
            if val_type.len() > self.val_type.len() {
                return Err(format!("Inputted string in set value is too long: inputted = {:?}, pinned = {:?}", self.val_type, val_type).into())
            }
        } else {
            assert_eq!(self.val_type, ScanValueType::from(&val_type));
        }
        attach(Pid::from_raw(self.process.pid())).inspect_err(|x| log::error!("attaching in set value failed : {x}"))?;
        waitpid(Pid::from_raw(self.process.pid()), None).inspect_err(|x| log::error!("waitpid in set value failed : {x}"))?;
        let mut file = File::options().write(true).open(format!("/proc/{}/mem", self.process.pid())).inspect_err(|x| log::error!("file open in set value failed : {x}"))?;
        file.seek(std::io::SeekFrom::Start(self.address as u64)).inspect_err(|x| log::error!("file seek in set value failed : {x}"))?;
        file.write(&val_type.as_bytes()).inspect_err(|x| log::error!("file write in set value failed : {x}"))?;
        detach(Pid::from_raw(self.process.pid()), None).inspect_err(|x| log::error!("detach in set value failed : {x}"))?;
        log::info!("Set value = {:?} of address = {}", &val_type.as_bytes(),self.address);
        Ok(())
    }

    pub fn to_string(&self) -> String {
        let pid = match self.process.stat() {
            Ok(stat) => format!("{:<8}", stat.pid),
            Err(_err) => "Error - couldn't parse name or pid".to_string(),
        };

        format!(
            "{:<8}{:<20}{:<12}",
            pid,
            self.address,
            self.val_type.to_string()
        )
    }
}

impl PartialEq for MemoryAddress {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
            && self.process.pid() == other.process.pid()
            && self.val_type == other.val_type
    }
}

#[cfg(test)]
mod tests {
    use crate::app::MemoryScanner;
    use std::{error::Error, fs::File, io::{Read, Write}, os::fd::AsFd, time::Duration};
    use procfs::process::Process;
    use crate::app::scansettings::{ScanSettings, ScanValue};
    use std::rc::Rc;
    #[test]
    fn test_set_value() -> Result<(), Box<dyn Error>>{
        let mut child_pid= 0;
        let (read_fd, write_fd) = nix::unistd::pipe()?;

        match unsafe{nix::unistd::fork()} {
            Ok(nix::unistd::ForkResult::Parent { child, .. }) => {
                nix::unistd::close(write_fd)?;

                child_pid = child.as_raw();

                std::thread::sleep(Duration::from_millis(100));
            }
            Ok(nix::unistd::ForkResult::Child) => {
                nix::unistd::close(read_fd)?;

                let hello = "hello world".to_string();
                std::thread::sleep(Duration::from_secs(7));
                let mut file: File = write_fd.into();
                file.write_all(&[(hello == "world hello") as u8]).unwrap();
                let _s = std::hint::black_box(hello);
                drop(file);
                unsafe { nix::libc::_exit(0) };
            }
            Err(_) => println!("Fork failed"),
        }

        let mut ms = MemoryScanner::new();
        let self_proc = Process::new(child_pid)?;
        let sett = ScanSettings::new(Rc::new(self_proc),ScanValue::String("hello world".to_string()));
        ms.first_scan(sett, usize::MAX)?;
        let results = ms.addresses_and_values(&ms.matching_addresses)?;
        let found_tuple = results.get(0).ok_or("Nothing at index 0")?;
        found_tuple.0.set_value(ScanValue::String("world hello".to_string()))?;
        let mut buf = [0];
        let mut file: File = read_fd.into();
        file.read(&mut buf)?;
        assert_eq!(buf[0], 1);
        Ok(())
    }
}
