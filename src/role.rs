use crate::board::Board;
use crate::bitboard::{Bitboard, EMPTY_BITBOARD};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Role {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

#[derive(Debug, Clone, Copy)]
pub struct ByRole<T> {
    pub pawn: T,
    pub knight: T,
    pub bishop: T,
    pub rook: T,
    pub queen: T,
    pub king: T,
}

// Gets the role at a square. Can only be called on occupied squares
pub fn get_role(board: &Board, square: &Bitboard) -> Option<Role> {
    if (board.role.pawn & square) != EMPTY_BITBOARD {
        return Some(Role::Pawn);
    } else if (board.role.knight & square) != EMPTY_BITBOARD {
        return Some(Role::Knight);
    } else if (board.role.bishop & square) != EMPTY_BITBOARD {
        return Some(Role::Bishop);
    } else if (board.role.rook & square) != EMPTY_BITBOARD {
        return Some(Role::Rook);
    } else if (board.role.queen & square) != EMPTY_BITBOARD {
        return Some(Role::Queen);
    } else if (board.role.king & square) != EMPTY_BITBOARD {
        return Some(Role::King);
    } else {
        return None;
    }
}