use std::{error::Error, fs::File, io::{BufReader, Read, Seek}, rc::Rc};

use nix::{
    sys::ptrace::{attach, detach},
    unistd::Pid,
};
use procfs::process::{MMapPath, Process};

use crate::app::{memoryaddress::MemoryAddress, scansettings::ScanSettings};

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

    // Returns a list of addresses of process memory where value matches pattern 
    fn kmp(file: &mut File, pattern: &[u8], len: usize, process: Rc<Process>, start_offset: usize) -> Result<Vec<MemoryAddress>,Box<dyn Error>> {
        fn prefix_function(pat: &[u8]) -> Vec<usize> {
            let n = pat.len();
            let mut pi: Vec<usize> = Vec::new();
            for i in 1..n {
                let mut j = pi[i-1];
                while j > 0 && pat[i] != pat[j] {
                    j = pi[j-1];
                }
                if pat[i] == pat[j] {
                    j += 1;
                }
                pi[i] = j;
            }
            return pi;
        }

        let lps = prefix_function(pattern);
        let mut i = 0;
        let mut j = 0;
        let mut matching_addresses = Vec::new(); 
        let mut chr: u8 = 0;
        let mut bufread = BufReader::new(file);
        bufread.read(std::slice::from_mut(&mut chr))?;

        while i < len {
            if pattern[j] == chr {
                i += 1;
                j += 1;
                bufread.read(std::slice::from_mut(&mut chr))?;
            }
            if j == pattern.len() {
                matching_addresses.push( MemoryAddress::new(process.clone(), start_offset + i - j));
                j = lps[j - 1];
            } else if i < len && pattern[j] != chr {
                if j != 0 {
                    j = lps[j - 1];
                } else {
                    i += 1;
                    bufread.read(std::slice::from_mut(&mut chr))?;
                }
            }
        }
        return Ok(matching_addresses);
    }

    pub fn first_scan(&mut self, scan_settings: ScanSettings) -> Result<(), Box<dyn Error>> {
        let process = scan_settings.process();
        attach(Pid::from_raw(process.pid()))?;
        let mut file = File::open(format!("/proc/{}/mem", process.pid()))?;
        let mut addresses = Vec::new();

        for map in process.maps()? {
            if map.pathname == MMapPath::Heap || map.pathname == MMapPath::Stack {
                file.seek(std::io::SeekFrom::Start(map.address.0))?;
                let len = (map.address.1 - map.address.0) as usize;
                if let Ok(mut v) = MemoryScanner::kmp(&mut file, &scan_settings.value().as_bytes(), len, process.clone(), map.address.0 as usize) {
                    addresses.append(&mut v);
                }
                // ignoring err,  want to continue seeking pattern in other maps
            }
        }
        detach(Pid::from_raw(process.pid()), None)?;
        self.matching_addresses = addresses;
        Ok(())
    }

    pub fn next_scan(&mut self, scan_settings: ScanSettings) {
        
    }
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
