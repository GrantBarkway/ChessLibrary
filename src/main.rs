use chesslibrary::bitboard::EMPTY_BITBOARD;
use chesslibrary::board::Board;
use chesslibrary::engine::search::{NODE_COUNT};
//use chesslibrary::engine::eval::evaluate;
//use chesslibrary::movegen::{get_legal_moves};
//use chesslibrary::bitboard::{bitboard_to_string_move};
//use chesslibrary::colour::{Colour};
use chesslibrary::perft::perft_test;
use chesslibrary::square::Square;
use chesslibrary::mv::Move;

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    
    /*
    for i in 0..10 {
        if let Some(legal_moves) = get_legal_moves(&board).get(i) {
            board.play_unsafe(*legal_moves)
        }
    }*/
    
    let mut board = Board::starting_position();
    
    //board.play_unsafe(Move::new(&board, &Square::E2, &Square::E4, &EMPTY_BITBOARD, false, false, None));
    //board.unplay(board.last_move);

    //board.display_board();
    
    /*
    // Gets best move with evaluation
    let (best_move, evaluation) = pick_move(&board, 3, &Colour::White);
    if let Some(best_move) = best_move {
        eprintln!("Best Move: {} to {} with evaluation {}", bitboard_to_string_move(best_move.from_square), bitboard_to_string_move(best_move.to_square), evaluation);
    }
    
    // List of legal moves
    for i in get_legal_moves(&mut board) {
        println!("{:?} {} to {} (castle: {}, en_passant_target_square: {}, promotion: {:?}, capture: {:?})", i.role, bitboard_to_string_move(i.from_square), bitboard_to_string_move(i.to_square), i.castle, i.en_passant_target, i.promotion, i.capture);
    }
    */

    eprintln!("Boards evaluated: {:?}", perft_test(&mut board, 7));
    
    let elapsed = now.elapsed();
    
    let boards_per_second = NODE_COUNT.load(std::sync::atomic::Ordering::Relaxed) * 1000000 / elapsed.as_micros() as usize;
    
    println!("Elapsed: {:.2?}", elapsed);
    //println!("Nodes evaluated per second: {:.2?}", boards_per_second);
    //println!("Nodes evaluated: {:?}", NODE_COUNT.load(std::sync::atomic::Ordering::Relaxed));
}