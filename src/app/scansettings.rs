use std::{io::Read, rc::Rc};

use procfs::process::Process;

pub struct ScanSettings {
    process: Rc<Process>,
    value: ScanValue, //type len (byte,word,dword, etc.) or enum of types
                      //first scan or next scan
                      //unsigned or signed bool
}

impl ScanSettings {
    pub fn process(&self) -> &Rc<Process> {
        &self.process
    }

    pub fn value(&self) -> &ScanValue {
        &self.value
    }
}

pub enum ScanValue {
    Byte(u8),
    Word(u16),
    DWord(u32),
    QWord(u64),
    Float(f32),
    Double(f64),
    String(String),
    Array(Vec<u8>), //if very bored, change to &[u8], but fun with lifetimes is not worth it for now
}

impl ScanValue {
    pub fn as_bytes(&self) -> Box<[u8]> {
        use ScanValue::*;
        match self {
            #[cfg(target_endian = "little")]
            Byte(n) => Box::new(n.to_le_bytes()),
            #[cfg(target_endian = "big")]
            Byte(n) => Box::new(n.to_be_bytes()),

            #[cfg(target_endian = "little")]
            Word(n) => Box::new(n.to_le_bytes()),
            #[cfg(target_endian = "big")]
            Word(n) => Box::new(n.to_be_bytes()),

            #[cfg(target_endian = "little")]
            DWord(n) => Box::new(n.to_le_bytes()),
            #[cfg(target_endian = "big")]
            DWord(n) => Box::new(n.to_be_bytes()),

            #[cfg(target_endian = "little")]
            QWord(n) => Box::new(n.to_le_bytes()),
            #[cfg(target_endian = "big")]
            QWord(n) => Box::new(n.to_be_bytes()),

            #[cfg(target_endian = "little")]
            Float(n) => Box::new(n.to_le_bytes()),
            #[cfg(target_endian = "big")]
            Float(n) => Box::new(n.to_be_bytes()),

            #[cfg(target_endian = "little")]
            Double(n) => Box::new(n.to_le_bytes()),
            #[cfg(target_endian = "big")]
            Double(n) => Box::new(n.to_be_bytes()),

            String(s) => Box::from(s.as_bytes()),

            Array(v) => Box::from(v.as_slice()),
        }
    }
}
