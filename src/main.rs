use std::{
    mem, thread,
    time::{Duration, Instant},
};
use winapi::{
    ctypes::c_int,
    shared::ntdef::SHORT,
    um::winuser::{INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP},
};

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum Direction {
    LEFT,
    RIGHT,
}

struct Timer {
    last: Instant,
    interval: Duration,
}

impl Timer {
    fn new(interval: Duration) -> Self {
        Self {
            last: Instant::now(),
            interval,
        }
    }

    fn ready(&mut self) -> bool {
        if self.last.elapsed() >= self.interval {
            self.last = Instant::now();
            true
        } else {
            false
        }
    }
}

pub fn get_key_state(key: i32) -> KeyState {
    unsafe {
        let state = winapi::um::winuser::GetKeyState(key);
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

pub fn switch() {
    send_input(0x11, KeyState::DOWN);
    thread::sleep(Duration::from_millis(10));
    send_input(0x11, KeyState::UP);
}

pub fn jump() {
    send_input(0x20, KeyState::DOWN);
    thread::sleep(Duration::from_millis(5));
    send_input(0x20, KeyState::UP);
}

pub fn update_console(direction: &Direction, in_game: bool) {
    print!("{}[2J", 27 as char);
    println!("Direction = {:?}", direction);
    println!("In_Game = {}", in_game);
}

fn main() {
    let mut direction = Direction::LEFT;
    let mut key_timer = Timer::new(Duration::from_millis(5));
    let mut in_game = false;
    update_console(&direction, in_game);

    loop {
        if key_timer.ready() {
            //Stop key - End
            if get_key_state(0x23) == KeyState::DOWN {
                break;
            }

            //Start Game
            if get_key_state(0x46) == KeyState::DOWN {
                direction = Direction::LEFT;
                in_game = true;
                update_console(&direction, in_game);
            }

            if get_key_state(0x41) == KeyState::DOWN {
                if direction == Direction::LEFT {
                    jump();
                } else {
                    switch();
                    direction = Direction::LEFT;
                    update_console(&direction, in_game);
                }
            }

            if get_key_state(0x44) == KeyState::DOWN {
                if direction == Direction::RIGHT {
                    jump();
                } else {
                    switch();
                    direction = Direction::RIGHT;
                    update_console(&direction, in_game);
                }
            }
        }
    }
}
