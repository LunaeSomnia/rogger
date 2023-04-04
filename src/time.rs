use std::{
    fmt::Display,
    io::{Cursor, Read},
};

use chrono::{Datelike, NaiveDateTime, Timelike};

pub const TIMEVAL_SIZE: usize = 16; // In bytes
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct TimeVal {
    sec: i64,
    nsec: i64,
}

impl TimeVal {
    pub fn from_bytes(bytes: &[u8; TIMEVAL_SIZE]) -> Result<Self, ()> {
        let mut cursor = Cursor::new(bytes);

        let mut seconds = [0u8; TIMEVAL_SIZE / 2];
        let mut nseconds = [0u8; TIMEVAL_SIZE / 2];

        let seconds_result = cursor.read_exact(&mut seconds);
        let nseconds_result = cursor.read_exact(&mut nseconds);

        if seconds_result.is_ok() && nseconds_result.is_ok() {
            Ok(TimeVal {
                sec: i64::from_le_bytes(seconds),
                nsec: i64::from_le_bytes(nseconds),
            })
        } else {
            Err(())
        }
    }
}

impl Display for TimeVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let naive_time = NaiveDateTime::from_timestamp_opt(self.sec, self.nsec as u32).unwrap();
        f.write_str(&format!(
            "{}/{}/{} {}:{}:{}",
            naive_time.day(),
            naive_time.month(),
            naive_time.year(),
            naive_time.hour(),
            naive_time.minute(),
            naive_time.second()
        ))
    }
}
