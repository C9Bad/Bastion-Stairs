use std::thread::sleep;
use std::time::{Duration, Instant};
use winapi::shared::ntdef::SHORT;

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

#[derive(Debug)]
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

fn main() {
    let mut direction = Direction::LEFT;
    let mut key_timer = Timer::new(Duration::from_millis(25));
    let mut console_timer = Timer::new(Duration::from_millis(500));

    loop {
        if key_timer.ready() {
            println!("{}", false);
        }

        if console_timer.ready() {
            println!("{}", true);
        }
    }
}
