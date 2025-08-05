use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not, Shl, Shr};
use crate::square::{EIGHTH_RANK, FIFTH_RANK, FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H, FIRST_RANK, FOURTH_RANK, SECOND_RANK, SEVENTH_RANK, SIXTH_RANK, THIRD_RANK};
use arrayvec::ArrayVec;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bitboard(pub u64);

pub const EMPTY_BITBOARD: Bitboard = Bitboard(0);

impl Bitboard {
    // Splits a bitboard with multiple 1s into a vector of component bitboards containing only one 1 in it
    pub fn get_component_bitboards(&self) -> ArrayVec::<Bitboard, 64> {
        let mut bitboard_vector = ArrayVec::<Bitboard, 64>::new();
        let mut bitboard_copy = self.clone().0 as i64;
        let mut lowest_set_bit: i64;
        while bitboard_copy != 0 {
            lowest_set_bit = bitboard_copy & -bitboard_copy;
            bitboard_vector.push(Bitboard(lowest_set_bit as u64));
            bitboard_copy = bitboard_copy - lowest_set_bit;
        }
        return bitboard_vector;
    }
    
    pub fn count_ones(&self) -> u32 {
        return self.0.count_ones();
    }
    
    pub const fn trailing_zeros(&self) -> i32 {
        return self.0.trailing_zeros() as i32;
    }
    
    pub fn get_file(self) -> Bitboard {
        if self & FILE_A != EMPTY_BITBOARD {
            return FILE_A
        } else if self & FILE_B != EMPTY_BITBOARD {
            return FILE_B
        } else if self & FILE_C != EMPTY_BITBOARD {
            return FILE_C
        } else if self & FILE_D != EMPTY_BITBOARD {
            return FILE_D
        } else if self & FILE_E != EMPTY_BITBOARD {
            return FILE_E
        } else if self & FILE_F != EMPTY_BITBOARD {
            return FILE_F
        } else if self & FILE_G != EMPTY_BITBOARD {
            return FILE_G
        } else if self & FILE_H != EMPTY_BITBOARD {
            return FILE_H
        } else {
            return EMPTY_BITBOARD;
        }
    }
    
    pub fn get_rank(self) -> Bitboard {
        if self & FIRST_RANK != EMPTY_BITBOARD {
            return FIRST_RANK
        } else if self & SECOND_RANK != EMPTY_BITBOARD {
            return SECOND_RANK
        } else if self & THIRD_RANK != EMPTY_BITBOARD {
            return THIRD_RANK
        } else if self & FOURTH_RANK != EMPTY_BITBOARD {
            return FOURTH_RANK
        } else if self & FIFTH_RANK != EMPTY_BITBOARD {
            return FIFTH_RANK
        } else if self & SIXTH_RANK != EMPTY_BITBOARD {
            return SIXTH_RANK
        } else if self & SEVENTH_RANK != EMPTY_BITBOARD {
            return SEVENTH_RANK
        } else if self & EIGHTH_RANK != EMPTY_BITBOARD {
            return EIGHTH_RANK
        } else {
            return EMPTY_BITBOARD
        }
    }
}

// Not efficient, just for easier debugging
pub fn bitboard_to_string_move(sq: Bitboard) -> String {
    let mut move_as_string = String::new();
    
    if (&sq & FILE_A).count_ones() != 0 {
        move_as_string.push_str("A");
    } else if (&sq & FILE_B).count_ones() != 0 {
        move_as_string.push_str("B");
    } else if (&sq & FILE_C).count_ones() != 0 {
        move_as_string.push_str("C");
    } else if (&sq & FILE_D).count_ones() != 0 {
        move_as_string.push_str("D");
    } else if (&sq & FILE_E).count_ones() != 0 {
        move_as_string.push_str("E");
    } else if (&sq & FILE_F).count_ones() != 0 {
        move_as_string.push_str("F");
    } else if (&sq & FILE_G).count_ones() != 0 {
        move_as_string.push_str("G");
    } else if (&sq & FILE_H).count_ones() != 0 {
        move_as_string.push_str("H");
    }
    
    if (&sq & FIRST_RANK).count_ones() != 0 {
        move_as_string.push_str("1");
    } else if (&sq & SECOND_RANK).count_ones() != 0 {
        move_as_string.push_str("2");
    } else if (&sq & THIRD_RANK).count_ones() != 0 {
        move_as_string.push_str("3");
    } else if (&sq & FOURTH_RANK).count_ones() != 0 {
        move_as_string.push_str("4");
    } else if (&sq & FIFTH_RANK).count_ones() != 0 {
        move_as_string.push_str("5");
    } else if (&sq & SIXTH_RANK).count_ones() != 0 {
        move_as_string.push_str("6");
    } else if (&sq & SEVENTH_RANK).count_ones() != 0 {
        move_as_string.push_str("7");
    } else if (&sq & EIGHTH_RANK).count_ones() != 0 {
        move_as_string.push_str("8");
    }

    return move_as_string;
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Eq for Bitboard {}

impl Shl<i32> for Bitboard {
    type Output = Self;
    
    fn shl(self, rhs: i32) -> Self::Output {
        let Self(lhs) = self;
        Self(lhs << rhs)
    }
}

impl Shr<i32> for Bitboard {
    type Output = Self;
        
    fn shr(self, rhs: i32) -> Self::Output {
        let Self(lhs) = self;
        Self(lhs >> rhs)
    }
}

impl BitAnd for &Bitboard {
    type Output = Bitboard;
    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitAnd<Bitboard> for &Bitboard {
    type Output = Bitboard;
    fn bitand(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 & rhs.0)
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
    type Output = Self;
    fn bitand(self, rhs: &Bitboard) -> Self::Output {
        Self(self.0 & rhs.0)
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