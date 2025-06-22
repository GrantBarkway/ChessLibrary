use crate::board::Board;

pub struct Game {
    position: Vec<Board>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            position: Vec::new(),
        }
    }
}

