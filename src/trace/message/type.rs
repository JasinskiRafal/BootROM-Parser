use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Type {
    BootCoreBootRomVersion,
    Unknown,
}

impl From<u32> for Type {
    fn from(value: u32) -> Self {
        match value {
            0x155 => Self::BootCoreBootRomVersion,
            _ => Self::Unknown,
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::BootCoreBootRomVersion => write!(f, "BOOTCORE - Boot ROM version"),
            _ => write!(f, "Unknown code!"),
        }
    }
}
