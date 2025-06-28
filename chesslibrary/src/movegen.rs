// Just while testing
#![allow(dead_code, unused_variables)]

use crate::board::{Board};
use crate::colour::Colour;
use crate::bitboard::Bitboard;
use crate::mv::Move;

const KING_MOVE_SHIFT: [i32; 8] = [
  -9, -8, -7,
  -1,      1,
   7,  8,  9];

pub fn get_legal_moves(board: Board) -> Vec<Move> {
    return get_king_moves(board);
}

pub fn is_check() {

}

pub fn get_king_moves(board: Board) -> Vec<Move> {
    let mut move_vector: Vec<Move> = Vec::new();
    let king_bitboard: Bitboard;
    let turn_colour: Bitboard;
    match board.turn {
        Colour::White => (king_bitboard, turn_colour) = (Bitboard(board.colour.white.0 & board.role.king.0), board.colour.white),
        Colour::Black => (king_bitboard, turn_colour) = (Bitboard(board.colour.black.0 & board.role.king.0), board.colour.black),
    }
    for i in KING_MOVE_SHIFT {
        let shifted_bit = king_bitboard.shift(i);
        eprintln!("{:?}", shifted_bit);
        // Need to add if in check functionality eventually
        if (shifted_bit.0 & turn_colour.0).count_ones() == 0 {
            move_vector.push(Move::new(&board,&king_bitboard, &shifted_bit))
        }
    }
    return move_vector;
}