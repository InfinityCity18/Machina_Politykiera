use nix::{
    sys::ptrace::{attach, detach},
    unistd::Pid,
};
use procfs::process::Process;
use std::{
    error::Error,
    fs::File,
    io::{Read, Seek, Write},
    rc::Rc,
};

use crate::app::scansettings::ScanValue;
use crate::app::scansettings::ScanValueType;
#[derive(Clone)]
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
        let mut file = File::open(format!("/proc/{}/mem", self.process.pid()))?;
        file.seek(std::io::SeekFrom::Start(self.address as u64))?;
        let mut bytes = Vec::with_capacity(self.val_type.len());
        file.read_to_end(&mut bytes)?;
        detach(Pid::from_raw(self.process.pid()), None)?;
        Ok(ScanValue::convert_from_bytes(&bytes, self.val_type)?)
    }

    pub fn set_value(&self, val_type: ScanValue) -> Result<(), Box<dyn Error>> {
        assert_eq!(self.val_type, ScanValueType::from(&val_type));
        attach(Pid::from_raw(self.process.pid()))?;
        let mut file = File::open(format!("/proc/{}/mem", self.process.pid()))?;
        file.seek(std::io::SeekFrom::Start(self.address as u64))?;
        file.write(&val_type.as_bytes())?;
        detach(Pid::from_raw(self.process.pid()), None)?;
        Ok(())
    }

    pub fn to_string(&self) -> String {
        let pid = match self.process.stat() {
            Ok(stat) => format!("{:<8}", stat.pid),
            Err(_err) => "Error - couldn't parse name or pid".to_string(),
        };

        format!(
            "{:<8}{:.12}{:.12}",
            pid,
            self.address,
            self.val_type.to_string()
        )
    }
}

impl PartialEq for MemoryAddress {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address && self.process.pid() == other.process.pid()
    }
}
