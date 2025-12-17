use crate::movegen::{get_king_attacks, get_bishop_attacks, get_black_pawn_attacks, get_knight_attacks, get_queen_attacks, get_rook_attacks, get_white_pawn_attacks};
use crate::square::{NEIGHBOUR_FILES};
use crate::{board::Board};
use crate::colour::Colour;
use crate::castle::CastleSide;
use crate::engine::search::NODE_COUNT;
use std::sync::atomic::{Ordering};
use crate::bitboard::{Bitboard, EMPTY_BITBOARD};

// Pawn structure bitboards
const WHITE_KINGSIDE_PAWN_STRUCTURE: Bitboard = Bitboard(0b10000011100000000);
const WHITE_QUEENSIDE_PAWN_STRUCTURE: Bitboard = Bitboard(0b100000001110000000000000);
const BLACK_KINGSIDE_PAWN_STRUCTURE: Bitboard = Bitboard(0b111000000010000000000000000000000000000000000000000);
const BLACK_QUEENSIDE_PAWN_STRUCTURE: Bitboard = Bitboard(0b11100000100000000000000000000000000000000000000000000000);

const PAWN_MATERIAL_VALUE: i32 = 10000;
const KNIGHT_MATERIAL_VALUE: i32 = 30500;
const BISHOP_MATERIAL_VALUE: i32 = 33300;
const ROOK_MATERIAL_VALUE: i32 = 56300;
const QUEEN_MATERIAL_VALUE: i32 = 95000;

// Provides a positive i32 if the colour provided is doing better than the other colour, and a negative value if the colour is doing worse
pub fn evaluate(board: &Board, colour: &Colour) -> i32 {
    
    NODE_COUNT.fetch_add(1, Ordering::Relaxed);
    
    let mut evaluation: i32 = 0;
    
    // Material evaluation
    evaluation += material_evaluation(board);
    evaluation += castling_bonus(board, &Colour::White) - castling_bonus(board, &Colour::Black);
    
    let mut adjusted_mobility_evaluation: f32 = evaluation as f32;
    
    match colour {
        Colour::White => {
            if (evaluation != i32::MIN) & (evaluation != i32::MAX) {
                adjusted_mobility_evaluation *= calculate_attack_mobility(board, &Colour::White);
            }
            evaluation = adjusted_mobility_evaluation as i32;
            return evaluation
        },
        Colour::Black => {
            if (evaluation != i32::MIN) & (evaluation != i32::MAX) {
                adjusted_mobility_evaluation *= calculate_attack_mobility(board, &Colour::Black);
            }
            evaluation = adjusted_mobility_evaluation as i32;
            return -evaluation
        },
    }
}

pub fn material_evaluation(board: &Board) -> i32 {
    let mut material_evaluation: i32 = 0;

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
    
    material_evaluation += (white_pawns.count_ones() - black_pawns.count_ones()) as i32 * PAWN_MATERIAL_VALUE;
    material_evaluation += (white_knights.count_ones() - black_knights.count_ones()) as i32 * KNIGHT_MATERIAL_VALUE;
    material_evaluation += (white_bishops.count_ones() - black_bishops.count_ones()) as i32 * BISHOP_MATERIAL_VALUE;
    material_evaluation += (white_rooks.count_ones() - black_rooks.count_ones()) as i32 * ROOK_MATERIAL_VALUE;
    material_evaluation += (white_queens.count_ones() - black_queens.count_ones()) as i32 * QUEEN_MATERIAL_VALUE;

    return material_evaluation;
}

