use crate::{board::Board, movegen::get_legal_moves, mv};
use crate::bitboard::bitboard_to_string_move;

pub fn perft_test(board: &Board, depth: i32) -> i64 {
    return perft_recursive(board, depth, 0);
}

pub fn perft_recursive(board: &Board, depth: i32, mut total_nodes: i64) -> i64 {
    
    if depth <= 0 {
        return 1;
    }
    
    for mv in get_legal_moves(board) {
        //println!("{:?} {} to {} (castle: {}, en_passant_target_square: {}, promotion: {:?}, capture: {:?})", mv.role, bitboard_to_string_move(mv.from_square), bitboard_to_string_move(mv.to_square), mv.castle, mv.en_passant_target, mv.promotion, mv.capture);
        let mut board_copy = board.clone();
        board_copy.play_unsafe(mv);
        total_nodes += perft_recursive(&board_copy, depth - 1, 0);
    }
    
    return total_nodes;
}