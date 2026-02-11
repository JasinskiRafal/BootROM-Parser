mod message;

use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Trace {
    size: u32,
    timestamp: u32,
    message_level: message::Level,
    message_code: message::Code,
}

const START_VALUE: u32 = 0xffddbb00;

impl Trace {
    pub fn new(
        size: u32,
        timestamp: u32,
        message_level: u32,
        message_code: u32,
        argument: Option<u32>,
    ) -> Self {
        Self {
            size: size,
            timestamp: timestamp,
            message_level: message::Level::from(message_level),
            message_code: message::Code::new(message_code, argument),
        }
    }
}

impl fmt::Display for Trace {
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
            size: 0x10,
            timestamp: 0x1234,
            message_level: message::Level::Info,
            message_code: message::Code::new(0x1234, None),
        };
        let new_trace = Trace::new(0x10, 0x1234, 0x0, 0x1234, None);
        assert_eq!(correct_trace, new_trace);
    }

    #[test]
    fn trace_fmt_test() {
        {
            let simple_trace = Trace::new(0x10, 0x1234, 0x0, 0x1234, None);
            let correct_fmt = String::from("[INFO] 0x1234 - Unknown code!");
            let created_fmt = format!("{}", simple_trace);
            assert_eq!(correct_fmt, created_fmt);
        }

        {
            let simple_trace = Trace::new(0x10, 0x1234, 0x0, 0x1234, Some(0x1234));
            let correct_fmt = String::from("[INFO] 0x1234 - Unknown code! (0x1234)");
            let created_fmt = format!("{}", simple_trace);
            assert_eq!(correct_fmt, created_fmt);
        }

        {
            let simple_trace = Trace::new(0x10, 0x1234, 0x0, 0x155, None);
            let correct_fmt = String::from("[INFO] 0x1234 - BOOTCORE - Boot ROM version");
            let created_fmt = format!("{}", simple_trace);
            assert_eq!(correct_fmt, created_fmt);
        }
    }
}
