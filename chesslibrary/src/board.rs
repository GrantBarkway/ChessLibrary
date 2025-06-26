// Just while testing
#![allow(dead_code, unused_variables)]

use crate::mv::Move;
use crate::role::{Colour, ByRole, ByColour};

// Bitboard read in order (by which bit is set)
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
    turn: Colour,
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
            turn: Colour::White,
        }
    }
    
    pub fn make_move(&mut self, mv: Move) {
        self.clear_square(mv.from_square);
        self.occupied.0 ^= mv.target_square.0;
    }
    
    pub fn clear_square(&mut self, square: Bitboard) {
        
        self.occupied.0 ^= square.0;

        if (self.role.pawn.0 & square.0).count_ones() != 0 {
            self.role.pawn.0 ^= square.0;
        } else if (self.role.knight.0 & square.0).count_ones() != 0 {
            self.role.knight.0 ^= square.0;
        } else if (self.role.bishop.0 & square.0).count_ones() != 0 {
            self.role.bishop.0 ^= square.0;
        } else if (self.role.rook.0 & square.0).count_ones() != 0 {
            self.role.rook.0 ^= square.0;
        } else if (self.role.queen.0 & square.0).count_ones() != 0 {
            self.role.queen.0 ^= square.0;
        } else {
            self.role.king.0 ^= square.0;
        }
        
        if (self.colour.black.0 & square.0).count_ones() != 0 {
            self.colour.black.0 ^= square.0;
        } else if (self.colour.white.0 & square.0).count_ones() != 0 {
            self.colour.white.0 ^= square.0;
        }
    }

    pub fn get_colour(self, square: Bitboard) -> Colour {
        if (self.colour.black.0 & square.0).count_ones() != 0 {
            return Colour::Black;
        } else if (self.colour.white.0 & square.0).count_ones() != 0 {
            return Colour::White;
        } else {
            return Colour::None;
        }
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
