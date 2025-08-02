use crate::bitboard::Bitboard;
use crate::role::{Role, get_role};
use crate::board::Board;
use crate::colour::{Colour, get_colour};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Move {
    pub role: Option<Role>,
    pub colour: Option<Colour>,
    pub from_square: Bitboard,
    pub to_square: Bitboard,
    pub en_passant_target: Bitboard,
    pub en_passant: bool,
    pub castle: bool,
    pub promotion: Option<Role>,
}

pub const EMPTY_MOVE: Move = Move {
    role: None,
    colour: None,
    from_square: Bitboard(0),
    to_square: Bitboard(0),
    en_passant_target: Bitboard(0),
    en_passant: false,
    castle: false,
    promotion: None,
};

impl Move {
    pub fn new(board: &Board, from_square: &Bitboard, to_square: &Bitboard, en_passant_target_square: &Bitboard, en_passant_bool: &bool, castle_bool: &bool, promotion_piece: Option<Role>) -> Move {
        Move {
            role: if let Some(get_role) = get_role(board, &from_square) {Some(get_role)} else {None},
            colour: if let Some(get_colour) = get_colour(board, from_square) {Some(get_colour)} else {None},
            from_square: *from_square,
            to_square: *to_square,
            en_passant_target: *en_passant_target_square,
            en_passant: *en_passant_bool,
            castle: *castle_bool,
            promotion: promotion_piece,
        }
    }
}