use nix::{
    sys::{
        ptrace::{attach, detach},
        wait::waitpid,
    },
    unistd::Pid,
};
use procfs::process::{MMapPath, Process};
use ratatui::widgets::{ListItem, ListState};
use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read, Seek},
    rc::Rc,
};

use crate::app::{
    memoryaddress::MemoryAddress,
    scansettings::{ScanSettings, ScanValue, ScanValueType},
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{List, StatefulWidget, Widget},
};

// we need to hold the memory values for displaying ehhhh
// truly

pub struct MemoryScanner<'a> {
    matching_addresses: Vec<MemoryAddress>,

    pub widget_state: ListState,
    list_items: Vec<ListItem<'a>>,
}

impl MemoryScanner<'_> {
    /// Creates new instance of `MemoryScanner`
    pub fn new() -> Self {
        let mut me = Self {
            matching_addresses: vec![],
            widget_state: ListState::default(),
            list_items: vec![],
        };
        me.widget_state.select_first();
        me
    }

    pub fn update_list(&mut self) {
        if self.matching_addresses.len() == 0 {
            self.list_items = vec![];
            return;
        }
        if self.widget_state.selected() == None {
            self.widget_state.select_first();
        }
        match self.addresses_and_values() {
            Ok(items) => {
                self.list_items = items
                    .into_iter()
                    .map(|(mem_addr, val)| {
                        let res_value = match ScanValue::convert_from_bytes(
                            val.as_slice(),
                            mem_addr.val_type,
                        ) {
                            Ok(v) => v.to_string(),
                            Err(_) => "Error".to_string(),
                        };

                        ListItem::new(format!("{}{}", mem_addr.to_string(), res_value))
                    })
                    .collect()
            }
            Err(_) => (),
        }
    }

    pub fn get_selected(&self) -> Option<MemoryAddress> {
        match self.widget_state.selected() {
            Some(i) => Some(self.matching_addresses[i].clone()),
            None => None,
        }
    }

    pub(super) fn addresses_and_values(&self) -> Result<Vec<(MemoryAddress, Vec<u8>)>, Box<dyn Error>> {
        // its like this cuz of assumption of only one process being scanned
        let process = &self
            .matching_addresses
            .get(0)
            .ok_or("No matching addresses")?
            .process;

        attach(Pid::from_raw(process.pid()))?;
        waitpid(Pid::from_raw(process.pid()), None)?;
        let mut file = File::open(format!("/proc/{}/mem", process.pid()))?;
        let mut v = Vec::new();
        for addr in &self.matching_addresses {
            let mut buf = Vec::with_capacity(addr.val_type.len());
            if file
                .seek(std::io::SeekFrom::Start(addr.address as u64))
                .is_err()
            {
                continue;
            }
            file.by_ref()
                .take(addr.val_type.len() as u64)
                .read_to_end(&mut buf)?;
            v.push((addr.clone(), buf));
        }
        drop(file);
        detach(Pid::from_raw(process.pid()), None)?;
        Ok(v)
    }

    // Returns a list of addresses of process memory where value matches pattern
    fn kmp(
        file: &File,
        val: &ScanValue,
        len_of_memory: usize,
        process: Rc<Process>,
        start_offset: usize,
    ) -> Result<Vec<MemoryAddress>, Box<dyn Error>> {
        let pattern = val.as_bytes();
        fn prefix_function(pat: &[u8]) -> Vec<usize> {
            let n = pat.len();
            let mut pi: Vec<usize> = vec![0; n];
            for i in 1..n {
                let mut j = pi[i - 1];
                while j > 0 && pat[i] != pat[j] {
                    j = pi[j - 1];
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
                j += 1;
                i += 1;
                if j < pattern.len() {
                    let _ = bufread.read(std::slice::from_mut(&mut chr));
                }
            } else if j != 0 {
                j = lps[j - 1];
            } else {
                i += 1;
                let _ = bufread.read(std::slice::from_mut(&mut chr));
            }

            if j == pattern.len() {
                matching_addresses.push(MemoryAddress::new(
                    process.clone(),
                    start_offset + i - j,
                    (val).into(),
                ));
                j = lps[j - 1];
                let _ = bufread.read(std::slice::from_mut(&mut chr));
            }
        }
        return Ok(matching_addresses);
    }

    pub fn first_scan(&mut self, scan_settings: ScanSettings) -> Result<(), Box<dyn Error>> {
        let process = scan_settings.process();
        attach(Pid::from_raw(process.pid()))?;
        waitpid(Pid::from_raw(process.pid()), None)?;
        let mut file = File::open(format!("/proc/{}/mem", process.pid()))?;
        let mut addresses = Vec::new();

        for map in process.maps()? {
            if map.pathname == MMapPath::Heap
                || map.pathname == MMapPath::Stack
                || map.pathname == MMapPath::Anonymous
            {
                file.seek(std::io::SeekFrom::Start(map.address.0))?;
                let len = (map.address.1 - map.address.0) as usize;

                if let Ok(mut v) = MemoryScanner::kmp(
                    &mut file,
                    scan_settings.value(),
                    len,
                    process.clone(),
                    map.address.0 as usize,
                ) {
                    addresses.append(&mut v);
                }
                // ignoring err,  want to continue seeking pattern in other maps
            }
        }
        drop(file);
        detach(Pid::from_raw(process.pid()), None)?;
        self.matching_addresses = addresses;
        Ok(())
    }

    pub fn next_scan(&mut self, scan_settings: ScanSettings) -> Result<(), Box<dyn Error>> {
        let process = scan_settings.process();
        if process.pid()
            != self
                .matching_addresses
                .get(0)
                .ok_or("No addresses in list")?
                .process
                .pid()
        {
            return Err("Processes not matching".into());
        }
        attach(Pid::from_raw(process.pid()))?;
        waitpid(Pid::from_raw(process.pid()), None)?;
        let mut file = File::open(format!("/proc/{}/mem", process.pid()))?;

        self.matching_addresses.retain(|addr| {
            if file
                .seek(std::io::SeekFrom::Start(addr.address as u64))
                .is_err()
            {
                return false;
            }
            let mut buf = Vec::with_capacity(addr.val_type.len());
            // Decided to compare even if it didnt read
            let _ = file
                .by_ref()
                .take(addr.val_type.len() as u64)
                .read_to_end(&mut buf);
            scan_settings.value().as_bytes() == buf.as_slice().into()
        });
        drop(file);
        detach(Pid::from_raw(process.pid()), None)?;
        Ok(())
    }
    pub fn clear(&mut self) {
        self.matching_addresses = vec![];
    }
}
impl Widget for &mut MemoryScanner<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let list = List::new(self.list_items.clone()).highlight_symbol(">> ");

        StatefulWidget::render(list, area, buf, &mut self.widget_state);
    }
}
#[cfg(test)]
mod tests {

