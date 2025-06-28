// Just while testing
#![allow(dead_code, unused_variables)]

use crate::board::Board;
use crate::bitboard::Bitboard;

#[derive(Debug, PartialEq)]
pub enum Role {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

pub struct ByRole<T> {
    pub pawn: T,
    pub knight: T,
    pub bishop: T,
    pub rook: T,
    pub queen: T,
    pub king: T,
}

// Gets the role at a square
pub fn get_role(board: &Board, square: &Bitboard) -> Option<Role> {
    if (board.role.pawn.0 & square.0).count_ones() != 0 {
        return Some(Role::Pawn);
    } else if (board.role.knight.0 & square.0).count_ones() != 0 {
        return Some(Role::Knight);
    } else if (board.role.bishop.0 & square.0).count_ones() != 0 {
        return Some(Role::Bishop);
    } else if (board.role.rook.0 & square.0).count_ones() != 0 {
        return Some(Role::Rook);
    } else if (board.role.queen.0 & square.0).count_ones() != 0 {
        return Some(Role::Queen);
    } else if (board.role.king.0 & square.0).count_ones() != 0 {
        return Some(Role::King);
    } else {
        return None;
    }
}