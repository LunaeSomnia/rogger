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
                HStack::new(cx, |cx| {
                    KeyboardKey::new(cx, KeyCode::KEY_ESC, "ESC");
                    Element::new(cx).class("key-space");
                    KeyboardKey::new(cx, KeyCode::KEY_F1, "F1");
                    KeyboardKey::new(cx, KeyCode::KEY_F2, "F2");
                    KeyboardKey::new(cx, KeyCode::KEY_F3, "F3");
                    KeyboardKey::new(cx, KeyCode::KEY_F4, "F4");
                    Element::new(cx).class("key-space").class("x0-5");
                    KeyboardKey::new(cx, KeyCode::KEY_F5, "F5");
                    KeyboardKey::new(cx, KeyCode::KEY_F6, "F6");
                    KeyboardKey::new(cx, KeyCode::KEY_F7, "F7");
                    KeyboardKey::new(cx, KeyCode::KEY_F8, "F8");
                    Element::new(cx).class("key-space").class("x0-5");
                    KeyboardKey::new(cx, KeyCode::KEY_F9, "F9");
                    KeyboardKey::new(cx, KeyCode::KEY_F10, "F10");
                    KeyboardKey::new(cx, KeyCode::KEY_F11, "F11");
                    KeyboardKey::new(cx, KeyCode::KEY_F12, "F12");
                })
                .class("keyboard-row");

                HStack::new(cx, |cx| {
                    KeyboardKey::new(cx, KeyCode::KEY_GRAVE, "`~");
                    KeyboardKey::new(cx, KeyCode::KEY_1, "1!");
                    KeyboardKey::new(cx, KeyCode::KEY_2, "2@");
                    KeyboardKey::new(cx, KeyCode::KEY_3, "3#");
                    KeyboardKey::new(cx, KeyCode::KEY_4, "4$");
                    KeyboardKey::new(cx, KeyCode::KEY_5, "5%");
                    KeyboardKey::new(cx, KeyCode::KEY_6, "6^");
                    KeyboardKey::new(cx, KeyCode::KEY_7, "7&");
                    KeyboardKey::new(cx, KeyCode::KEY_8, "8*(");
                    KeyboardKey::new(cx, KeyCode::KEY_9, "9)");
                    KeyboardKey::new(cx, KeyCode::KEY_0, "0)");
                    KeyboardKey::new(cx, KeyCode::KEY_MINUS, "-_");
                    KeyboardKey::new(cx, KeyCode::KEY_EQUAL, "=+");
                    KeyboardKey::new(cx, KeyCode::KEY_BACKSPACE, "BACK").class("x2");
                })
                .class("keyboard-row");

                HStack::new(cx, |cx| {
                    KeyboardKey::new(cx, KeyCode::KEY_TAB, "TAB").class("x1-5");
                    KeyboardKey::new(cx, KeyCode::KEY_Q, "Q");
                    KeyboardKey::new(cx, KeyCode::KEY_W, "W");
                    KeyboardKey::new(cx, KeyCode::KEY_E, "E");
                    KeyboardKey::new(cx, KeyCode::KEY_R, "R");
                    KeyboardKey::new(cx, KeyCode::KEY_T, "T");
                    KeyboardKey::new(cx, KeyCode::KEY_Y, "Y");
                    KeyboardKey::new(cx, KeyCode::KEY_U, "U");
                    KeyboardKey::new(cx, KeyCode::KEY_I, "I");
                    KeyboardKey::new(cx, KeyCode::KEY_O, "O");
                    KeyboardKey::new(cx, KeyCode::KEY_P, "P");
                    KeyboardKey::new(cx, KeyCode::KEY_LEFTBRACE, "[{");
                    KeyboardKey::new(cx, KeyCode::KEY_RIGHTBRACE, "]}");
                    KeyboardKey::new(cx, KeyCode::KEY_BACKSLASH, "\\|").class("x1-5");
                })
                .class("keyboard-row");

                HStack::new(cx, |cx| {
                    KeyboardKey::new(cx, KeyCode::KEY_CAPSLOCK, "CAPS").class("x1-75");
                    KeyboardKey::new(cx, KeyCode::KEY_A, "A");
                    KeyboardKey::new(cx, KeyCode::KEY_S, "S");
                    KeyboardKey::new(cx, KeyCode::KEY_D, "D");
                    KeyboardKey::new(cx, KeyCode::KEY_F, "F");
                    KeyboardKey::new(cx, KeyCode::KEY_G, "G");
                    KeyboardKey::new(cx, KeyCode::KEY_H, "H");
                    KeyboardKey::new(cx, KeyCode::KEY_J, "J");
                    KeyboardKey::new(cx, KeyCode::KEY_K, "K");
                    KeyboardKey::new(cx, KeyCode::KEY_L, "L");
                    KeyboardKey::new(cx, KeyCode::KEY_SEMICOLON, ";:");
                    KeyboardKey::new(cx, KeyCode::KEY_APOSTROPHE, "\'\"");
                    KeyboardKey::new(cx, KeyCode::KEY_ENTER, "ENTER").class("x2-25");
                })
                .class("keyboard-row");

                HStack::new(cx, |cx| {
                    KeyboardKey::new(cx, KeyCode::KEY_LEFTSHIFT, "LSHIFT").class("x2-25");
                    KeyboardKey::new(cx, KeyCode::KEY_Z, "Z");
                    KeyboardKey::new(cx, KeyCode::KEY_X, "X");
                    KeyboardKey::new(cx, KeyCode::KEY_C, "C");
                    KeyboardKey::new(cx, KeyCode::KEY_V, "V");
                    KeyboardKey::new(cx, KeyCode::KEY_B, "B");
                    KeyboardKey::new(cx, KeyCode::KEY_N, "N");
                    KeyboardKey::new(cx, KeyCode::KEY_M, "M");
                    KeyboardKey::new(cx, KeyCode::KEY_COMMA, ",<");
                    KeyboardKey::new(cx, KeyCode::KEY_DOT, ".>");
                    KeyboardKey::new(cx, KeyCode::KEY_SLASH, "/?");
                    KeyboardKey::new(cx, KeyCode::KEY_RIGHTSHIFT, "RSHIFT").class("x2-75");
                })
                .class("keyboard-row");

                HStack::new(cx, |cx| {
                    KeyboardKey::new(cx, KeyCode::KEY_LEFTCTRL, "LCTRL").class("x1-5");
                    KeyboardKey::new(cx, KeyCode::KEY_LEFTMETA, "SUPER");
                    KeyboardKey::new(cx, KeyCode::KEY_LEFTALT, "LALT").class("x1-25");
                    KeyboardKey::new(cx, KeyCode::KEY_SPACE, "SPACE").class("x6-5");
                    KeyboardKey::new(cx, KeyCode::KEY_RIGHTALT, "RALT").class("x1-25");
                    KeyboardKey::new(cx, KeyCode::KEY_RIGHTMETA, "SUPER");
                    KeyboardKey::new(cx, KeyCode::KEY_HELP, "HELP");
                    KeyboardKey::new(cx, KeyCode::KEY_RIGHTCTRL, "RCTRL").class("x1-5");
                })
                .class("keyboard-row");
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
