// Just while testing
#![allow(dead_code, unused_variables)]

use crate::board::{Board};
use crate::role::{Role, get_role};
use crate::colour::{Colour, get_colour};
use crate::bitboard::{Bitboard};

// A8, B8, C8, D8, E8, F8, G8, H8,
// ...
// ... 
// ...
// ...
// ...
// ...
// A1, B1, C1, D1, E1, F1, G1, H1

// Bitboard read in order (by which bit is set)
// {1,2,3,4,5,6,7,8}
// {9,10,11,12,13,14,15,16}
// {17,18,19,20,21,22,23,24}
// ...
// ...
// ...
// ...
// {57,58,59,60,61,62,63,64}

pub struct Square;

impl Square {
    pub const A8: Bitboard = Bitboard(0b1000000000000000000000000000000000000000000000000000000000000000);
    pub const B8: Bitboard = Bitboard(0b100000000000000000000000000000000000000000000000000000000000000);
    pub const C8: Bitboard = Bitboard(0b10000000000000000000000000000000000000000000000000000000000000);
    pub const D8: Bitboard = Bitboard(0b1000000000000000000000000000000000000000000000000000000000000);
    pub const E8: Bitboard = Bitboard(0b100000000000000000000000000000000000000000000000000000000000);
    pub const F8: Bitboard = Bitboard(0b10000000000000000000000000000000000000000000000000000000000);
    pub const G8: Bitboard = Bitboard(0b1000000000000000000000000000000000000000000000000000000000);
    pub const H8: Bitboard = Bitboard(0b100000000000000000000000000000000000000000000000000000000);
    pub const A7: Bitboard = Bitboard(0b10000000000000000000000000000000000000000000000000000000);
    pub const B7: Bitboard = Bitboard(0b1000000000000000000000000000000000000000000000000000000);
    pub const C7: Bitboard = Bitboard(0b100000000000000000000000000000000000000000000000000000);
    pub const D7: Bitboard = Bitboard(0b10000000000000000000000000000000000000000000000000000);
    pub const E7: Bitboard = Bitboard(0b1000000000000000000000000000000000000000000000000000);
    pub const F7: Bitboard = Bitboard(0b100000000000000000000000000000000000000000000000000);
    pub const G7: Bitboard = Bitboard(0b10000000000000000000000000000000000000000000000000);
    pub const H7: Bitboard = Bitboard(0b1000000000000000000000000000000000000000000000000);
    pub const A6: Bitboard = Bitboard(0b100000000000000000000000000000000000000000000000);
    pub const B6: Bitboard = Bitboard(0b10000000000000000000000000000000000000000000000);
    pub const C6: Bitboard = Bitboard(0b1000000000000000000000000000000000000000000000);
    pub const D6: Bitboard = Bitboard(0b100000000000000000000000000000000000000000000);
    pub const E6: Bitboard = Bitboard(0b10000000000000000000000000000000000000000000);
    pub const F6: Bitboard = Bitboard(0b1000000000000000000000000000000000000000000);
    pub const G6: Bitboard = Bitboard(0b100000000000000000000000000000000000000000);
    pub const H6: Bitboard = Bitboard(0b10000000000000000000000000000000000000000);
    pub const A5: Bitboard = Bitboard(0b1000000000000000000000000000000000000000);
    pub const B5: Bitboard = Bitboard(0b100000000000000000000000000000000000000);
    pub const C5: Bitboard = Bitboard(0b10000000000000000000000000000000000000);
    pub const D5: Bitboard = Bitboard(0b1000000000000000000000000000000000000);
    pub const E5: Bitboard = Bitboard(0b100000000000000000000000000000000000);
    pub const F5: Bitboard = Bitboard(0b10000000000000000000000000000000000);
    pub const G5: Bitboard = Bitboard(0b1000000000000000000000000000000000);
    pub const H5: Bitboard = Bitboard(0b100000000000000000000000000000000);
    pub const A4: Bitboard = Bitboard(0b10000000000000000000000000000000);
    pub const B4: Bitboard = Bitboard(0b1000000000000000000000000000000);
    pub const C4: Bitboard = Bitboard(0b100000000000000000000000000000);
    pub const D4: Bitboard = Bitboard(0b10000000000000000000000000000);
    pub const E4: Bitboard = Bitboard(0b1000000000000000000000000000);
    pub const F4: Bitboard = Bitboard(0b100000000000000000000000000);
    pub const G4: Bitboard = Bitboard(0b10000000000000000000000000);
    pub const H4: Bitboard = Bitboard(0b1000000000000000000000000);
    pub const A3: Bitboard = Bitboard(0b100000000000000000000000);
    pub const B3: Bitboard = Bitboard(0b10000000000000000000000);
    pub const C3: Bitboard = Bitboard(0b1000000000000000000000);
    pub const D3: Bitboard = Bitboard(0b100000000000000000000);
    pub const E3: Bitboard = Bitboard(0b10000000000000000000);
    pub const F3: Bitboard = Bitboard(0b1000000000000000000);
    pub const G3: Bitboard = Bitboard(0b100000000000000000);
    pub const H3: Bitboard = Bitboard(0b10000000000000000);
    pub const A2: Bitboard = Bitboard(0b1000000000000000);
    pub const B2: Bitboard = Bitboard(0b100000000000000);
    pub const C2: Bitboard = Bitboard(0b10000000000000);
    pub const D2: Bitboard = Bitboard(0b1000000000000);
    pub const E2: Bitboard = Bitboard(0b100000000000);
    pub const F2: Bitboard = Bitboard(0b10000000000);
    pub const G2: Bitboard = Bitboard(0b1000000000);
    pub const H2: Bitboard = Bitboard(0b100000000);
    pub const A1: Bitboard = Bitboard(0b10000000);
    pub const B1: Bitboard = Bitboard(0b1000000);
    pub const C1: Bitboard = Bitboard(0b100000);
    pub const D1: Bitboard = Bitboard(0b10000);
    pub const E1: Bitboard = Bitboard(0b1000);
    pub const F1: Bitboard = Bitboard(0b100);
    pub const G1: Bitboard = Bitboard(0b10);
    pub const H1: Bitboard = Bitboard(0b1);
}

