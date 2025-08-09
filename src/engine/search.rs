use crate::colour::Colour;
use crate::board::{Board};
use crate::movegen::get_legal_moves;
use crate::engine::eval::evaluate;
use crate::mv::Move;
use std::cmp;
use std::sync::atomic::{AtomicUsize};
use once_cell::sync::Lazy;

pub static NODE_COUNT: Lazy<AtomicUsize> = Lazy::new(|| AtomicUsize::new(0));

pub fn pick_move(board: &Board, depth: i32, bot_colour: &Colour) -> (Option<Move>, i32) {
    
    let legal_moves = get_legal_moves(board);

    let mut best_mv: Option<Move> = None;
    let mut best_mv_evaluation: i32 = i32::MIN;
    
    for mv in legal_moves {
        let mut current_board = board.clone();
        current_board.play_unsafe(mv);
        let eval = minmax(&current_board, depth - 1, false, i32::MIN, i32::MAX, bot_colour);
        if eval > best_mv_evaluation {
            best_mv = Some(mv);
            best_mv_evaluation = eval;
        }
    }

    return (best_mv, best_mv_evaluation);
}

pub fn minmax(current_board: &Board, depth: i32, is_bots_move: bool, mut alpha: i32, mut beta: i32, bot_colour: &Colour) -> i32 {
    
    if depth == 0 {
        return quiesce(current_board, bot_colour, is_bots_move, alpha, beta);
    }
    
    if is_bots_move {
        if current_board.is_checkmate(bot_colour) {
            return i32::MIN;
        }
        
        if depth >= 2 && !current_board.is_check(bot_colour) {
            let mut null_move_board: Board = current_board.clone();
            null_move_board.swap_turn();
            let eval = minmax( &null_move_board, depth - 2, false, alpha, beta, bot_colour);
            if eval >= beta {
                return beta;
            }
        }
        
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

// Quiescence search to only evaluate positions with no tactical move to prevent bad trades when max depth is reached
fn quiesce(current_board: &Board, bot_colour: &Colour, is_bots_move: bool, mut alpha: i32, mut beta: i32) -> i32 {
    let stand_pat = evaluate(current_board, bot_colour);
    let mut best_value = stand_pat;
    
    if is_bots_move {
        if best_value >= beta {
            return best_value;
        }
        alpha = cmp::max(alpha, best_value);
        for mv in get_legal_moves(current_board).iter().filter(|mv| mv.capture) {
            let mut new_board = current_board.clone();
            new_board.play_unsafe(*mv);
            let score = quiesce(&new_board, bot_colour, false, alpha, beta);
            best_value = cmp::max(best_value, score);
            alpha = cmp::max(alpha,best_value);
            if alpha >= beta {
                break;
            }
        }
        return best_value;
    } else {
        if best_value <= alpha {
            return best_value;
        }
        beta = cmp::min(beta, best_value);
        for mv in get_legal_moves(current_board).iter().filter(|mv| mv.capture) {
            let mut new_board = current_board.clone();
            new_board.play_unsafe(*mv);
            let score = quiesce(&new_board, bot_colour, true, alpha, beta);
            best_value = cmp::min(best_value, score);
            beta = cmp::min(beta,best_value);
            if alpha >= beta {
                break;
            }
        }
        return best_value;
    }
}