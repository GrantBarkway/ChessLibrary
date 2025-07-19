// Just while testing
#![allow(dead_code, unused_variables)]

use crate::mv::Move;
use crate::role::{ByRole};
use crate::colour::{Colour, ByColour};
use crate::bitboard::Bitboard;

#[derive(Debug)]
// Order of board
// ....
//0b1000000000000000,0b100000000000000,0b10000000000000,0b1000000000000,0b100000000000,0b10000000000,0b1000000000,0b100000000
//0b10000000,0b1000000,0b100000,0b10000,0b1000,0b100,0b10,0b1

pub struct Board {
    pub role: ByRole<Bitboard>,
    pub colour: ByColour<Bitboard>,
    pub occupied: Bitboard,
    pub turn: Colour,
}

impl Board {
    pub fn new() -> Board {
        Board {
            role: ByRole {
                pawn: Bitboard(0x00ff_0000_0000_ff00),
                knight: Bitboard(0x4200_0000_0000_0042),
                bishop: Bitboard(0x2400_0000_0000_0024),
                rook: Bitboard(0x8100_0000_0000_0081),
                queen: Bitboard(0x1000_0000_0000_0010),
                king: Bitboard(0x0800_0000_0000_0008),
            },
            colour: ByColour {
                black: Bitboard(0xffff_0000_0000_0000),
                white: Bitboard(0xffff),
            },
            occupied: Bitboard(0xffff_0000_0000_ffff),
            turn: Colour::White,
        }
    }
    
    // Makes move on the board
    pub fn make_move(&mut self, mv: Move) {
        self.clear_square(&mv.to_square);
        self.set_square(&mv.to_square, &mv.role, &mv.colour);
        self.occupied |= &mv.to_square;
        self.clear_square(&mv.from_square);

        if self.turn == Colour::White {
            self.turn = Colour::Black;
        } else {
            self.turn = Colour::White;
        }
    }
    
    // Not very efficient, just need primitive for testing
    pub fn display_board(&self) {
        let mut set_bit: u64 = 0b1000000000000000000000000000000000000000000000000000000000000000;
        for i in 0..8 {
            let mut rank = String::from("");
            for i in 0..8 {
                if (self.occupied & set_bit).count_ones() != 0 {
                    if (self.colour.white & set_bit).count_ones() != 0 {
                        if (self.role.king & set_bit).count_ones() != 0 {
                            rank.push('k');
                        } else if (self.role.queen & set_bit).count_ones() != 0 {
                            rank.push('q');
                        } else if (self.role.rook & set_bit).count_ones() != 0 {
                            rank.push('r');
                        } else if (self.role.bishop & set_bit).count_ones() != 0 {
                            rank.push('b');
                        } else if (self.role.knight & set_bit).count_ones() != 0 {
                            rank.push('n');
                        } else {
                            rank.push('p');
                        }
                    } else {
                        if (self.role.king & set_bit).count_ones() != 0 {
                            rank.push('K');
                        } else if (self.role.queen & set_bit).count_ones() != 0 {
                            rank.push('Q');
                        } else if (self.role.rook & set_bit).count_ones() != 0 {
                            rank.push('R');
                        } else if (self.role.bishop & set_bit).count_ones() != 0 {
                            rank.push('B');
                        } else if (self.role.knight & set_bit).count_ones() != 0 {
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
