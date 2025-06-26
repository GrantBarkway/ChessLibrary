// Just while testing
#![allow(dead_code, unused_variables)]


#[derive(PartialEq)]
pub enum Colour {
    Black,
    White,
    None,
}

pub enum Role {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub struct ByRole<T> {
    pub pawn: T,
    pub knight: T,
    pub bishop: T,
    pub rook: T,
    pub queen: T,
    pub king: T,
}

pub struct ByColour<T> {
    pub white: T,
    pub black: T,
}