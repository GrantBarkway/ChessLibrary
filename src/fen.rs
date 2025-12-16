use crate::board::Board;
use crate::bitboard::{Bitboard, string_move_to_bitboard};
use crate::colour::Colour;
use crate::role::Role;

impl Board {
    
    // Not very efficient, just need primitive for testing
    pub fn from_fen(fen: String) -> Board {
        let mut board = Board::empty_board();
        let mut mask: u64 = 0b1000000000000000000000000000000000000000000000000000000000000000; 
        let fen_vec: Vec<&str> = fen.split_whitespace().collect();
        while mask != 0 {
            for char in fen_vec[0].chars() {
                if char == 'r' {
                    board.set_square(&Bitboard(mask), &Some(Role::Rook), &Some(Colour::Black));
                    mask = mask >> 1;
                } else if char == 'n' {
                    board.set_square(&Bitboard(mask), &Some(Role::Knight), &Some(Colour::Black));
                    mask = mask >> 1;
                } else if char == 'b' {
                    board.set_square(&Bitboard(mask), &Some(Role::Bishop), &Some(Colour::Black));
                    mask = mask >> 1;
                } else if char == 'k' {
                    board.set_square(&Bitboard(mask), &Some(Role::King), &Some(Colour::Black));
                    mask = mask >> 1;
                } else if char == 'q' {
                    board.set_square(&Bitboard(mask), &Some(Role::Queen), &Some(Colour::Black));
                    mask = mask >> 1;
                } else if char == 'p' {
                    board.set_square(&Bitboard(mask), &Some(Role::Pawn), &Some(Colour::Black));
                    mask = mask >> 1;
                } else if char == 'R' {
                    board.set_square(&Bitboard(mask), &Some(Role::Rook), &Some(Colour::White));
                    mask = mask >> 1;
                } else if char == 'N' {
                    board.set_square(&Bitboard(mask), &Some(Role::Knight), &Some(Colour::White));
                    mask = mask >> 1;
                } else if char == 'B' {
                    board.set_square(&Bitboard(mask), &Some(Role::Bishop), &Some(Colour::White));
                    mask = mask >> 1;
                } else if char == 'K' {
                    board.set_square(&Bitboard(mask), &Some(Role::King), &Some(Colour::White));
                    mask = mask >> 1;
                } else if char == 'Q' {
                    board.set_square(&Bitboard(mask), &Some(Role::Queen), &Some(Colour::White));
                    mask = mask >> 1;
                } else if char == 'P' {
                    board.set_square(&Bitboard(mask), &Some(Role::Pawn), &Some(Colour::White));
                    mask = mask >> 1;
                } else if char.is_ascii_digit() {
                    if let Some(digit) = char.to_digit(10) {
                        mask = mask >> digit;
                    }
                }
            }
        }
        
        for char in fen_vec[1].chars() {
            if char == 'w' {
                board.turn = Colour::White;
            } else {
                board.turn = Colour::Black;
            }
        }

        for char in fen_vec[2].chars() {
            if char == 'K' {
                board.castling_rights.white.kingside = true;
            } else if char == 'Q' {
                board.castling_rights.white.queenside = true;
            } else if char == 'k' {
                board.castling_rights.black.kingside = true;
            } else if char == 'q' {
                board.castling_rights.black.queenside = true;
            }
        }

        board.en_passant_target_square = string_move_to_bitboard(fen_vec[3]);
        
        // Ignoring half move and full moves for now

        return board;
    }
}