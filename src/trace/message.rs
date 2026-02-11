use std::fmt;

mod r#type;
use r#type::Type;

#[derive(Debug, PartialEq)]
pub enum Level {
    Info,
    Warning,
    Error,
    Debug,
    Unknown,
}

impl From<u32> for Level {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Info,
            1 => Self::Warning,
            2 => Self::Error,
            3 => Self::Debug,
            _ => Self::Unknown,
        }
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Level::Info => write!(f, "INFO"),
            Level::Warning => write!(f, "WARN"),
            Level::Error => write!(f, "ERROR"),
            Level::Debug => write!(f, "DEBUG"),
            Level::Unknown => write!(f, "UNKNOWN"),
        }
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
