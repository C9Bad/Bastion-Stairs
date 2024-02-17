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

#[derive(Debug, PartialEq)]
pub enum Direction {
    LEFT,
    RIGHT,
}
