use std::{error::Error, rc::Rc};

use procfs::process::Process;

pub struct ScanSettings {
    process: Rc<Process>,
    value: ScanValue,
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
}

#[derive(Clone, Copy)]
pub enum ScanValueType {
    Byte,
    Word,
    DWord,
    QWord,
    Float,
    Double,
    String(usize),
}

impl From<&ScanValue> for ScanValueType {
    fn from(value: &ScanValue) -> Self {
        use ScanValueType::*;
        match value {
            ScanValue::Byte(_) => Byte,
            ScanValue::Word(_) => Word,
            ScanValue::DWord(_) => DWord,
            ScanValue::QWord(_) => QWord,
            ScanValue::Float(_) => Float,
            ScanValue::Double(_) => Double,
            ScanValue::String(x) => String(x.len()),
        }
    }
}

impl ScanValueType {
    pub fn len(&self) -> usize {
        use ScanValueType::*;
        match self {
            Byte => size_of::<u8>(),
            Word => size_of::<u16>(),
            DWord => size_of::<u32>(),
            QWord => size_of::<u64>(),
            Float => size_of::<f32>(),
            Double => size_of::<f64>(),
            String(x) => *x,
        }
    }
}

impl ScanValue {
    pub fn convert_from_bytes(
        bytes: &[u8],
        val_type: ScanValueType,
    ) -> Result<ScanValue, Box<dyn Error>> {
        use ScanValueType::*;
        let returnv = match val_type {
            #[cfg(target_endian = "little")]
            Byte => ScanValue::Byte(u8::from_le_bytes(bytes.try_into()?)),
            #[cfg(target_endian = "big")]
            Byte => ScanValue::Byte(u8::from_be_bytes(bytes.try_into()?)),

            #[cfg(target_endian = "little")]
            Word => ScanValue::Word(u16::from_le_bytes(bytes.try_into()?)),
            #[cfg(target_endian = "big")]
            Word => ScanValue::Word(u16::from_be_bytes(bytes.try_into()?)),

            #[cfg(target_endian = "little")]
            DWord => ScanValue::DWord(u32::from_le_bytes(bytes.try_into()?)),
            #[cfg(target_endian = "big")]
            DWord => ScanValue::DWord(u32::from_be_bytes(bytes.try_into()?)),

            #[cfg(target_endian = "little")]
            QWord => ScanValue::QWord(u64::from_le_bytes(bytes.try_into()?)),
            #[cfg(target_endian = "big")]
            QWord => ScanValue::QWord(u64::from_be_bytes(bytes.try_into()?)),

            #[cfg(target_endian = "little")]
            Float => ScanValue::Float(f32::from_le_bytes(bytes.try_into()?)),
            #[cfg(target_endian = "big")]
            Float => ScanValue::Float(f32::from_be_bytes(bytes.try_into()?)),

            #[cfg(target_endian = "little")]
            Double => ScanValue::Double(f64::from_le_bytes(bytes.try_into()?)),
            #[cfg(target_endian = "big")]
            Double => ScanValue::Double(f64::from_be_bytes(bytes.try_into()?)),

            String(s) => ScanValue::String(std::str::from_utf8(bytes)?.to_string()),
        };
        Ok(returnv)
    }

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
        }
    }

    pub fn len(&self) -> usize {
        use ScanValue::*;
        match self {
            Byte(_) => size_of::<u8>(),
            Word(_) => size_of::<u16>(),
            DWord(_) => size_of::<u32>(),
            QWord(_) => size_of::<u64>(),
            Float(_) => size_of::<f32>(),
            Double(_) => size_of::<f64>(),
            String(x) => size_of_val(x),
        }
    }
}
