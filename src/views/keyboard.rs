use std::{
    fs::File,
    io::{BufReader, Read},
    process::Command,
};

use vizia::prelude::*;

use crate::{key::InputEvent, key_code::KeyCode, press::PressType};

use super::KeyboardKey;

pub const KEY_COUNT: usize = 248;

#[derive(Clone, Copy, Lens)]
pub struct Keyboard {
    pub keys: [bool; KEY_COUNT],
}

impl Default for Keyboard {
    fn default() -> Self {
        Self {
            keys: [false; KEY_COUNT],
        }
    }
}

impl Keyboard {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        let h = Keyboard::default()
            .build(cx, |cx| {
                KeyboardKey::new(cx, KeyCode::KEY_ESC, "ESC");
                KeyboardKey::new(cx, KeyCode::KEY_F1, "F1");
                KeyboardKey::new(cx, KeyCode::KEY_F2, "F2");
                KeyboardKey::new(cx, KeyCode::KEY_F3, "F3");
                KeyboardKey::new(cx, KeyCode::KEY_F4, "F4");
                KeyboardKey::new(cx, KeyCode::KEY_F5, "F5");
                KeyboardKey::new(cx, KeyCode::KEY_F6, "F6");
                KeyboardKey::new(cx, KeyCode::KEY_F7, "F7");
                KeyboardKey::new(cx, KeyCode::KEY_F8, "F8");
                KeyboardKey::new(cx, KeyCode::KEY_F9, "F9");
                KeyboardKey::new(cx, KeyCode::KEY_F10, "F10");
                KeyboardKey::new(cx, KeyCode::KEY_F11, "F11");
                KeyboardKey::new(cx, KeyCode::KEY_F12, "F12");
            })
            .on_build(|cx| {
                get_keyboard_events(cx);
            });

        h
    }
}

pub enum KeyboardEvent {
    PressDown(KeyCode),
    PressUp(KeyCode),
}

impl View for Keyboard {
    fn element(&self) -> Option<&'static str> {
        Some("keyboard")
    }

    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            KeyboardEvent::PressDown(v) => {
                self.keys[v.to_value() as usize] = true;
                println!("DOWN {}", v)
            }
            KeyboardEvent::PressUp(v) => {
                self.keys[v.to_value() as usize] = false;
                println!("UP   {}", v)
            }
        })
    }
}

pub fn get_keyboard_events(cx: &mut EventContext) {
    for device in get_keyboard_device_filenames() {
        cx.spawn(|cx| {
            println!("DEBUG: Opening device {device}");
            let device_file = File::open(device).expect("Couldn't open input device");
            let mut reader = BufReader::new(device_file);

            let mut buffer = [0u8; 24];
            loop {
                reader
                    .read_exact(&mut buffer)
                    .expect("Couldn't read from device");

                if !buffer.is_empty() {
                    let input_event = InputEvent::from_bytes(&buffer).unwrap();
                    let press_type = input_event.get_press_type();
                    if input_event.is_key() {
                        dbg!(input_event.get_key_code());
                        match press_type {
                            PressType::Down => {
                                cx.emit(KeyboardEvent::PressDown(input_event.get_key_code()))
                                    .unwrap();
                            }
                            PressType::Up => {
                                cx.emit(KeyboardEvent::PressUp(input_event.get_key_code()))
                                    .unwrap();
                            }
                            _ => {}
                        };
                    }
                }
            }
        })
    }
}

fn get_keyboard_device_filenames() -> Vec<String> {
    let mut command_str = "grep -E 'Handlers|EV' /proc/bus/input/devices".to_string();
    command_str.push_str("| grep -B1 120013");
    command_str.push_str("| grep -Eo event[0-9]+");

    let res = Command::new("sh")
        .arg("-c")
        .arg(command_str)
        .output()
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });
    let res_str = std::str::from_utf8(&res.stdout).unwrap();

    let mut filenames = Vec::new();
    for file in res_str.trim().split('\n') {
        let mut filename = "/dev/input/".to_string();
        filename.push_str(file);
        filenames.push(filename);
    }

    filenames
}
