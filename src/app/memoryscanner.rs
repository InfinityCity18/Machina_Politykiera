use std::error::Error;

use nix::{
    sys::ptrace::{attach, detach},
    unistd::Pid,
};

use crate::app::{memoryaddress::MemoryAddress, scansettings::ScanSettings, App};

// we need to hold the memory values for displaying ehhhh
// truly

pub struct MemoryScanner {
    matching_addresses: Vec<MemoryAddress>,
}

impl MemoryScanner {
    /// Creates new instance of `MemoryScanner`
    pub fn new() -> Self {
        Self {
            matching_addresses: vec![],
        }
    }

    pub fn first_scan(&mut self, scan_settings: ScanSettings) -> Result<(), Box<dyn Error>> {
        let process = scan_settings.process();
        attach(Pid::from_raw(process.pid()))?;

        detach(Pid::from_raw(process.pid()), None)?;
        Ok(())
    }

    pub fn next_scan(&mut self, scan_settings: ScanSettings) {}
}

/*
use std::{fs::File, io::{Read, Seek, SeekFrom, Write}, thread, time::Duration};

use nix::{sys::ptrace::{attach, cont, detach}, unistd::Pid};
use procfs::process::{MMapPath, Process};
fn main() {
    let pid = 2891028;
    let me = Process::new(pid).unwrap();
    attach(Pid::from_raw(pid)).unwrap();
    let mut mem = File::options().read(true).write(true).open(format!("/proc/{}/mem", pid)).unwrap();
    let maps = me.maps().unwrap();

    let hello = "hello".to_string();

    for map in maps {
        if map.pathname == MMapPath::Heap {
            mem.seek(SeekFrom::Start(map.address.0)).unwrap();
            let mut buf = vec![0; (map.address.1 - map.address.0) as usize];
            mem.read_exact(&mut buf).unwrap();
            let idx = buf.windows(5).position(|p| p == b"hello").unwrap();
            let mut buf = [0; 1];
            mem.seek(SeekFrom::Start(map.address.0 + idx as u64 + 1)).unwrap();
            mem.read_exact(&mut buf).unwrap();
            println!("BUFOR MEOW : {:?}", buf);
            mem.write(&[b'n']).unwrap();
        }
    }
    detach(Pid::from_raw(pid), None).unwrap();
    println!("{}", hello);
}
*/
