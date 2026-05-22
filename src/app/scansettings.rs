use std::{error::Error, rc::Rc};

use procfs::process::Process;

pub struct ScanSettings {
    process: Rc<Process>,
    value: ScanValue,
}

impl ScanSettings {
    pub fn new(process: Rc<Process>, value: ScanValue) -> ScanSettings {
        ScanSettings { process, value }
    }

    pub fn process(&self) -> &Rc<Process> {
        &self.process
    }

    pub fn value(&self) -> &ScanValue {
        &self.value
    }
}

#[derive(PartialEq, Debug)]
pub enum ScanValue {
    Byte(u8),
    Word(u16),
    DWord(u32),
    QWord(u64),
    Float(f32),
    Double(f64),
    String(String),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

    pub fn to_string(&self) -> String {
        use ScanValueType::*;
        match self {
            Byte => "Byte".to_string(),
            Word => "Word".to_string(),
            DWord => "DWord".to_string(),
            QWord => "QWord".to_string(),
            Float => "Float".to_string(),
            Double => "Double".to_string(),
            String(x) => format!("String({})", x),
        }
    }
}

impl ScanValue {
    pub fn from_user_input(input: String, typ: ScanValueType) -> Result<ScanValue, Box<dyn Error>> {
        use ScanValue::*;
        let parsed = match typ {
            ScanValueType::Byte => Byte(input.parse()?),
            ScanValueType::Word => Word(input.parse()?),
            ScanValueType::DWord => DWord(input.parse()?),
            ScanValueType::QWord => QWord(input.parse()?),
            ScanValueType::Float => Float(input.parse()?),
            ScanValueType::Double => Double(input.parse()?),
            ScanValueType::String(l) => {
                if input.len() > l {
                    return Err("Inputted string is too long".into());
                } else {
                    String(input)
                }
            }
        };
        Ok(parsed)
    }
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
            String(_) => ScanValue::String(std::str::from_utf8(bytes)?.to_string()),
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

    pub fn to_string(&self) -> String {
        match self {
            ScanValue::Byte(v) => v.to_string(),
            ScanValue::Word(v) => v.to_string(),
            ScanValue::DWord(v) => v.to_string(),
            ScanValue::QWord(v) => v.to_string(),
            ScanValue::Float(v) => v.to_string(),
            ScanValue::Double(v) => v.to_string(),
            ScanValue::String(v) => v.clone(),
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

#[cfg(test)]
mod tests {
    use super::ScanValueType::*;
    use crate::app::scansettings::ScanValue;
    use std::error::Error;

    #[test]
    fn test_usr_input_byte() -> Result<(), Box<dyn Error>> {
        use super::ScanValueType::Byte;
        assert_eq!(
            ScanValue::from_user_input("12".to_string(), Byte)?,
            ScanValue::Byte(12)
        );
        Ok(())
    }
    #[test]
    fn test_usr_input_byte_overflow() {
        assert!(ScanValue::from_user_input("512".to_string(), Byte).is_err());
    }

    #[test]
    fn test_usr_input_float() {
        assert_eq!(
            ScanValue::from_user_input("1.22143".to_string(), Float).unwrap(),
            ScanValue::Float(1.22143)
        );
    }

    #[test]
    fn test_usr_input_string() {
        assert_eq!(
            ScanValue::from_user_input("hello world".to_string(), String("hello world".len()))
                .unwrap(),
            ScanValue::String("hello world".to_string())
        );
    }

    #[test]
    fn test_usr_input_string_too_long() {
        assert!(ScanValue::from_user_input("hello".to_string(), String(2)).is_err());
    }

    #[test]
    fn test_scanval_from_bytes() {
        assert_eq!(
            ScanValue::convert_from_bytes(&['a' as u8, 'b' as u8, 'c' as u8], String(5)).unwrap(),
            ScanValue::String("abc".to_string())
        );
    }

    #[test]
    fn test_scanval_from_bytes_err_utf8() {
        assert!(
            ScanValue::convert_from_bytes(&['a' as u8, 'b' as u8, 211 as u8], String(5)).is_err()
        );
    }

    #[test]
    fn test_scanval_le_be() {
        let b = ScanValue::Word(5);
        assert_eq!(b.len(), 2);
        #[cfg(target_endian = "big")]
        assert_eq!(b.as_bytes(), Box::from([0b00000000, 0b00000101]));
        #[cfg(target_endian = "little")]
        assert_eq!(b.as_bytes(), Box::from([0b00000101, 0b00000000]));
    }
}
