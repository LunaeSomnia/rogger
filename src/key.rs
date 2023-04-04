use std::{
    fmt::{Display, Write},
    io::{Cursor, Read},
};

use crate::{
    key_code::KeyCode,
    press::PressType,
    time::{TimeVal, TIMEVAL_SIZE},
};

pub const INPUT_EVENT_SIZE: usize = 24; // in bytes

#[derive(Clone, Copy, Debug)]
pub struct InputEvent {
    time: TimeVal,
    event_type: u16,
    code: KeyCode,
    value: u32,
}

impl InputEvent {
    pub fn from_bytes(bytes: &[u8; INPUT_EVENT_SIZE]) -> Result<Self, ()> {
        let mut cursor = Cursor::new(bytes);

        let mut time = [0u8; TIMEVAL_SIZE];
        let mut event_type = [0u8; 2];
        let mut code = [0u8; 2];
        let mut value = [0u8; 4];

        let time_result = cursor.read_exact(&mut time);
        let event_type_result = cursor.read_exact(&mut event_type);
        let code_result = cursor.read_exact(&mut code);
        let value_result = cursor.read_exact(&mut value);

        if time_result.is_ok()
            && event_type_result.is_ok()
            && code_result.is_ok()
            && value_result.is_ok()
        {
            let timeval = TimeVal::from_bytes(&time).unwrap();
            let code = u16::from_le_bytes(code);
            Ok(InputEvent {
                time: timeval,
                event_type: u16::from_le_bytes(event_type),
                code: KeyCode::from_value(code),
                value: u32::from_le_bytes(value),
            })
        } else {
            Err(())
        }
    }

    pub fn get_press_type(&self) -> PressType {
        match self.value {
            2 => PressType::Autorepeat,
            1 => PressType::Down,
            0 => PressType::Up,
            _ => PressType::Other(self.value),
        }
    }

    pub fn is_key(&self) -> bool {
        self.event_type > 0
    }

    pub fn get_key_code(&self) -> KeyCode {
        self.code
    }
}

impl Display for InputEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('[').unwrap();
        self.time.fmt(f)?;
        f.write_str(&format!(
            "] {} {} {}",
            self.code,
            self.event_type,
            self.get_press_type()
        ))
    }
}