pub const ALL_SQUARES: [Bitboard; 64] = 
[Square::H1,Square::G1,Square::F1,Square::E1,Square::D1,Square::C1,Square::B1,Square::A1,
Square::H2,Square::G2,Square::F2,Square::E2,Square::D2,Square::C2,Square::B2,Square::A2,
Square::H3,Square::G3,Square::F3,Square::E3,Square::D3,Square::C3,Square::B3,Square::A3,
Square::H4,Square::G4,Square::F4,Square::E4,Square::D4,Square::C4,Square::B4,Square::A4,
Square::H5,Square::G5,Square::F5,Square::E5,Square::D5,Square::C5,Square::B5,Square::A5,
Square::H6,Square::G6,Square::F6,Square::E6,Square::D6,Square::C6,Square::B6,Square::A6,
Square::H7,Square::G7,Square::F7,Square::E7,Square::D7,Square::C7,Square::B7,Square::A7,
Square::H8,Square::G8,Square::F8,Square::E8,Square::D8,Square::C8,Square::B8,Square::A8];

pub const FILE_A: Bitboard = Bitboard(0b1000000010000000100000001000000010000000100000001000000010000000);
pub const FILE_B: Bitboard = Bitboard(0b0100000001000000010000000100000001000000010000000100000001000000);
pub const FILE_C: Bitboard = Bitboard(0b0010000000100000001000000010000000100000001000000010000000100000);
pub const FILE_D: Bitboard = Bitboard(0b0001000000010000000100000001000000010000000100000001000000010000);
pub const FILE_E: Bitboard = Bitboard(0b0000100000001000000010000000100000001000000010000000100000001000);
pub const FILE_F: Bitboard = Bitboard(0b0000010000000100000001000000010000000100000001000000010000000100);
pub const FILE_G: Bitboard = Bitboard(0b0000001000000010000000100000001000000010000000100000001000000010);
pub const FILE_H: Bitboard = Bitboard(0b0000000100000001000000010000000100000001000000010000000100000001);

pub const ALL_FILES: [Bitboard; 8] = [FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H];

pub const FIRST_RANK: Bitboard = Bitboard(0b11111111);
pub const SECOND_RANK: Bitboard = Bitboard(0b1111111100000000);
pub const THIRD_RANK: Bitboard = Bitboard(0b111111110000000000000000);
pub const FOURTH_RANK: Bitboard = Bitboard(0b11111111000000000000000000000000);
pub const FIFTH_RANK: Bitboard = Bitboard(0b1111111100000000000000000000000000000000);
pub const SIXTH_RANK: Bitboard = Bitboard(0b111111110000000000000000000000000000000000000000);
pub const SEVENTH_RANK: Bitboard = Bitboard(0b11111111000000000000000000000000000000000000000000000000);
pub const EIGHTH_RANK: Bitboard = Bitboard(0b1111111100000000000000000000000000000000000000000000000000000000);

pub const ALL_RANKS: [Bitboard; 8] = [FIRST_RANK, SECOND_RANK, THIRD_RANK, FOURTH_RANK, FIFTH_RANK, SIXTH_RANK, SEVENTH_RANK, EIGHTH_RANK];

impl Board {
    // Clears a specific square
    pub fn clear_square(&mut self, square: &Bitboard) {
        
        let square_bitboard_clear_bit = !square.0;
        
        self.occupied &= square_bitboard_clear_bit;
        
        if let Some(square_role) = get_role(self, &square) {
            match square_role {
                Role::Pawn => self.role.pawn &= square_bitboard_clear_bit,
                Role::Knight => self.role.knight &= square_bitboard_clear_bit,
                Role::Bishop => self.role.bishop &= square_bitboard_clear_bit,
                Role::Rook => self.role.rook &= square_bitboard_clear_bit,
                Role::Queen => self.role.queen &= square_bitboard_clear_bit,
                Role::King => self.role.king &= square_bitboard_clear_bit
            };
        }
        
        if let Some(role_colour) = get_colour(&self, &square) {
            match role_colour {
                Colour::White => self.colour.white &= square_bitboard_clear_bit,
                Colour::Black => self.colour.black &= square_bitboard_clear_bit,
            };
        }
    }

    pub fn set_square(&mut self, square: &Bitboard, role: &Option<Role>, colour: &Option<Colour>) {
        if let Some(role) = role {
            match role {
                Role::Pawn => self.role.pawn |= square,
                Role::Knight => self.role.knight |= square,
                Role::Bishop => self.role.bishop |= square,
                Role::Rook => self.role.rook |= square,
                Role::Queen => self.role.queen |= square,
                Role::King => self.role.king |= square
            };
        }
        
        self.occupied.0 |= square.0;
        
        if let Some(colour) = colour {
            match colour {
                Colour::White => self.colour.white |= square,
                Colour::Black => self.colour.black |= square,
            }
        }
    }
}