use chesslibrary::board::Board;
use chesslibrary::engine::search::minmax;
//use chesslibrary::engine::eval::evaluate;
use chesslibrary::movegen::{get_legal_moves};
use chesslibrary::bitboard::{bitboard_to_string_move, EMPTY_BITBOARD};
use chesslibrary::mv::Move;
use chesslibrary::square::Square;

const TO_TEST: i128 = 1;

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let mut board = Board::new();
    board.display_board();
    for _i in 0..TO_TEST {
        //evaluate(&board, &board.turn);
        eprintln!("Evaluation: {}", minmax(&board, 8, true, i32::MIN, i32::MAX, &board.turn));
        //get_legal_moves(&mut board);
    }
    
    for i in get_legal_moves(&mut board) {
        println!("{:?} {} to {} (castle: {}, en_passant_target_square: {}, promotion: {:?}, capture: {:?})", i.role, bitboard_to_string_move(i.from_square), bitboard_to_string_move(i.to_square), i.castle, i.en_passant_target, i.promotion, i.capture);
    }
    
    let elapsed = now.elapsed();
    
    let boards_per_second = TO_TEST * 1000000 / elapsed.as_micros() as i128;
    
    println!("Elapsed: {:.2?}", elapsed);
    println!("Nodes per second: {:.2?}", boards_per_second);
}