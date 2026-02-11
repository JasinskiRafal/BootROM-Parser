mod message;

use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Trace {
    timestamp: u32,
    message_level: message::Level,
    message_code: message::Code,
}

impl Trace {
    pub fn new(timestamp: u32, message_level: message::Level, message_code: message::Code) -> Self {
        Self {
            timestamp: timestamp,
            message_level: message_level,
            message_code: message_code,
        }
    }

    pub fn from_slice(slice: &[u32]) -> Option<Self> {
        const START_VALUE: u32 = 0xffddbb00;

        let slice_start_value = slice[0];
        if slice_start_value != START_VALUE {
            return None;
        }

        let slice_size = slice[1];
        if slice_size == 0xc && slice.len() >= 5 {
            return Some(Self::new(
                slice[2],
                message::Level::from(slice[3]),
                message::Code::new(slice[4], None),
            ));
        } else if slice_size == 0x10 && slice.len() >= 6 {
            return Some(Self::new(
                slice[2],
                message::Level::from(slice[3]),
                message::Code::new(slice[4], Some(slice[5])),
            ));
        }

        None
    }

    pub fn from_parsed_hex_dump(values: &[u32]) -> Vec<Trace> {
        let mut traces: Vec<Trace> = Vec::new();

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
            let correct_fmt = String::from("[INFO] 0x1234 - BOOTCORE - Boot ROM version");
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
