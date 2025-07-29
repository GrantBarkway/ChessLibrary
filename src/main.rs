use chesslibrary::board::Board;
use chesslibrary::movegen::{get_legal_moves};
use chesslibrary::bitboard::{Bitboard, bitboard_to_string_move};
use chesslibrary::mv::Move;
use chesslibrary::square::Square;

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let mut board = Board::new();
    let ka3 = Move::new(&board, &Square::E1, &Square::A3, &Bitboard(0), &false, None);
    board.play_unsafe(ka3);
    let ra6 = Move::new(&board, &Square::A8, &Square::A6, &Bitboard(0), &false, None);
    board.play_unsafe(ra6);
    board.display_board();
    for _i in 0..99999 {
        get_legal_moves(&mut board);
    }
    
    for i in get_legal_moves(&mut board) {
        println!("{:?} {} to {}", i.role, bitboard_to_string_move(i.from_square), bitboard_to_string_move(i.to_square));
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}