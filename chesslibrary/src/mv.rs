// Just while testing
#![allow(dead_code, unused_variables)]

use crate::bitboard::Bitboard;
use crate::role::{Role, get_role};
use crate::board::Board;

#[derive(Debug)]

pub struct Move {
    pub role: Option<Role>,
    pub to_square: Bitboard,
    pub from_square: Bitboard,
    pub capture: Option<Role>,
}

impl Move {
    pub fn new(board: &Board, from_square: &Bitboard, to_square: &Bitboard) -> Move {
        Move {
            role: if let Some(get_role) = get_role(board, &from_square) {Some(get_role)} else {None},
            to_square: *to_square,
            from_square: *from_square,
            capture: None,
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