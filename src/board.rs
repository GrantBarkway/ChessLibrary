use crate::mv::Move;
use crate::role::{Role, ByRole};
use crate::colour::{Colour, ByColour};
use crate::bitboard::{Bitboard, EMPTY_BITBOARD};
use crate::movegen::{get_bishop_attacks, get_black_pawn_attacks, get_knight_attacks, get_legal_moves, get_queen_attacks, get_rook_attacks, get_white_pawn_attacks};
use crate::castle::{ByCastleSide};
use crate::square::Square;


// Order of board
// ....
//0b1000000000000000,0b100000000000000,0b10000000000000,0b1000000000000,0b100000000000,0b10000000000,0b1000000000,0b100000000
//0b10000000,0b1000000,0b100000,0b10000,0b1000,0b100,0b10,0b1

#[derive(Debug, Clone)]
pub struct Board {
    pub role: ByRole<Bitboard>,
    pub colour: ByColour<Bitboard>,
    pub occupied: Bitboard,
    pub turn: Colour,
    pub castling_rights: ByColour<ByCastleSide<bool>>,
    pub last_move: Option<Move>,
}

impl Board {
    pub fn starting_position() -> Board {
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
            },
            last_move: None,
        }
    }

    pub fn empty_board() -> Board {
        Board {
            role: ByRole {
                pawn: Bitboard(0),
                knight: Bitboard(0),
                bishop: Bitboard(0),
                rook: Bitboard(0),
                queen: Bitboard(0),
                king: Bitboard(0),
            },
            colour: ByColour {
                black: Bitboard(0),
                white: Bitboard(0),
            },
            occupied: Bitboard(0),
            turn: Colour::White,
            castling_rights: ByColour {
                black:
                    ByCastleSide { 
                        kingside: false,
                        queenside: false
                    },
                white:
                    ByCastleSide { 
                        kingside: false,
                        queenside: false,
                    }
            },
            last_move: None,
        }
    }
    
    // Makes move on the board
    pub fn play(&mut self, mv: Move) {
        let legal_moves = get_legal_moves(self);
        if legal_moves.contains(&mv) {
            
            self.castling_rights(mv);

            if mv.castle == true {
                self.play_castle(mv);
            } else if mv.en_passant == true {
                self.play_en_passant(mv);
            } else if mv.promotion != None{
                self.play_promotion(mv);
            } else {
                self.play_normal(mv);
            }
            
            self.swap_turn();
            
            self.last_move = Some(mv);
        } else {
            panic!("Not a legal move!")
        }
    }

    pub fn play_unsafe(&mut self, mv: Move) {

        self.castling_rights(mv);

        if mv.castle == true {
            self.play_castle(mv);
        } else if mv.en_passant == true {
            self.play_en_passant(mv);
        } else if mv.promotion != None {
            self.play_promotion(mv);
        } else {
            self.play_normal(mv);
        }
        
        self.swap_turn();

        self.last_move = Some(mv);
    }
    
    pub fn play_normal(&mut self, mv: Move) {
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

    pub fn play_en_passant(&mut self, mv: Move) {
        let opponent_pawn_square: Bitboard;
        self.clear_square(&mv.from_square);
        self.clear_square(&mv.to_square);
        self.set_square(&mv.to_square, &mv.role, &mv.colour);
        match self.turn {
            Colour::White =>  {
                opponent_pawn_square = mv.to_square >> 8;
                self.clear_square(&opponent_pawn_square);
            }
            Colour::Black => {
                opponent_pawn_square = mv.to_square << 8;
                self.clear_square(&opponent_pawn_square);
            }
        }
    }
    
    pub fn play_promotion(&mut self, mv: Move) {
        self.clear_square(&mv.to_square);
        self.clear_square(&mv.from_square);
        self.set_square(&mv.to_square, &mv.promotion, &mv.colour);
    }
    
    // Swaps the board turn
    pub fn swap_turn(&mut self) {
        if self.turn == Colour::White {
            self.turn = Colour::Black;
        } else {
            self.turn = Colour::White;
        }
    }
    
    // Determines if the king of specified colour is in check on a given board
    pub fn is_check(&self, colour_to_check: &Colour) -> bool {
        let king_square: Bitboard;
        match colour_to_check {

            Colour::White => {
                king_square = self.colour.white & self.role.king;
                
                // white pawn attacks are opposite of black pawn attacks
                if get_white_pawn_attacks(self, &king_square) & (self.colour.black & self.role.pawn) != EMPTY_BITBOARD {
                    return true;
                } else if get_knight_attacks(self, &king_square) & (self.colour.black & self.role.knight) != EMPTY_BITBOARD {
                    return true;
                } else if get_bishop_attacks(self, &king_square) & (self.colour.black & self.role.bishop) != EMPTY_BITBOARD {
                    return true;
                } else if get_rook_attacks(self, &king_square) & (self.colour.black & self.role.rook) != EMPTY_BITBOARD {
                    return true;
                } else if get_queen_attacks(self, &king_square) & (self.colour.black & self.role.queen) != EMPTY_BITBOARD {
                    return true;
                } else {
                    return false;
                }
            }
            
            Colour::Black => {
                king_square = self.colour.black & self.role.king;
                
                // white pawn attacks are opposite of black pawn attacks
                if get_black_pawn_attacks(self, &king_square) & (self.colour.white & self.role.pawn) != EMPTY_BITBOARD {
                    return true;
                } else if get_knight_attacks(self, &king_square) & (self.colour.white & self.role.knight) != EMPTY_BITBOARD {
                    return true;
                } else if get_bishop_attacks(self, &king_square) & (self.colour.white & self.role.bishop) != EMPTY_BITBOARD {
                    return true;
                } else if get_rook_attacks(self, &king_square) & (self.colour.white & self.role.rook) != EMPTY_BITBOARD {
                    return true;
                } else if get_queen_attacks(self, &king_square) & (self.colour.white & self.role.queen) != EMPTY_BITBOARD {
                    return true;
                } else {
                    return false;
                }
            }

        }
    }

    pub fn is_checkmate(&self, colour_to_check: &Colour) -> bool {
        if self.is_check(colour_to_check) & (get_legal_moves(self).len() == 0) {
            return true;
        } else {
            return false;
        }
    }

    pub fn is_stalemate(&self, colour_to_check: &Colour) -> bool {
        if !self.is_check(colour_to_check) & (get_legal_moves(self).len() == 0) {
            return true;
        } else {
            return false;
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
                    } else {
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