use std::{
    fs::File,
    io::{BufReader, Read},
    process::Command,
};

use vizia::prelude::*;

use crate::{
    key::InputEvent,
    press::PressType,
    views::{Keyboard, KeyboardEvent},
};

mod key;
mod key_code;
mod press;
mod time;
mod views;

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet("./styles.css").unwrap();

        Keyboard::new(cx);

        cx.spawn(|cx| {
            let device_file_name = get_keyboard_device_filenames()[2].clone();

            let device_file = File::open(device_file_name).expect("Couldn't open input device");
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
        });
    })
    .ignore_default_theme()
    .resizable(false)
    .inner_size((800, 400))
    .run();
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
