use crate::movegen::{get_king_attacks, get_bishop_attacks, get_black_pawn_attacks, get_knight_attacks, get_rook_attacks, get_white_pawn_attacks};
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
const DFILE_EFILE_PAWN_CENTER_SQUARES: Bitboard = Bitboard(0b0000000000000000000110000001100000011000000110000000000000000000);

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
    evaluation += material_evaluation(board, &Colour::White) - material_evaluation(board, &Colour::Black);
    evaluation += pawn_evaluation(board, &(board.colour.white & board.role.pawn)) - pawn_evaluation(board, &(board.colour.black & board.role.pawn));
    evaluation += castling_evaluation(board, &Colour::White) - castling_evaluation(board, &Colour::Black);
    
    //Testing additional 100 to help with mobility
    let mut adjusted_mobility_evaluation: f32 = evaluation as f32;
    
    match colour {
        Colour::White => {
            adjusted_mobility_evaluation += 100.0;
            if (evaluation != i32::MIN) & (evaluation != i32::MAX) {
                adjusted_mobility_evaluation *= calculate_attack_mobility(board, &Colour::White);
            }
            evaluation = adjusted_mobility_evaluation as i32;
            return evaluation
        },
        Colour::Black => {
            adjusted_mobility_evaluation -= 100.0;
            if (evaluation != i32::MIN) & (evaluation != i32::MAX) {
                adjusted_mobility_evaluation *= calculate_attack_mobility(board, &Colour::Black);
            }
            evaluation = adjusted_mobility_evaluation as i32;
            return -evaluation
        },
    }
}

// Gets overall material evaluation for a colour of 
pub fn material_evaluation(board: &Board, colour: &Colour) -> i32 {
    let mut material_evaluation: i32 = 0;
    let pawns: Bitboard;
    let knights: Bitboard;
    let bishops: Bitboard;
    let rooks: Bitboard;
    let queens: Bitboard;

    match colour {
        Colour::White => {
            pawns = board.role.pawn & board.colour.white;
            knights = board.role.knight & board.colour.white;
            bishops = board.role.bishop & board.colour.white;
            rooks = board.role.rook & board.colour.white;
            queens = board.role.queen & board.colour.white;
        }
        Colour::Black => {
            pawns = board.role.pawn & board.colour.black;
            knights = board.role.knight & board.colour.black;
            bishops = board.role.bishop & board.colour.black;
            rooks = board.role.rook & board.colour.black;
            queens = board.role.queen & board.colour.black;
        }
    }
    
    material_evaluation += pawns.count_ones() as i32 * PAWN_MATERIAL_VALUE;
    material_evaluation += knights.count_ones() as i32 * KNIGHT_MATERIAL_VALUE;
    material_evaluation += bishops.count_ones() as i32 * BISHOP_MATERIAL_VALUE;
    material_evaluation += rooks.count_ones() as i32 * ROOK_MATERIAL_VALUE;
    material_evaluation += queens.count_ones() as i32 * QUEEN_MATERIAL_VALUE;
    
    return material_evaluation;
}

// evaluation for pawns
pub fn pawn_evaluation(board: &Board, pawns: &Bitboard) -> i32 {
    let mut pawn_evaluation: i32 = 0;
    
    pawn_evaluation += (DFILE_EFILE_PAWN_CENTER_SQUARES & pawns).count_ones() as i32 * PAWN_MATERIAL_VALUE/100;

    // Half a pawn penalty for doubled pawns and quarter pawn penalty for isolated pawns
    for (file, neighbours) in NEIGHBOUR_FILES {
        if (file & pawns).count_ones() > 1 {
            pawn_evaluation -= PAWN_MATERIAL_VALUE/2
        }
        if (neighbours & pawns) == EMPTY_BITBOARD {
            pawn_evaluation -= PAWN_MATERIAL_VALUE/4;
        }
        // Passed pawn bonus
        if ((board.role.pawn ^ pawns) & file).count_ones() == 0 {
            pawn_evaluation += PAWN_MATERIAL_VALUE/8;
        }
    }

    return pawn_evaluation;
}

pub fn castling_evaluation(board: &Board, colour: &Colour) -> i32 {
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
    white_attack_count += get_white_pawn_attacks(board, &(board.colour.white & board.role.pawn)).count_ones() as f32 * 1.0;
    white_attack_count += get_knight_attacks(board, &(board.colour.white & board.role.knight)).count_ones() as f32 * 1.1;
    white_attack_count += get_bishop_attacks(board, &(board.colour.white & board.role.bishop)).count_ones() as f32 * 1.1;
    white_attack_count += get_rook_attacks(board, &(board.colour.white & board.role.rook)).count_ones() as f32 * 1.0;
    
    let mut black_attack_count: f32 = 0.0;
    black_attack_count += get_king_attacks(board, &(board.colour.black & board.role.king)).count_ones() as f32 * 1.0;
    black_attack_count += get_black_pawn_attacks(board, &(board.colour.black & board.role.pawn)).count_ones() as f32 * 1.0;
    black_attack_count += get_knight_attacks(board, &(board.colour.black & board.role.knight)).count_ones() as f32 * 1.1;
    black_attack_count += get_bishop_attacks(board, &(board.colour.black & board.role.bishop)).count_ones() as f32 * 1.1;
    black_attack_count += get_rook_attacks(board, &(board.colour.black & board.role.rook)).count_ones() as f32 * 1.0;
    
    match colour {
        Colour::White => return white_attack_count/black_attack_count,
        Colour::Black => return black_attack_count/white_attack_count
    }
}