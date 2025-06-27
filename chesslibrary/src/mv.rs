// Just while testing
#![allow(dead_code, unused_variables)]

use crate::board::{Bitboard};

pub struct Move {
    pub target_square: Bitboard,
    pub from_square: Bitboard,
}

impl Move {
    pub fn new() -> Move {
        Move {
            target_square: Bitboard(0b10000000000000000),
            from_square: Bitboard(0b10),
        }
    }
}

enum Rank {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth
}

enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H
}