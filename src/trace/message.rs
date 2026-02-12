//! BootROM Message Module
//!
//! This module handles the representation and formatting of BootROM message
//! levels and codes. It provides enums for message levels and types, along
//! with functionality to convert raw numeric codes into human-readable strings.

use std::fmt;

use strum_macros;

mod r#type;
use r#type::Type;

/// Message level enum representing the severity of BootROM traces
///
/// BootROM messages are categorized into different levels indicating their
/// importance and severity during the boot process.
#[derive(Debug, PartialEq, strum_macros::FromRepr, strum_macros::Display)]
#[strum(serialize_all = "UPPERCASE")]
#[repr(u32)]
pub enum Level {
    /// Informational messages about normal boot progression
    Info = 0,

    /// Potential issues that don't prevent boot
    Warning = 1,

    /// Critical errors that may prevent successful boot
    Error = 2,

    /// Detailed debugging information
    Debug = 3,

    /// Unrecognized or invalid message levels
    Unknown,
}

impl From<u32> for Level {
    /// Converts a numeric value to a Level enum
    ///
    /// # Arguments
    ///
    /// * `value` - Numeric representation of the message level
    ///
    /// # Returns
    ///
    /// The corresponding Level enum value, or Level::Unknown if the value
    /// doesn't match any known level.
    fn from(value: u32) -> Self {
        Level::from_repr(value).unwrap_or(Level::Unknown)
    }
}

/// Represents a BootROM message code with optional argument
///
/// Each BootROM trace contains a message code that identifies the specific
/// event that occurred. Some messages include an additional argument that
/// provides more context about the event.
#[derive(Debug, PartialEq)]
pub struct Code {
    /// The message type enum identifying the specific BootROM event
    message_type: Type,

    /// Optional argument providing additional context for the message
    argument: Option<u32>,
}

impl Code {
    /// Creates a new Code instance
    ///
    /// # Arguments
    ///
    /// * `code` - Numeric message code
    /// * `argument` - Optional additional data for the message
    ///
    /// # Returns
    ///
    /// A new Code instance with the message type and argument
    pub fn new(code: u32, argument: Option<u32>) -> Self {
        Self {
            message_type: Type::from(code),
            argument: argument,
        }
    }
}

impl fmt::Display for Code {
    /// Formats the message code for human-readable output
    ///
    /// The output includes the message type description, and if an argument
    /// is present, it's displayed in hexadecimal format in parentheses.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to write to
    ///
    /// # Returns
    ///
    /// * `fmt::Result` - Result of the formatting operation
    ///
    /// # Examples
    ///
    /// ```
    /// // Without argument
    /// "BOOTCORE - Boot ROM version"
    ///
    /// // With argument
    /// "SECBOOT - AuthImageSignatureOk (0xdeadbeef)"
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message_type)?;

        if let Some(argument) = self.argument {
            write!(f, " (0x{:x})", argument)?;
        }

        write!(f, "")
    }
}
