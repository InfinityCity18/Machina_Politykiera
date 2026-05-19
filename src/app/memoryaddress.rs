use std::{error::Error, fs::File, io::{Read, Seek}, rc::Rc};

use procfs::process::Process;

pub struct MemoryAddress {
    pub process: Rc<Process>,
    pub len: usize,
    pub offset: usize,
}

impl MemoryAddress {
    pub fn new(process: Rc<Process>, len: usize, offset: usize) -> Self {
        Self { process: process, len, offset }
    }

    pub fn matches(&self, pat: &[u8], file: &mut File) -> Result<bool, Box<dyn Error>> {
        let mut buf = vec![0; pat.len()];
        file.seek(std::io::SeekFrom::Start(self.offset as u64))?;
        if file.read(&mut buf)? == pat.len() {
            return Err(String::from("Read count didn't match pattern length").into());
        }
        if buf == pat {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
