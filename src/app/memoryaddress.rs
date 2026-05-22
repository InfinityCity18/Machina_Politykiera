use std::{error::Error, fs::File, io::{Read, Seek}, rc::Rc};

use nix::{sys::ptrace::attach, unistd::Pid};
use procfs::process::Process;
use crate::app::scansettings::{ScanValue, ScanValueType};

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
        let mut file = File::open(format!("/proc/{}/mem", self.process.pid()))?;
        file.seek(std::io::SeekFrom::Start(self.address as u64))?;
        let mut bytes = Vec::with_capacity(self.val_type.len());
        file.read_to_end(&mut bytes)?;
        Ok(ScanValue::convert_from_bytes(&bytes, self.val_type)?)
    }
}
