use std::mem;
use winapi::shared::ntdef::SHORT;

#[derive(Debug)]
pub enum KeyState {
    UP,
    DOWN,
    //TOGGLED,
}

impl KeyState {
    pub fn convert(state: SHORT) -> KeyState {
        const HIGH_BIT_MASK: SHORT = 1 << 15;
        if state & HIGH_BIT_MASK != 0 {
            KeyState::DOWN
        } else {
            KeyState::UP
        }
    }
}

pub fn get_key_state(key: i32) -> KeyState {
    unsafe {
        let state = winapi::um::winuser::GetKeyState(key);
        KeyState::convert(state)
    }
}

fn main() {
    while true {
        println!("a - {:?}", get_key_state(0x41));
    }
}
