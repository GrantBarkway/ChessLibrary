// Just while testing
#![allow(dead_code, unused_variables)]

#[derive(Copy, Clone, Debug)]
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
}