pub fn castling_bonus(board: &Board, colour: &Colour) -> i32 {
    let mut pawn_evaluation: i32 = 0;
    let pawns: Bitboard;
    match colour {
        Colour::White => {
            pawns = board.colour.white & board.role.pawn;
            if board.castle_side.white == Some(CastleSide::KingSide) {
                if (pawns & WHITE_KINGSIDE_PAWN_STRUCTURE).count_ones() > 2 {
                    pawn_evaluation += PAWN_MATERIAL_VALUE/4;
                }
                pawn_evaluation += PAWN_MATERIAL_VALUE/4;
            } else if board.castle_side.white == Some(CastleSide::QueenSide) {
                if (pawns & WHITE_QUEENSIDE_PAWN_STRUCTURE).count_ones() > 2 {
                    pawn_evaluation += PAWN_MATERIAL_VALUE/4;
                }
                pawn_evaluation += PAWN_MATERIAL_VALUE/4;
            } else {
                if board.castling_rights.white.kingside == true {
                    if (pawns & WHITE_KINGSIDE_PAWN_STRUCTURE).count_ones() > 2 {
                        pawn_evaluation += PAWN_MATERIAL_VALUE/8;
                    }
                    pawn_evaluation += PAWN_MATERIAL_VALUE/8;
                }
                if board.castling_rights.white.queenside == true {
                    if (pawns & WHITE_QUEENSIDE_PAWN_STRUCTURE).count_ones() > 2 {
                        pawn_evaluation += PAWN_MATERIAL_VALUE/8;
                    }
                    pawn_evaluation += PAWN_MATERIAL_VALUE/8;
                }
            }
        }
        Colour::Black => {
            pawns = board.colour.black & board.role.pawn;
            if board.castle_side.black == Some(CastleSide::KingSide) {
                if (pawns & BLACK_KINGSIDE_PAWN_STRUCTURE).count_ones() > 2 {
                    pawn_evaluation += PAWN_MATERIAL_VALUE/4;
                }
                pawn_evaluation += PAWN_MATERIAL_VALUE/4;
            } else if board.castle_side.black == Some(CastleSide::QueenSide) {
                if (pawns & BLACK_QUEENSIDE_PAWN_STRUCTURE).count_ones() > 2 {
                    pawn_evaluation += PAWN_MATERIAL_VALUE/4;
                }
                pawn_evaluation += PAWN_MATERIAL_VALUE/4;
            } else {
                if board.castling_rights.black.kingside == true {
                    if (pawns & BLACK_KINGSIDE_PAWN_STRUCTURE).count_ones() > 2 {
                        pawn_evaluation += PAWN_MATERIAL_VALUE/8;
                    }
                    pawn_evaluation += PAWN_MATERIAL_VALUE/8;
                }
                if board.castling_rights.black.queenside == true {
                    if (pawns & BLACK_QUEENSIDE_PAWN_STRUCTURE).count_ones() > 2 {
                        pawn_evaluation += PAWN_MATERIAL_VALUE/8;
                    }
                    pawn_evaluation += PAWN_MATERIAL_VALUE/8;
                }
            }
        }
    }
    return pawn_evaluation
}

pub fn calculate_attack_mobility(board: &Board, colour: &Colour) -> f32 {
    let mut white_attack_count: f32 = 0.0;
    
    white_attack_count += get_king_attacks(board, &(board.colour.white & board.role.king)).count_ones() as f32 * 1.0;
    white_attack_count += get_white_pawn_attacks(board, &(board.colour.white & board.role.pawn)).count_ones() as f32;
    white_attack_count += get_knight_attacks(board, &(board.colour.white & board.role.knight)).count_ones() as f32 * 1.0;
    white_attack_count += get_bishop_attacks(board, &(board.colour.white & board.role.bishop)).count_ones() as f32 * 1.0;
    white_attack_count += get_rook_attacks(board, &(board.colour.white & board.role.rook)).count_ones() as f32 * 1.0;
    white_attack_count += get_queen_attacks(board, &(board.colour.white & board.role.queen)).count_ones() as f32 * 1.0;
    
    let mut black_attack_count: f32 = 0.0;
    black_attack_count += get_king_attacks(board, &(board.colour.black & board.role.king)).count_ones() as f32 * 1.0;
    black_attack_count += get_black_pawn_attacks(board, &(board.colour.black & board.role.pawn)).count_ones() as f32;
    black_attack_count += get_knight_attacks(board, &(board.colour.black & board.role.knight)).count_ones() as f32 * 1.0;
    black_attack_count += get_bishop_attacks(board, &(board.colour.black & board.role.bishop)).count_ones() as f32 * 1.0;
    black_attack_count += get_rook_attacks(board, &(board.colour.black & board.role.rook)).count_ones() as f32 * 1.0;
    black_attack_count += get_queen_attacks(board, &(board.colour.black & board.role.queen)).count_ones() as f32 * 1.0;
    
    match colour {
        Colour::White => return white_attack_count/black_attack_count,
        Colour::Black => return black_attack_count/white_attack_count
    }
}