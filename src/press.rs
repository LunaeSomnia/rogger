use std::fmt::Display;

pub enum PressType {
    Down,
    Up,
    Autorepeat,
    Other(u32),
}

impl PressType {
    pub fn is_other(&self) -> bool {
        match self {
            PressType::Other(_) => true,
            _ => false,
        }
    }
}

impl Display for PressType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            PressType::Autorepeat => "Autorepeat".to_string(),
            PressType::Down => "Down".to_string(),
            PressType::Up => "Up".to_string(),
            PressType::Other(other) => format!("0x{:x}", other),
        })
    }
}
