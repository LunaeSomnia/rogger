use vizia::prelude::*;

use crate::key_code::KeyCode;

use super::Keyboard;

#[derive(Clone, Copy, Lens)]
pub struct KeyboardKey {
    key_code: KeyCode,
}

impl KeyboardKey {
    pub fn new<'a>(cx: &'a mut Context, key_code: KeyCode, label: &'a str) -> Handle<'a, Self> {
        Self { key_code }
            .build(cx, |_| {})
            .text(label)
            .checked(Keyboard::keys.map(move |v| v[key_code.to_value() as usize]))
    }
}

impl View for KeyboardKey {
    fn element(&self) -> Option<&'static str> {
        Some("key")
    }
}
