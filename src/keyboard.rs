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
            let mut ret = None;
            
            for i in 0..KEYS.len() {
                if key == KEYS[i] {
                    ret = Some(i as u8);
                    break;
                }
            }

            ret
        } else {
            None
        };
    }
}