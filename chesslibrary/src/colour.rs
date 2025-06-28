// Just while testing
#![allow(dead_code, unused_variables)]

use crate::bitboard::Bitboard;
use crate::board::Board;

#[derive(Debug)]
pub enum Colour {
    Black,
    White,
}

#[derive(Debug)]
pub struct ByColour<T> {
    pub white: T,
    pub black: T,
}

 // Gets the colour at a square
pub fn get_colour(board: &Board, square: &Bitboard) -> Option<Colour> {
    if (board.colour.black.0 & square.0).count_ones() != 0 {
        return Some(Colour::Black);
    } else if (board.colour.white.0 & square.0).count_ones() != 0 {
        return Some(Colour::White);
    } else {
        return None;
    }
}