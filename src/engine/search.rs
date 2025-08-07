use crate::colour::Colour;
use crate::board::{Board};
use crate::movegen::get_legal_moves;
use crate::engine::eval::evaluate;
use std::cmp;

pub fn minmax(current_board: &Board, depth: i32, is_bots_move: bool, mut alpha: i32, mut beta: i32, bot_colour: &Colour) -> i32 {
    
    if depth == 0 {
        return evaluate(&current_board, &bot_colour);
        //return quiesce(current_board, bot_colour, is_bots_move, alpha, beta);
    }
    
    if is_bots_move {
        if current_board.is_checkmate(bot_colour) {
            return i32::MIN;
        }
        
        // Null move pruning
        /*
        if depth >= 2 && !current_board.is_check(bot_colour) {
            let null_move_board = make_null_move(current_board.clone());
            let eval = minmax( null_move_board, depth - 2, false, alpha, beta, bot_colour);
            if eval >= beta {
                return beta;
            }
        }
        */
        
        let mut max_eval = i32::MIN;
        for mv in get_legal_moves(&current_board) {
            let mut new_board = current_board.clone();
            new_board.play_unsafe(mv);
            let eval = minmax(&new_board, depth - 1, false, alpha, beta, bot_colour);
            max_eval = cmp::max(max_eval, eval);
            alpha = cmp::max(alpha, eval);
            if beta <= alpha {
                break;
            }
        }
        
        return max_eval;
    } else {
        if current_board.is_checkmate(bot_colour) {
            return i32::MAX;
        }

        let mut min_eval = i32::MAX;
        for mv in get_legal_moves(&current_board) {
            let mut new_board = current_board.clone();
            new_board.play_unsafe(mv);
            let eval = minmax(&new_board, depth - 1, true, alpha, beta, bot_colour);
            min_eval = cmp::min(min_eval, eval);
            beta = cmp::min(beta, eval);
            if beta <= alpha {
                break;
            }
        }

        return min_eval;
    }
}