//! BootROM Trace Parsing Module
//!
//! This module handles the parsing and representation of STM32 BootROM traces.
//! It includes functionality to detect trace patterns in memory dumps and
//! convert them into structured trace objects with human-readable output.

mod message;

use std::fmt;

/// Represents a single BootROM trace message
///
/// A trace contains timestamp information, a message level, and a message code
/// that identifies the specific BootROM event that occurred.
#[derive(Debug, PartialEq)]
pub struct Trace {
    /// 32-bit timestamp value from the BootROM trace
    timestamp: u32,

    /// Message level (INFO, WARNING, ERROR, DEBUG, or UNKNOWN)
    message_level: message::Level,

    /// Message code identifying the specific BootROM event
    message_code: message::Code,
}

impl Trace {
    /// Creates a new Trace instance
    ///
    /// # Arguments
    ///
    /// * `timestamp` - 32-bit timestamp value
    /// * `message_level` - Message level enum
    /// * `message_code` - Message code with optional argument
    ///
    /// # Returns
    ///
    /// A new Trace instance
    pub fn new(timestamp: u32, message_level: message::Level, message_code: message::Code) -> Self {
        Self {
            timestamp: timestamp,
            message_level: message_level,
            message_code: message_code,
        }
    }

    /// Attempts to parse a BootROM trace from a slice of u32 values
    ///
    /// This function looks for the BootROM trace magic number and validates
    /// the trace structure before extracting the timestamp, level, and code.
    ///
    /// # Arguments
    ///
    /// * `slice` - A slice of u32 values representing memory contents
    ///
    /// # Returns
    ///
    /// * `Some(Trace)` if a valid trace is found at the start of the slice
    /// * `None` if no valid trace is found
    ///
    /// # Trace Structure
    ///
    /// A valid BootROM trace has the following structure:
    /// - Index 0: Magic number (0xFFDDBB00)
    /// - Index 1: Size (0xC for 12 bytes or 0x10 for 16 bytes)
    /// - Index 2: Timestamp
    /// - Index 3: Message level
    /// - Index 4: Message code
    /// - Index 5: Argument (only for 16-byte traces)
    pub fn from_slice(slice: &[u32]) -> Option<Self> {
        /// BootROM trace magic number
        ///
        /// This constant identifies the start of a BootROM trace in memory.
        const START_VALUE: u32 = 0xffddbb00;

        let slice_start_value = slice[0];
        if slice_start_value != START_VALUE {
            return None;
        }

        let slice_size = slice[1];

        // Handle 12-byte traces (no argument)
        if slice_size == 0xc && slice.len() >= 5 {
            return Some(Self::new(
                slice[2],
                message::Level::from(slice[3]),
                message::Code::new(slice[4], None),
            ));
        }
        // Handle 16-byte traces (with argument)
        else if slice_size == 0x10 && slice.len() >= 6 {
            return Some(Self::new(
                slice[2],
                message::Level::from(slice[3]),
                message::Code::new(slice[4], Some(slice[5])),
            ));
        }

        None
    }

    /// Parses multiple BootROM traces from a vector of u32 values
    ///
    /// This function scans through the entire memory dump looking for
    /// BootROM trace patterns and extracts all valid traces found.
    ///
    /// # Arguments
    ///
    /// * `values` - A slice of u32 values representing the complete memory dump
    ///
    /// # Returns
    ///
    /// A vector containing all valid BootROM traces found in the input
    ///
    /// # Algorithm
    ///
    /// The function iterates through the input data, checking each position
    /// for the BootROM trace magic number. When found, it attempts to parse
    /// a complete trace using `from_slice()`.
    pub fn from_parsed_hex_dump(values: &[u32]) -> Vec<Trace> {
        let mut traces: Vec<Trace> = Vec::new();

        // Scan through the entire memory dump looking for traces
        for idx in 0..values.len() {
            let slice = &values[idx..];
            if let Some(trace) = Trace::from_slice(slice) {
                traces.push(trace);
            }
        }

        traces
    }
}

impl fmt::Display for Trace {
    /// Formats the trace for human-readable output
    ///
    /// The output format is: `[LEVEL] 0xtimestamp - message_description`
    /// For messages with arguments: `[LEVEL] 0xtimestamp - message_description (0xargument)`
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
    /// [INFO] 0x1234 - BOOTCORE - Boot ROM version
    ///
    /// // With argument
    /// [DEBUG] 0x5678 - SECBOOT - AuthImageSignatureOk (0xdeadbeef)
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] 0x{:x} - {}",
            self.message_level, self.timestamp, self.message_code
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn trace_test() {
        let correct_trace = Trace {
            timestamp: 0x1234,
            message_level: message::Level::Info,
            message_code: message::Code::new(0x1234, None),
        };
        let new_trace = Trace::new(
            0x1234,
            message::Level::Info,
            message::Code::new(0x1234, None),
        );
        assert_eq!(correct_trace, new_trace);
    }

    #[test]
    fn trace_fmt_test() {
        {
            let simple_trace = Trace::new(
                0x1234,
                message::Level::Info,
                message::Code::new(0x1234, None),
            );
            let correct_fmt = String::from("[INFO] 0x1234 - Unknown code!");
            let created_fmt = format!("{}", simple_trace);
            assert_eq!(correct_fmt, created_fmt);
        }

        {
            let simple_trace = Trace::new(
                0x1234,
                message::Level::Info,
                message::Code::new(0x1234, Some(0x1234)),
            );
            let correct_fmt = String::from("[INFO] 0x1234 - Unknown code! (0x1234)");
            let created_fmt = format!("{}", simple_trace);
            assert_eq!(correct_fmt, created_fmt);
        }

        {
            let simple_trace = Trace::new(
                0x1234,
                message::Level::Info,
                message::Code::new(0x155, None),
            );
            let correct_fmt = String::from("[INFO] 0x1234 - BOOTCORE BootRomVer");
            let created_fmt = format!("{}", simple_trace);
            assert_eq!(correct_fmt, created_fmt);
        }
    }

    #[test]
    fn trace_from_slice() {
        {
            let simple_trace = Trace::new(
                0x1234,
                message::Level::Info,
                message::Code::new(0x1234, None),
            );
            let value_slice = vec![0xffddbb00, 0xC, 0x1234, 0x0, 0x1234];
            let created_trace = Trace::from_slice(&value_slice).unwrap();
            assert_eq!(simple_trace, created_trace);
        }
        {
            let simple_trace = Trace::new(
                0x1234,
                message::Level::Info,
                message::Code::new(0x155, Some(0x1234)),
            );
            let value_slice = vec![0xffddbb00, 0x10, 0x1234, 0x0, 0x155, 0x1234];
            let created_trace = Trace::from_slice(&value_slice).unwrap();
            assert_eq!(simple_trace, created_trace);
        }
    }

    #[test]
    fn traces_from_hex_dump() {
        let hex_dump: Vec<u32> = vec![
            0xffddbb00, 0xC, 0x1234, 0x0, 0x1234, 0xffddbb00, 0x10, 0x1234, 0x0, 0x155, 0x1234,
        ];
        let correct_traces = vec![
            Trace::new(
                0x1234,
                message::Level::Info,
                message::Code::new(0x1234, None),
            ),
            Trace::new(
                0x1234,
                message::Level::Info,
                message::Code::new(0x155, Some(0x1234)),
            ),
        ];
        let traces = Trace::from_parsed_hex_dump(&hex_dump);
        assert_eq!(correct_traces, traces);
    }
}
