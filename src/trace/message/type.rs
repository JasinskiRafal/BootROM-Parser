use strum_macros;

#[derive(Debug, PartialEq, strum_macros::FromRepr, strum_macros::Display)]
#[repr(u32)]
pub enum Type {
    #[strum(to_string = "BOOTCORE - Boot ROM version")]
    BootCoreBootRomVersion = 0x155,
    #[strum(to_string = "Unknown code!")]
    Unknown,
}

impl From<u32> for Type {
    fn from(value: u32) -> Self {
        Type::from_repr(value).unwrap_or(Type::Unknown)
    }
}
