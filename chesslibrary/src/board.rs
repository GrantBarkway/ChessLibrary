// Just while testing
#![allow(dead_code, unused_variables)]

use crate::mv::Move;

pub struct Bitboard(pub u64);

pub struct Board {
    role: ByRole<Bitboard>,
    colour: ByColour<Bitboard>,
    occupied: Bitboard,
}

// inspired by Shakmaty
impl Board {
    pub fn new() -> Board {
        Board {
            role: ByRole {
                pawn: Bitboard(0x00ff_0000_0000_ff00),
                knight: Bitboard(0x4200_0000_0000_0042),
                bishop: Bitboard(0x2400_0000_0000_0024),
                rook: Bitboard(0x8100_0000_0000_0081),
                queen: Bitboard(0x0800_0000_0000_0008),
                king: Bitboard(0x1000_0000_0000_0010),
            },
            colour: ByColour {
                black: Bitboard(0xffff_0000_0000_0000),
                white: Bitboard(0xffff),
            },
            occupied: Bitboard(0xffff_0000_0000_ffff),
        }
    }

    fn make_move(&mut self, mv: Move) -> Board {
        return Board::new();
    }
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