    use std::fs::remove_file;
    use std::io::Seek;
    use std::io::Write;
    use std::rc::Rc;
    use std::time::Duration;

    use nix::libc::remove;
    use procfs::process::Process;

    use crate::app::scansettings::ScanValue;

    use super::MemoryScanner;
    use super::ScanSettings;
    #[test]
    fn test_memory_scanner() -> Result<(), Box<dyn std::error::Error>> {
        let mut child_pid = 0;
        match unsafe { nix::unistd::fork() } {
            Ok(nix::unistd::ForkResult::Parent { child, .. }) => {
                child_pid = child.as_raw();
                std::thread::sleep(Duration::from_millis(100));
            }
            Ok(nix::unistd::ForkResult::Child) => {
                let hello = "hello world".to_string();
                std::thread::sleep(Duration::from_secs(60));
                let _s = std::hint::black_box(hello);
                unsafe { nix::libc::_exit(0) };
            }
            Err(_) => println!("Fork failed"),
        }

        let searched_string = "hello world";
        let mut ms = MemoryScanner::new();
        let self_proc = Process::new(child_pid)?;
        let sett = ScanSettings::new(
            Rc::new(self_proc),
            ScanValue::String("hello world".to_string()),
        );
        ms.first_scan(sett)?;
        let results = ms.addresses_and_values()?;
        let found_tuple = results.get(0).ok_or("Nothing at index 0")?;
        assert_eq!(searched_string, String::from_utf8(found_tuple.1.clone())?);
        Ok(())
    }

    #[test]

    fn test_kmp() -> Result<(), Box<dyn std::error::Error>> {
        let mut file = std::fs::File::options()
            .create(true)
            .read(true)
            .write(true)
            .open("hello.txt")?;
        file.write(b"wdn ajubwdjawjd kabhello world dwa iodhwid ahwih")?;
        file.seek(std::io::SeekFrom::Start(0))?;

        let addresses = MemoryScanner::kmp(
            &file,
            &ScanValue::String("hello world".to_string()),
            110,
            Rc::new(Process::myself()?),
            0,
        )?;
        remove_file("hello.txt")?;
        assert_eq!(addresses[0].address, 19);
        Ok(())
    }
}
