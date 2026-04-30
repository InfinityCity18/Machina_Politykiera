use procfs::{ProcResult, process::{self, Process}};

/// A struct for convenient holding of processes list
pub struct ProcessList {
    processes: Vec<Process>
}

impl ProcessList {

    /// Creates new, empty instance of `ProcessList`
    pub fn new() -> Self {
        Self { processes: vec![] }
    }

    /// Updates list of processes
    pub fn update(&mut self) {
        match process::all_processes() {
            Ok(proc_it) => {
                self.processes = proc_it.filter_map(|res| res.ok()).collect()
            },
            Err(err) => unimplemented!("Updating list of processes error handling is unimplemented")
        }
    }

    /// Returns a reference to list of processes
    pub fn get(&self) -> &Vec<Process> {
        &self.processes
    }
}