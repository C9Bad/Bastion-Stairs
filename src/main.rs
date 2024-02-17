mod input;
mod timer;
mod types;

use std::{thread, time::Duration};

use input::{get_key_state, send_input};
use timer::Timer;
use types::{Direction, KeyState};

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

            //Start Game F || G
            if get_key_state(0x46) == KeyState::DOWN || get_key_state(0x47) == KeyState::DOWN {
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
