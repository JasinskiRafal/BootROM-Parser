use std::fmt;

use strum_macros;

mod r#type;
use r#type::Type;

#[derive(Debug, PartialEq, strum_macros::FromRepr, strum_macros::Display)]
#[strum(serialize_all = "UPPERCASE")]
#[repr(u32)]
pub enum Level {
    Info = 0,
    Warning = 1,
    Error = 2,
    Debug = 3,
    Unknown,
}

impl From<u32> for Level {
    fn from(value: u32) -> Self {
        Level::from_repr(value).unwrap_or(Level::Unknown)
    }
}

#[derive(Debug, PartialEq)]
pub struct Code {
    message_type: Type,
    argument: Option<u32>,
}

impl Code {
    pub fn new(code: u32, argument: Option<u32>) -> Self {
        Self {
            message_type: Type::from(code),
            argument: argument,
        }
    }
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message_type)?;

        if let Some(argument) = self.argument {
            write!(f, " (0x{:x})", argument)?;
        }

        write!(f, "")
    }
}
