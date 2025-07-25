// Just while testing
#![allow(dead_code, unused_variables)]

use crate::bitboard::Bitboard;
use crate::role::{Role, get_role};
use crate::board::Board;
use crate::colour::{Colour, get_colour};
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

#[derive(Debug)]
pub struct Move {
    pub role: Option<Role>,
    pub colour: Option<Colour>,
    pub from_square: Bitboard,
    pub to_square: Bitboard,
    pub capture: Option<Role>,
    pub en_passant_target: Bitboard,
    pub castle: bool,
    pub promotion: Option<Role>,
}

pub const EMPTY_MOVE: Move = Move {
    role: None,
    colour: None,
    from_square: Bitboard(0),
    to_square: Bitboard(0),
    capture: None,
    en_passant_target: Bitboard(0),
    castle: false,
    promotion: None,
};

impl Move {
    pub fn new(board: &Board, from_square: &Bitboard, to_square: &Bitboard, en_passant_target_square: &Bitboard, castle_bool: &bool, promotion_piece: Option<Role>) -> Move {
        Move {
            role: if let Some(get_role) = get_role(board, &from_square) {Some(get_role)} else {None},
            colour: if let Some(get_colour) = get_colour(board, from_square) {Some(get_colour)} else {None},
            from_square: *from_square,
            to_square: *to_square,
            capture: None,
            en_passant_target: *en_passant_target_square,
            castle: *castle_bool,
            promotion: promotion_piece,
        }
    }
}