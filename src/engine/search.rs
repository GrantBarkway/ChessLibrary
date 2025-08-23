use crate::colour::Colour;
use crate::board::{Board};
use crate::movegen::get_legal_moves;
use crate::engine::eval::evaluate;
use crate::mv::Move;
use crate::uci::to_uci;
use std::cmp;
use std::cmp::Reverse;
use std::sync::atomic::{AtomicUsize, Ordering};
use once_cell::sync::Lazy;
use arrayvec::ArrayVec;
use std::time::{Duration, Instant};

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

pub static NODE_COUNT: Lazy<AtomicUsize> = Lazy::new(|| AtomicUsize::new(0));

#[pyfunction]
pub fn pick_move(board_fen: String, bot_time: u64, bot_colour: String) -> PyResult<(String, i32)> {
    
    NODE_COUNT.store(0, Ordering::Relaxed);
    
    let start_time = Instant::now();
    
    let bot_colour = match bot_colour.as_str() {
        "white" => Colour::White,
        "black" => Colour::Black,
        _ => return Ok(("Invalid colour.".to_string(), 0)),
    };
    
    let board = Board::from_fen(board_fen);

    let max_time = Duration::from_millis(bot_time);
    
    let mut ordered_legal_moves = get_legal_moves(&board);
    
    let mut overall_best_mv: Option<Move> = None;
    let mut overall_best_mv_evaluation: i32 = i32::MIN;
    
    let mut current_depth = 1;
    
    while (start_time.elapsed() < max_time) & (current_depth < 50) {
        
        let mut local_best_mv: Option<Move> = None;
        let mut local_best_mv_evaluation: i32 = i32::MIN;
        
        let mut move_evaluation: ArrayVec<(Move, i32), 218> = ArrayVec::<(Move, i32), 218>::new();
        for mv in ordered_legal_moves {
            let mut current_board = board.clone();
            current_board.play_unsafe(mv);
            let eval = minmax(&current_board, current_depth - 1, false, i32::MIN, i32::MAX, &bot_colour, start_time, max_time);
            if eval > local_best_mv_evaluation {
                local_best_mv = Some(mv);
                local_best_mv_evaluation = eval;
            }
            move_evaluation.push((mv, eval));
        }
        
        if (start_time.elapsed() < max_time) & (current_depth < 50) {
            overall_best_mv = local_best_mv;
            overall_best_mv_evaluation = local_best_mv_evaluation;
            
            ordered_legal_moves = order_moves_by_evaluation(move_evaluation);

            current_depth += 1;
        } else {
            break
        }
    }
    
    eprintln!("Move picked: {:#?} with evaluation {}. Nodes searched: {} in {:?} at depth {}", overall_best_mv, overall_best_mv_evaluation, NODE_COUNT.load(std::sync::atomic::Ordering::Relaxed), start_time.elapsed(), current_depth);
    
    return Ok((to_uci(overall_best_mv), overall_best_mv_evaluation));
}

fn minmax(current_board: &Board, depth: i32, is_bots_move: bool, mut alpha: i32, mut beta: i32, bot_colour: &Colour, start_time: Instant, max_time: Duration) -> i32 {
    
    if start_time.elapsed() > max_time {
        return 0;
    }

    if depth == 0 {
        return quiesce(current_board, bot_colour, is_bots_move, alpha, beta, start_time, max_time);
    }
    
    if is_bots_move {

        if current_board.is_checkmate(bot_colour) {
            return i32::MIN;
        }
        
        if depth >= 2 && !current_board.is_check(bot_colour) {
            let mut null_move_board: Board = current_board.clone();
            null_move_board.swap_turn();
            let eval = minmax( &null_move_board, depth - 2, false, alpha, beta, bot_colour, start_time, max_time);
            if eval >= beta {
                return beta;
            }
        }
        
        let mut max_eval = i32::MIN;
        for mv in get_legal_moves(&current_board) {
            let mut new_board = current_board.clone();
            new_board.play_unsafe(mv);
            let eval = minmax(&new_board, depth - 1, false, alpha, beta, bot_colour, start_time, max_time);
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
            let eval = minmax(&new_board, depth - 1, true, alpha, beta, bot_colour, start_time, max_time);
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
fn quiesce(current_board: &Board, bot_colour: &Colour, is_bots_move: bool, mut alpha: i32, mut beta: i32, start_time: Instant, max_time: Duration) -> i32 {

    if start_time.elapsed() > max_time {
        return 0;
    }

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
            let score = quiesce(&new_board, bot_colour, false, alpha, beta, start_time, max_time);
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
            let score = quiesce(&new_board, bot_colour, true, alpha, beta, start_time, max_time);
            best_value = cmp::min(best_value, score);
            beta = cmp::min(beta,best_value);
            if alpha >= beta {
                break;
            }
        }

        return best_value;
    }
}

// Orders legal moves by decreasing evaluation
fn order_moves_by_evaluation(mut moves: ArrayVec<(Move, i32), 218>) -> ArrayVec<Move, 218> {
    moves.sort_by_key(|&(_,v)| Reverse(v));
    return moves.into_iter().map(|(k,_)| k.clone()).collect();
}

// Python module definition
#[pymodule]
fn chesslibrary(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pick_move, m)?)?;
    Ok(())
}