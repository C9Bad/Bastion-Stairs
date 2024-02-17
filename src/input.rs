use crate::types::KeyState;
use std::mem;
use winapi::{
    ctypes::c_int,
    um::winuser::{GetKeyState, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP},
};

pub fn get_key_state(key: i32) -> KeyState {
    unsafe {
        let state = GetKeyState(key);
        KeyState::convert(state)
    }
}

pub fn send_input(key: u16, keystate: KeyState) {
    unsafe {
        let mut ki: KEYBDINPUT = mem::zeroed();
        ki.wVk = key;

        match keystate {
            KeyState::DOWN => {
                ki.dwFlags = 0;
            }
            KeyState::UP => {
                ki.dwFlags = KEYEVENTF_KEYUP;
            }
        }

        let mut input: INPUT = mem::zeroed();
        input.type_ = INPUT_KEYBOARD;
        *input.u.ki_mut() = ki;
        winapi::um::winuser::SendInput(
            1,
            &mut input as *mut INPUT,
            mem::size_of::<INPUT>() as c_int,
        );
    }
}
