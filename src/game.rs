// Just while testing
#![allow(dead_code, unused_variables)]

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
    
    fn play(&mut self, board: Board) {
        self.position.push(board);
    }
}

