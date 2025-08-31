use crate::square::{NEIGHBOUR_FILES};
use crate::{board::Board};
use crate::colour::Colour;
use crate::castle::CastleSide;
use crate::engine::search::NODE_COUNT;
use std::sync::atomic::{Ordering};
use crate::bitboard::{Bitboard, EMPTY_BITBOARD};

const KNIGHT_ATTACK_COUNT: [i32; 64] = 
[2,3,4,4,4,4,3,2,
 3,4,6,6,6,6,4,3,
 4,6,8,8,8,8,6,4,
 4,6,8,8,8,8,6,4,
 4,6,8,8,8,8,6,4,
 4,6,8,8,8,8,6,4,
 3,4,6,6,6,6,4,3,
 2,3,4,4,4,4,3,2];

// Pawn structure bitboards
const WHITE_KINGSIDE_PAWN_STRUCTURE: Bitboard = Bitboard(0b10000011100000000);
const WHITE_QUEENSIDE_PAWN_STRUCTURE: Bitboard = Bitboard(0b100000001110000000000000);
const BLACK_KINGSIDE_PAWN_STRUCTURE: Bitboard = Bitboard(0b111000000010000000000000000000000000000000000000000);
const BLACK_QUEENSIDE_PAWN_STRUCTURE: Bitboard = Bitboard(0b11100000100000000000000000000000000000000000000000000000);

const BISHOP_BONUS_AREA: Bitboard = Bitboard(0x7E7E7E7E7E7E00);

const PAWN_MATERIAL_VALUE: i32 = 10000;
const KNIGHT_MATERIAL_VALUE: i32 = 30500;
const BISHOP_MATERIAL_VALUE: i32 = 33300;
const ROOK_MATERIAL_VALUE: i32 = 56300;
const QUEEN_MATERIAL_VALUE: i32 = 95000;

// Provides a positive i32 if the colour provided is doing better than the other colour, and a negative value if the colour is doing worse
pub fn evaluate(board: &Board, colour: &Colour) -> i32 {
    
    NODE_COUNT.fetch_add(1, Ordering::Relaxed);

    let white_pawns = board.role.pawn & board.colour.white;
    let white_knights = board.role.knight & board.colour.white;
    let white_bishops = board.role.bishop & board.colour.white;
    let white_rooks = board.role.rook & board.colour.white;
    let white_queens = board.role.queen & board.colour.white;

    let black_pawns = board.role.pawn & board.colour.black;
    let black_knights = board.role.knight & board.colour.black;
    let black_bishops = board.role.bishop & board.colour.black;
    let black_rooks = board.role.rook & board.colour.black;
    let black_queens = board.role.queen & board.colour.black;
    
    let mut evaluation: i32 = 0;
    
    // Material evaluation
    evaluation += pawn_evaluation(board, &white_pawns, colour) - pawn_evaluation(board, &black_pawns, colour);
    
    evaluation += knight_evaluation(&white_knights) - knight_evaluation(&black_knights);
    
    evaluation += bishop_evaluation(&white_bishops) - bishop_evaluation(&black_bishops);

    evaluation += (white_rooks.count_ones() - black_rooks.count_ones()) as i32 * ROOK_MATERIAL_VALUE;
    evaluation += (white_queens.count_ones() - black_queens.count_ones()) as i32 * QUEEN_MATERIAL_VALUE;
    
    match colour {
        Colour::White => return evaluation,
        Colour::Black => return -evaluation,
    }
}

pub fn pawn_evaluation(board: &Board, pawns: &Bitboard, colour: &Colour) -> i32 {
    let mut pawn_evaluation: i32 = 0;
    
    pawn_evaluation += pawns.count_ones() as i32 * PAWN_MATERIAL_VALUE;
    
    // Half a pawn penalty for doubled pawns and isolated pawns
    for (file, neighbours) in NEIGHBOUR_FILES {
        match (file & pawns).count_ones() {
            x if x > 1 => pawn_evaluation -= 1000,
            0 => (),
            _ => {
                if (neighbours & pawns) == EMPTY_BITBOARD {
                    pawn_evaluation -= 1000;
                }
            }
        }
    }

    match colour {
        Colour::White => {
            if board.castle_side.white == Some(CastleSide::KingSide) {
                if (pawns & WHITE_KINGSIDE_PAWN_STRUCTURE).count_ones() > 2 {
                    pawn_evaluation += 1000;
                }
            } else if board.castle_side.white == Some(CastleSide::QueenSide) {
                if (pawns & WHITE_QUEENSIDE_PAWN_STRUCTURE).count_ones() > 2 {
                    pawn_evaluation += 1000;
                }
            } else {
                if board.castling_rights.white.kingside == true {
                    if (pawns & WHITE_KINGSIDE_PAWN_STRUCTURE).count_ones() > 2 {
                        pawn_evaluation += 500;
                    }
                }
                if board.castling_rights.white.queenside == true {
                    if (pawns & WHITE_QUEENSIDE_PAWN_STRUCTURE).count_ones() > 2 {
                        pawn_evaluation += 500;
                    }
                }
            }
        }
        Colour::Black => {
            if board.castle_side.black == Some(CastleSide::KingSide) {
                if (pawns & BLACK_KINGSIDE_PAWN_STRUCTURE).count_ones() > 2 {
                    pawn_evaluation -= 1000;
                }
            } else if board.castle_side.black == Some(CastleSide::QueenSide) {
                if (pawns & BLACK_QUEENSIDE_PAWN_STRUCTURE).count_ones() > 2 {
                    pawn_evaluation -= 1000;
                }
            } else {
                if board.castling_rights.black.kingside == true {
                    if (pawns & BLACK_KINGSIDE_PAWN_STRUCTURE).count_ones() > 2 {
                        pawn_evaluation -= 500;
                    }
                }
                if board.castling_rights.black.queenside == true {
                    if (pawns & BLACK_QUEENSIDE_PAWN_STRUCTURE).count_ones() > 2 {
                        pawn_evaluation -= 500;
                    }
                }
            }
        }
    }
    
    return pawn_evaluation;
}

pub fn knight_evaluation(knights: &Bitboard) -> i32 {
    let mut knight_evaluation: i32 = 0;

    knight_evaluation += knights.count_ones() as i32 * KNIGHT_MATERIAL_VALUE;

    for knight in knights.get_component_bitboards() {
        knight_evaluation += KNIGHT_ATTACK_COUNT[knight.trailing_zeros() as usize] * 250;
    }

    return knight_evaluation;
}

pub fn bishop_evaluation(bishops: &Bitboard) -> i32 {
    let mut bishop_evaluation: i32 = 0;

    bishop_evaluation += bishops.count_ones() as i32 * BISHOP_MATERIAL_VALUE;

    if bishops.count_ones() > 1 {
        bishop_evaluation += 1000;
    }

    bishop_evaluation += (bishops & BISHOP_BONUS_AREA).count_ones() as i32 * 1000;

    return bishop_evaluation;

}