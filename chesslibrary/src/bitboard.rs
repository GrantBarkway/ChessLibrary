// Just while testing
#![allow(dead_code, unused_variables)]

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
            if (shifted.0 & self.0).count_ones() != 0 {
                bitboard_vector.push(shifted);
            }
        }
        return bitboard_vector;
    }
}