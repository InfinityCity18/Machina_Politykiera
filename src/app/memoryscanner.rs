use std::{error::Error, fs::File, io::{BufReader, Read, Seek}, rc::Rc};

use nix::{
    sys::ptrace::{attach, detach},
    unistd::Pid,
};
use procfs::process::{MMapPath, Process};

use crate::app::{memoryaddress::MemoryAddress, scansettings::{ScanSettings, ScanValue}};

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

    pub fn addresses_and_values(&self) -> Result<Vec<(usize, Vec<u8>)>, Box<dyn Error>> {
        // its like this cuz of assumption of only one process being scanned
        let process = &self.matching_addresses.get(0).ok_or("No matching addresses")?.process; 
        attach(Pid::from_raw(process.pid()))?;
        let mut file = File::open(format!("/proc/{}/mem", process.pid()))?;
        let mut v = Vec::new();
        for addr in &self.matching_addresses {
            let mut buf = Vec::with_capacity(addr.val_type.len());
            if file.seek(std::io::SeekFrom::Start(addr.address as u64)).is_err() {
                continue;
            }
            file.read_to_end(&mut buf)?;
            v.push((addr.address, buf));
        }
        detach(Pid::from_raw(process.pid()), None)?;
        Ok(v)
    }

    // Returns a list of addresses of process memory where value matches pattern 
    fn kmp(file: &mut File, val: &ScanValue, len_of_memory: usize, process: Rc<Process>, start_offset: usize) -> Result<Vec<MemoryAddress>,Box<dyn Error>> {
        let pattern = val.as_bytes();
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

        let lps = prefix_function(&pattern);
        let mut i = 0;
        let mut j = 0;
        let mut matching_addresses = Vec::new(); 
        let mut chr: u8 = 0;
        let mut bufread = BufReader::new(file);
        bufread.read(std::slice::from_mut(&mut chr))?;

        while i < len_of_memory {
            if pattern[j] == chr {
                i += 1;
                j += 1;
                bufread.read(std::slice::from_mut(&mut chr))?;
            }
            if j == pattern.len() {
                matching_addresses.push( MemoryAddress::new(process.clone(), start_offset + i - j,  (val).into()));
                j = lps[j - 1];
            } else if i < len_of_memory && pattern[j] != chr {
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
                if let Ok(mut v) = MemoryScanner::kmp(&mut file, scan_settings.value(), len, process.clone(), map.address.0 as usize) {
                    addresses.append(&mut v);
                }
                // ignoring err,  want to continue seeking pattern in other maps
            }
        }
        detach(Pid::from_raw(process.pid()), None)?;
        self.matching_addresses = addresses;
        Ok(())
    }

    pub fn next_scan(&mut self, scan_settings: ScanSettings) -> Result<(), Box<dyn Error>> {
        let process = scan_settings.process();
        attach(Pid::from_raw(process.pid()))?;
        let mut file = File::open(format!("/proc/{}/mem", process.pid()))?;

        for addr in &self.matching_addresses {
            file.seek(std::io::SeekFrom::Start(addr.address.try_into()?))?;

        }
        self.matching_addresses.retain(|addr| {
            if file.seek(std::io::SeekFrom::Start(addr.address as u64)).is_err() {
                return false
            }
            let mut buf = Vec::with_capacity(addr.val_type.len());
            // Decided to compare even if it didnt read
            let _ = file.read_to_end(&mut buf);
            scan_settings.value().as_bytes() == buf.as_slice().into()
        });
        detach(Pid::from_raw(process.pid()), None)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
use std::time::Duration;

use procfs::process::Process;

use crate::app::scansettings::ScanValue;

use super::MemoryScanner;
    use super::ScanSettings;
    #[test]
    fn test_memory_scanner() -> Result<(), Box<dyn std::error::Error>> {

        let mut child_pid= 0;
        match unsafe{nix::unistd::fork()} {
            Ok(nix::unistd::ForkResult::Parent { child, .. }) => {
            child_pid = child.as_raw();
        }
            Ok(nix::unistd::ForkResult::Child) => {
            let hello = "hello world";
            std::thread::sleep(Duration::from_secs(20));
            unsafe { nix::libc::_exit(0) };
        }
            Err(_) => println!("Fork failed"),
        }

        let searched_string = "hello world";
        let mut ms = MemoryScanner::new();
        let self_proc = Process::new(child_pid)?;
        let sett = ScanSettings::new(Rc::new(self_proc),ScanValue::String("hello world".to_string()));
        ms.first_scan(sett)?;
        let results = ms.addresses_and_values()?;
        let found_tuple = results.get(0).ok_or("Nothing at index 0")?;
        assert_eq!(searched_string.as_ptr(), found_tuple.0 as *const u8);
        Ok(())
    }
}
