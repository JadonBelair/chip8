use macroquad::input::{get_last_key_pressed, KeyCode};

pub const KEYS: [KeyCode; 16] = [
    KeyCode::X,
    KeyCode::Key1,
    KeyCode::Key2,
    KeyCode::Key3,
    KeyCode::Q,
    KeyCode::W,
    KeyCode::E,
    KeyCode::A,
    KeyCode::S,
    KeyCode::D,
    KeyCode::Z,
    KeyCode::C,
    KeyCode::Key4,
    KeyCode::R,
    KeyCode::F,
    KeyCode::V
];

pub struct Keyboard {
    keys: [bool; 16]
}
    
impl Keyboard {
    pub fn new() -> Self {
        Self {
            keys: [false; 16]
        }
    }

    pub fn set_keys(&mut self, keys: [bool; 16]) {
        self.keys = keys;
    }

    pub fn is_down(&self, key: u8) -> bool {
        return self.keys[key as usize];
    }

    pub fn just_pressed(&self) -> Option<u8> {
        return if let Some(key) = get_last_key_pressed() {
            match key {
                KeyCode::X    => Some(0),
                KeyCode::Key1 => Some(1),
                KeyCode::Key2 => Some(2),
                KeyCode::Key3 => Some(3),
                KeyCode::Q    => Some(4),
                KeyCode::W    => Some(5),
                KeyCode::E    => Some(6),
                KeyCode::A    => Some(7),
                KeyCode::S    => Some(8),
                KeyCode::D    => Some(9),
                KeyCode::Z    => Some(10),
                KeyCode::C    => Some(11),
                KeyCode::Key4 => Some(12),
                KeyCode::R    => Some(13),
                KeyCode::F    => Some(14),
                KeyCode::V    => Some(15),
                _ => None
            }
        } else {
            None
        };
    }
}