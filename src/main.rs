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
    })
    .ignore_default_theme()
    .resizable(false)
    .title("Rogger")
    .run();
}
