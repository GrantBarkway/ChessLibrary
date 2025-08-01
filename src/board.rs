use crate::mv::Move;
use crate::role::{Role, ByRole};
use crate::colour::{Colour, ByColour};
use crate::bitboard::{Bitboard, EMPTY_BITBOARD};
use crate::movegen::{get_black_attacks, get_legal_moves, get_white_attacks};
use crate::castle::{ByCastleSide};
use crate::square::Square;

// Order of board
// ....
//0b1000000000000000,0b100000000000000,0b10000000000000,0b1000000000000,0b100000000000,0b10000000000,0b1000000000,0b100000000
//0b10000000,0b1000000,0b100000,0b10000,0b1000,0b100,0b10,0b1

#[derive(Debug, Clone)]
pub struct Board {
    pub move_list: Vec<Move>,
    pub role: ByRole<Bitboard>,
    pub colour: ByColour<Bitboard>,
    pub occupied: Bitboard,
    pub turn: Colour,
    pub castling_rights: ByColour<ByCastleSide<bool>>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            move_list: Vec::new(),
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
            castling_rights: ByColour {
                black:
                    ByCastleSide { 
                        kingside: true,
                        queenside: true
                    },
                white:
                    ByCastleSide { 
                        kingside: true,
                        queenside: true,
                    }
            }
        }
    }
    
    // Makes move on the board
    pub fn play(&mut self, mv: Move) {
        let legal_moves = get_legal_moves(self);
        if legal_moves.contains(&mv) {
            
            self.castling_rights(mv);

            if mv.castle == true {
                self.play_castle(mv);
            } else {
                self.play_non_castle(mv);
            }
            
            self.swap_turn();
            
            self.move_list.push(mv);
        } else {
            panic!("Not a legal move!")
        }
    }

    pub fn play_unsafe(&mut self, mv: Move) {

        self.castling_rights(mv);

        self.clear_square(&mv.to_square);
        self.set_square(&mv.to_square, &mv.role, &mv.colour);
        self.clear_square(&mv.from_square);
        
        self.swap_turn();

        self.move_list.push(mv);
    }
    
    pub fn play_non_castle(&mut self, mv: Move) {
        self.clear_square(&mv.to_square);
        self.set_square(&mv.to_square, &mv.role, &mv.colour);
        self.clear_square(&mv.from_square);
    }
    
    pub fn play_castle(&mut self, mv: Move) {
        self.clear_square(&mv.from_square);
        self.set_square(&mv.to_square, &mv.role, &mv.colour);
        match mv.to_square {
            // White kingside
            Square::G1 => {
                self.clear_square(&Square::H1);
                self.set_square(&Square::F1, &Some(Role::Rook), &mv.colour);
            }
            // White queenside
            Square::C1 => {
                self.clear_square(&Square::A1);
                self.set_square(&Square::D1, &Some(Role::Rook), &mv.colour);
            }
            // Black kingside
            Square::G8 => {
                self.clear_square(&Square::H8);
                self.set_square(&Square::F8, &Some(Role::Rook), &mv.colour);
            }
            // Black queenside
            Square::C8 => {
                self.clear_square(&Square::A8);
                self.set_square(&Square::D8, &Some(Role::Rook), &mv.colour);
            }
            _ => (),
        }
    }
    
    // Swaps the board turn
    pub fn swap_turn(&mut self) {
        if self.turn == Colour::White {
            self.turn = Colour::Black;
        } else {
            self.turn = Colour::White;
        }
    }
    
    // Determines if the king is in check on a given board
    pub fn is_check(&self, colour_to_check: Colour) -> bool {
        let king_square: Bitboard;
        let attack_squares: Bitboard;
        match colour_to_check {
            Colour::White => (king_square, attack_squares) = (self.colour.white & self.role.king, get_black_attacks(&self)),
            Colour::Black => (king_square, attack_squares) = (self.colour.black & self.role.king, get_white_attacks(&self)),
        }

        if king_square & attack_squares == EMPTY_BITBOARD {
            return false;
        } else {
            return true;
        }
    }
    
    // Not very efficient, just need primitive for testing
    pub fn display_board(&self) {
        let mut set_bit: u64 = 0b1000000000000000000000000000000000000000000000000000000000000000;
        for _i in 0..8 {
            let mut rank = String::from("");
            for _i in 0..8 {
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