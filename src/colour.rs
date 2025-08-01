use crate::bitboard::{Bitboard, EMPTY_BITBOARD};
use crate::board::Board;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Colour {
    Black,
    White,
}

#[derive(Debug, Clone, Copy)]
pub struct ByColour<T> {
    pub white: T,
    pub black: T,
}
 
 // Gets the colour of piece at a square
pub fn get_colour(board: &Board, square: &Bitboard) -> Option<Colour> {
    if (board.colour.black & square) != EMPTY_BITBOARD {
        return Some(Colour::Black);
    } else if (board.colour.white & square) != EMPTY_BITBOARD {
        return Some(Colour::White);
    } else {
        return None;
    }
}