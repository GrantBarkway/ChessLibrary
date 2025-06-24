// Just while testing
#![allow(dead_code, unused_variables)]

use crate::mv::Move;

// Bitboard read in order
// {1,2,3,4,5,6,7,8}
// {9,10,11,12,13,14,15,16}
// {17,18,19,20,21,22,23,24}
// ...
// ...
// ...
// ...
// {57,58,59,60,61,62,63,64}

#[derive(Debug)]
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

    pub fn make_move(&mut self, mv: Move) -> Board {
        return Board::new();
    }
    
    // Not very efficient, just need primitive for testing
    pub fn display_board(&self) {
        let mut set_bit: u64 = 0b1000000000000000000000000000000000000000000000000000000000000000;
        for i in 0..8 {
            let mut rank = String::from("");
            for i in 0..8 {
                if (self.occupied.0 & set_bit).count_ones() != 0 {
                    if (self.colour.white.0 & set_bit).count_ones() != 0 {
                        if (self.role.king.0 & set_bit).count_ones() != 0 {
                            rank.push('k');
                        } else if (self.role.queen.0 & set_bit).count_ones() != 0 {
                            rank.push('q');
                        } else if (self.role.rook.0 & set_bit).count_ones() != 0 {
                            rank.push('r');
                        } else if (self.role.bishop.0 & set_bit).count_ones() != 0 {
                            rank.push('b');
                        } else if (self.role.knight.0 & set_bit).count_ones() != 0 {
                            rank.push('n');
                        } else {
                            rank.push('p');
                        }
                    } else {
                        if (self.role.king.0 & set_bit).count_ones() != 0 {
                            rank.push('K');
                        } else if (self.role.queen.0 & set_bit).count_ones() != 0 {
                            rank.push('Q');
                        } else if (self.role.rook.0 & set_bit).count_ones() != 0 {
                            rank.push('R');
                        } else if (self.role.bishop.0 & set_bit).count_ones() != 0 {
                            rank.push('B');
                        } else if (self.role.knight.0 & set_bit).count_ones() != 0 {
                            rank.push('N');
                        } else {
                            rank.push('P');
                        }
                    }
                } else {
                    rank.push('.');
                }
                set_bit = set_bit >> 1;
            }
            println!("{:?}", rank);
        }
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

enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}
