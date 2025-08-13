use crate::{board::Board};
use crate::colour::Colour;
use crate::engine::search::NODE_COUNT;
use std::sync::atomic::{Ordering};

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
    evaluation += (white_pawns.count_ones() - black_pawns.count_ones()) as i32 * 1000;
    evaluation += (white_knights.count_ones() - black_knights.count_ones()) as i32 * 3050;
    evaluation += (white_bishops.count_ones() - black_bishops.count_ones()) as i32 * 3330;
    evaluation += (white_rooks.count_ones() - black_rooks.count_ones()) as i32 * 5630;
    evaluation += (white_queens.count_ones() - black_queens.count_ones()) as i32 * 9500; 
    
    match colour {
        Colour::White => return evaluation,
        Colour::Black => return -evaluation,
    }
}