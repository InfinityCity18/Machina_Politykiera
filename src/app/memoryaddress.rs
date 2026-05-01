use procfs::process::Process;

pub struct MemoryAddress {
    process: Process,
    offset: usize,
}
