// Just while testing
#![allow(dead_code, unused_variables)]

use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOrAssign};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Bitboard(pub u64);

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

impl BitOrAssign<&Bitboard> for Bitboard {
    fn bitor_assign(&mut self, rhs: &Bitboard) {
        self.0 |= rhs.0
    }
}