use crate::mv::Move;
use crate::{bitboard::Bitboard, square::Square};
use crate::board::Board;
use crate::colour::Colour;
use crate::role::Role;

pub enum CastleSide {
    KingSide,
    QueenSide
}

#[derive(Debug, Clone, Copy)]
pub struct ByCastleSide<T> {
    pub kingside: T,
    pub queenside: T,
}

// The squares in between the rook and king that need to be unoccupied to castle
pub const WHITE_KINGSIDE_CASTLE_SQUARES: Bitboard = Bitboard(0b1110);
pub const WHITE_QUEENSIDE_CASTLE_SQUARES: Bitboard = Bitboard(0b111000);

pub const BLACK_KINGSIDE_CASTLE_SQUARES: Bitboard = Bitboard(0b111000000000000000000000000000000000000000000000000000000000);
pub const BLACK_QUEENSIDE_CASTLE_SQUARES: Bitboard = Bitboard(0b11100000000000000000000000000000000000000000000000000000000000);

/*
pub fn check_rook_move(mv: &Move) {
    if mv.from_square = 
}*/


impl Board {
    
    // Determines what castling rights to remove from a move
    pub fn castling_rights(&mut self, mv: Move) {
        
        if mv.role == Some(Role::King) {
            match self.turn {
                
                Colour::White => {
                    self.castling_rights.white.kingside = false;
                    self.castling_rights.white.queenside = false;
                }
            
                Colour::Black => {
                    self.castling_rights.black.kingside = false;
                    self.castling_rights.black.queenside = false;
                }
            }
        }

        if mv.role == Some(Role::Rook) {
            match self.turn {

                Colour::White => {
                    if mv.from_square == Square::A1 {
                        self.castling_rights.white.queenside = false;
                    } else if mv.from_square == Square::H1 {
                        self.castling_rights.white.kingside = false;
                    }
                }
                
                Colour::Black => {
                    if mv.from_square == Square::A8 {
                        self.castling_rights.black.queenside = false;
                    } else if mv.from_square == Square::H8 {
                        self.castling_rights.black.kingside = false;
                    }
                }
            }
        }
    

    }
}