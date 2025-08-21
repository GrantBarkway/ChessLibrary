use crate::square::{NEIGHBOUR_FILES, FILE_D, FILE_E};
use crate::{board::Board};
use crate::colour::Colour;
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

const E_D_FILE_CENTER_EIGHT: Bitboard = Bitboard(0b0000000000000000000110000001100000011000000110000000000000000000);

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
    evaluation += pawn_evaluation(&white_pawns) - pawn_evaluation(&black_pawns);

    evaluation += knight_evaluation(&white_knights) - knight_evaluation(&black_knights);
    
    evaluation += bishop_evaluation(&white_bishops) - bishop_evaluation(&black_bishops);

    evaluation += (white_rooks.count_ones() - black_rooks.count_ones()) as i32 * 5630;
    evaluation += (white_queens.count_ones() - black_queens.count_ones()) as i32 * 9500;
    
    match colour {
        Colour::White => return evaluation,
        Colour::Black => return -evaluation,
    }
}

pub fn pawn_evaluation(pawns: &Bitboard) -> i32 {
    let mut pawn_evaluation: i32 = 0;
    
    pawn_evaluation += pawns.count_ones() as i32 * 1000;
    
    // Half a pawn penalty for doubled pawns and isolated pawns
    for (file, neighbours) in NEIGHBOUR_FILES {
        match (file & pawns).count_ones() {
            x if x > 1 => pawn_evaluation -= 500,
            0 => (),
            _ => {
                if (neighbours & pawns) == EMPTY_BITBOARD {
                    pawn_evaluation -= 500;
                }
            }
        }
    }
    
    return pawn_evaluation;
}

pub fn knight_evaluation(knights: &Bitboard) -> i32 {
    let mut knight_evaluation: i32 = 0;

    knight_evaluation += knights.count_ones() as i32 * 3050;

    for knight in knights.get_component_bitboards() {
        knight_evaluation += KNIGHT_ATTACK_COUNT[knight.trailing_zeros() as usize] * 100
    }

    return knight_evaluation;
}

pub fn bishop_evaluation(bishops: &Bitboard) -> i32 {
    let mut bishop_evaluation: i32 = 0;

    bishop_evaluation += bishops.count_ones() as i32 * 3330;

    if bishops.count_ones() > 1 {
        bishop_evaluation += 500;
    }

    return bishop_evaluation;

}