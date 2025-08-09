use std::ops::Shr;

use crate::board::Board;
use crate::bitboard::Bitboard;
use crate::colour::Colour;
use crate::role::Role;

impl Board {
    pub fn from_fen(fen: String) -> Board {
        let mut board = Board::empty_board();
        let mut mask: u64 = 0b1000000000000000000000000000000000000000000000000000000000000000;
        while mask != 0 {
            for char in fen.chars() {
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
        return board;
    }
}