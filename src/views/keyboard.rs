use vizia::prelude::*;

use crate::key_code::KeyCode;

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
        Keyboard::default().build(cx, |cx| {
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
