// Just while testing
#![allow(dead_code, unused_variables)]

use crate::board::{Board};
use crate::colour::Colour;
use crate::bitboard::Bitboard;
use crate::mv::Move;
use crate::square::{EIGHTH_RANK, FILE_A, FILE_H, FIRST_RANK, SECOND_RANK, SEVENTH_RANK};

const KNIGHT_ATTACKS: [Bitboard; 64] = [
    Bitboard(0x0000000000020400), Bitboard(0x0000000000050800), Bitboard(0x00000000000a1100), Bitboard(0x0000000000142200), Bitboard(0x0000000000284400), Bitboard(0x0000000000508800), Bitboard(0x0000000000a01000), Bitboard(0x0000000000402000),
    Bitboard(0x0000000002040004), Bitboard(0x0000000005080008), Bitboard(0x000000000a110011), Bitboard(0x0000000014220022), Bitboard(0x0000000028440044), Bitboard(0x0000000050880088), Bitboard(0x00000000a0100010), Bitboard(0x0000000040200020),
    Bitboard(0x0000000204000402), Bitboard(0x0000000508000805), Bitboard(0x0000000a1100110a), Bitboard(0x0000001422002214), Bitboard(0x0000002844004428), Bitboard(0x0000005088008850), Bitboard(0x000000a0100010a0), Bitboard(0x0000004020002040),
    Bitboard(0x0000020400040200), Bitboard(0x0000050800080500), Bitboard(0x00000a1100110a00), Bitboard(0x0000142200221400), Bitboard(0x0000284400442800), Bitboard(0x0000508800885000), Bitboard(0x0000a0100010a000), Bitboard(0x0000402000204000),
    Bitboard(0x0002040004020000), Bitboard(0x0005080008050000), Bitboard(0x000a1100110a0000), Bitboard(0x0014220022140000), Bitboard(0x0028440044280000), Bitboard(0x0050880088500000), Bitboard(0x00a0100010a00000), Bitboard(0x0040200020400000),
    Bitboard(0x0204000402000000), Bitboard(0x0508000805000000), Bitboard(0x0a1100110a000000), Bitboard(0x1422002214000000), Bitboard(0x2844004428000000), Bitboard(0x5088008850000000), Bitboard(0xa0100010a0000000), Bitboard(0x4020002040000000),
    Bitboard(0x0400040200000000), Bitboard(0x0800080500000000), Bitboard(0x1100110a00000000), Bitboard(0x2200221400000000), Bitboard(0x4400442800000000), Bitboard(0x8800885000000000), Bitboard(0x100010a000000000), Bitboard(0x2000204000000000),
    Bitboard(0x0004020000000000), Bitboard(0x0008050000000000), Bitboard(0x00110a0000000000), Bitboard(0x0022140000000000), Bitboard(0x0044280000000000), Bitboard(0x0088500000000000), Bitboard(0x0010a00000000000), Bitboard(0x0020400000000000)
];

const KING_MOVE_SHIFT: [i32; 8] = [
  -9, -8, -7,
  -1,      1,
   7,  8,  9];

pub fn get_legal_moves(board: Board, colour: Colour) -> Vec<Move> {
    let mut legal_moves: Vec<Move> = Vec::new();
    legal_moves.extend(get_king_moves(&board, &colour));
    legal_moves.extend(get_pawn_moves(&board, &colour));
    legal_moves.extend(get_knight_moves(&board, &colour));
    return legal_moves;
}

pub fn is_check() {

}

pub fn get_king_moves(board: &Board, colour: &Colour) -> Vec<Move> {
    let mut move_vector: Vec<Move> = Vec::new();
    let king_bitboard: Bitboard;
    let turn_colour: Bitboard;
    let mut move_bitboard: Bitboard = Bitboard(0);
    match colour {
        Colour::White => (king_bitboard, turn_colour) = (board.colour.white & board.role.king, board.colour.white),
        Colour::Black => (king_bitboard, turn_colour) = (board.colour.black & board.role.king, board.colour.black),
    }
    
    move_bitboard |= (king_bitboard & !FILE_A).shift(1);
    move_bitboard |= (king_bitboard & !FILE_H).shift(-1);
    move_bitboard |= (king_bitboard & !FIRST_RANK).shift(-8);
    move_bitboard |= (king_bitboard & !EIGHTH_RANK).shift(8);
    
    move_bitboard |= (king_bitboard & !(FILE_A|FIRST_RANK)).shift(-7);
    move_bitboard |= (king_bitboard & !(FILE_A|EIGHTH_RANK)).shift(9);
    move_bitboard |= (king_bitboard & !(FILE_H|FIRST_RANK)).shift(-9);
    move_bitboard |= (king_bitboard & !(FILE_H|EIGHTH_RANK)).shift(7);
    
    // Need to also add checks for if the square is attacked
    for single_move in move_bitboard.get_component_bitboards() {
        if (single_move & turn_colour).count_ones() == 0 {
            move_vector.push(Move::new(&board, &king_bitboard, &single_move));
        }
    }

    return move_vector;
}

pub fn get_knight_moves(board: &Board, colour: &Colour) -> Vec<Move> {
    let mut move_vector: Vec<Move> = Vec::new();
    let knight_bitboard: Bitboard;
    let turn_colour: Bitboard;
    match colour {
        Colour::White => (knight_bitboard, turn_colour) = (board.colour.white & board.role.knight, board.colour.white),
        Colour::Black => (knight_bitboard, turn_colour) = (board.colour.black & board.role.knight, board.colour.black),
    }
    for individual_knight in knight_bitboard.get_component_bitboards() {
        for knight_move in KNIGHT_ATTACKS[individual_knight.0.trailing_zeros() as usize].get_component_bitboards() {
            if (knight_move & turn_colour).count_ones() == 0 {
                move_vector.push(Move::new(&board, &individual_knight, &knight_move));
            }
        }
    }
    return move_vector;
}

pub fn get_pawn_moves(board: &Board, colour: &Colour) -> Vec<Move> {
    let mut move_vector: Vec<Move> = Vec::new();
    let pawn_bitboard: Bitboard;
    let starting_rank: Bitboard;
    let move_shift: i32;
    match colour {
        Colour::White => (pawn_bitboard, starting_rank, move_shift) = ((board.colour.white & board.role.pawn), SECOND_RANK, 8),
        Colour::Black => (pawn_bitboard, starting_rank, move_shift) = ((board.colour.black & board.role.pawn), SEVENTH_RANK, -8),
    }
    for individual_pawn in pawn_bitboard.get_component_bitboards() {
        let mut moved_pawn = individual_pawn;
        if (moved_pawn & starting_rank).count_ones() != 0 {
            moved_pawn = individual_pawn.shift(move_shift);
            if (moved_pawn & board.occupied).count_ones() == 0 {
                move_vector.push(Move::new(&board, &individual_pawn, &moved_pawn));
            }
        }
        moved_pawn = moved_pawn.shift(move_shift);
        if (moved_pawn & board.occupied).count_ones() == 0 {
            move_vector.push(Move::new(&board, &individual_pawn, &moved_pawn));
        }
    }
    return move_vector;
}