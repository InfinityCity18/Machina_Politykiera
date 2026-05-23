use std::{error::Error, fs::File, io::{Read, Seek, Write}, rc::Rc};

use nix::{sys::{ptrace::{attach, detach}, wait::waitpid}, unistd::Pid};
use procfs::process::Process;
use crate::app::scansettings::{ScanValue, ScanValueType};

#[derive(Clone, Debug)]
pub struct MemoryAddress {
    pub process: Rc<Process>,
    pub address: usize,
    pub val_type: ScanValueType
}

impl MemoryAddress {
    pub fn new(process: Rc<Process>, address: usize, val_type: ScanValueType) -> Self {
        Self { process: process, address, val_type}
    }

    pub fn matches(&self, pat: &[u8], file: &mut File) -> Result<bool, Box<dyn Error>> {
        let mut buf = vec![0; pat.len()];
        file.seek(std::io::SeekFrom::Start(self.address as u64))?;
        if file.read(&mut buf)? == pat.len() {
            return Err(String::from("Read count didn't match pattern length").into());
        }
        if buf == pat {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn read_value(&self) -> Result<ScanValue, Box<dyn Error>> {
        attach(Pid::from_raw(self.process.pid()))?;
        waitpid(Pid::from_raw(self.process.pid()), None)?;
        let mut file = File::open(format!("/proc/{}/mem", self.process.pid()))?;
        file.seek(std::io::SeekFrom::Start(self.address as u64))?;
        let mut bytes = Vec::with_capacity(self.val_type.len());
        std::io::Read::by_ref(&mut file).take(self.val_type.len() as u64).read_to_end(&mut bytes)?;
        detach(Pid::from_raw(self.process.pid()), None)?;
        Ok(ScanValue::convert_from_bytes(&bytes, self.val_type)?)
    }

    pub fn set_value(&self, val_type: ScanValue) -> Result<(), Box<dyn Error>> {
        assert_eq!(self.val_type, ScanValueType::from(&val_type));
        attach(Pid::from_raw(self.process.pid()))?;
        waitpid(Pid::from_raw(self.process.pid()), None)?;
        let mut file = File::options().write(true).open(format!("/proc/{}/mem", self.process.pid()))?;
        file.seek(std::io::SeekFrom::Start(self.address as u64))?;
        file.write(&val_type.as_bytes())?;
        detach(Pid::from_raw(self.process.pid()), None)?;
        Ok(())
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
        ms.first_scan(sett)?;
        let results = ms.addresses_and_values()?;
        let found_tuple = results.get(0).ok_or("Nothing at index 0")?;
        found_tuple.0.set_value(ScanValue::String("world hello".to_string()))?;
        let mut buf = [0];
        let mut file: File = read_fd.into();
        file.read(&mut buf)?;
        assert_eq!(buf[0], 1);
        Ok(())
    }
}
