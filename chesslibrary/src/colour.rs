// Just while testing
#![allow(dead_code, unused_variables)]

pub enum Colour {
    Black,
    White,
}

pub struct ByColour<T> {
    pub white: T,
    pub black: T,
}