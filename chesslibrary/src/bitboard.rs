// Just while testing
#![allow(dead_code, unused_variables)]

use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOrAssign, Not, BitOr};
use crate::square::{EIGHTH_RANK, FIFTH_RANK, FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H, FIRST_RANK, FOURTH_RANK, SECOND_RANK, SEVENTH_RANK, SIXTH_RANK, THIRD_RANK};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Bitboard(pub u64);

pub const EMPTY_BITBOARD: Bitboard = Bitboard(0);

impl Bitboard {
    // Credit to Shakmaty for this function code
    pub const fn shift(&self, offset: i32) -> Bitboard {
        Bitboard(if offset > 63 {
            0
        } else if offset >= 0 {
            self.0 << offset
        } else if offset >= -63 {
            self.0 >> -offset
        } else {
            0
        })
    }
    
    // Splits a bitboard with multiple 1s into a vector of component bitboards containing only one 1 in it
    pub fn get_component_bitboards(&self) -> Vec<Bitboard> {
        let mut bitboard_vector = Vec::new();
        let bit_mask = Bitboard(1);
        for i in 0..64 {
            let shifted = bit_mask.shift(i);
            if (shifted & self).count_ones() != 0 {
                bitboard_vector.push(shifted);
            }
        }
        return bitboard_vector;
    }

    pub fn count_ones(&self) -> u32 {
        return self.0.count_ones();
    }
}

// Not efficient, just for easier debugging
pub fn bitboard_to_string_move(sq: Bitboard) -> String {
    let mut move_as_string = String::new();
    
    if (sq & FILE_A).count_ones() != 0 {
        move_as_string.push_str("A");
    } else if (sq & FILE_B).count_ones() != 0 {
        move_as_string.push_str("B");
    } else if (sq & FILE_C).count_ones() != 0 {
        move_as_string.push_str("C");
    } else if (sq & FILE_D).count_ones() != 0 {
        move_as_string.push_str("D");
    } else if (sq & FILE_E).count_ones() != 0 {
        move_as_string.push_str("E");
    } else if (sq & FILE_F).count_ones() != 0 {
        move_as_string.push_str("F");
    } else if (sq & FILE_G).count_ones() != 0 {
        move_as_string.push_str("G");
    } else if (sq & FILE_H).count_ones() != 0 {
        move_as_string.push_str("H");
    }
    
    if (sq & FIRST_RANK).count_ones() != 0 {
        move_as_string.push_str("1");
    } else if (sq & SECOND_RANK).count_ones() != 0 {
        move_as_string.push_str("2");
    } else if (sq & THIRD_RANK).count_ones() != 0 {
        move_as_string.push_str("3");
    } else if (sq & FOURTH_RANK).count_ones() != 0 {
        move_as_string.push_str("4");
    } else if (sq & FIFTH_RANK).count_ones() != 0 {
        move_as_string.push_str("5");
    } else if (sq & SIXTH_RANK).count_ones() != 0 {
        move_as_string.push_str("6");
    } else if (sq & SEVENTH_RANK).count_ones() != 0 {
        move_as_string.push_str("7");
    } else if (sq & EIGHTH_RANK).count_ones() != 0 {
        move_as_string.push_str("8");
    }

    return move_as_string;
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl BitAnd for Bitboard {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAnd<u64> for Bitboard {
    type Output = Self;
    fn bitand(self, rhs: u64) -> Self::Output {
        Self(self.0 & rhs)
    }
}

impl BitAnd<&Bitboard> for Bitboard {
    type Output = u64;
    fn bitand(self, rhs: &Bitboard) -> Self::Output {
        self.0 & rhs.0
    }
}

impl BitAndAssign<u64> for Bitboard {
    fn bitand_assign(&mut self, rhs: u64) {
        *self = Self(self.0 & rhs)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Bitboard) {
        *self = Self(self.0 & rhs.0)
    }
}

impl BitOr for Bitboard {
    type Output = Bitboard;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign<&Bitboard> for Bitboard {
    fn bitor_assign(&mut self, rhs: &Bitboard) {
        self.0 |= rhs.0
    }
}

impl BitOrAssign<Bitboard> for Bitboard {
    fn bitor_assign(&mut self, rhs: Bitboard) {
        self.0 |= rhs.0
    }
}

impl Not for Bitboard {
    type Output = Bitboard;
    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}