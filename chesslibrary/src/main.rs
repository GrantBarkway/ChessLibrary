use chesslibrary::board::Board;
use chesslibrary::mv::Move;
use chesslibrary::square::Square;
use chesslibrary::movegen::{get_legal_moves};
use chesslibrary::bitboard::{bitboard_to_string_move};

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let board = Board::new();
    board.display_board();
    for _i in 0..999999 {
        get_legal_moves(&board);
    }
    for i in get_legal_moves(&board) {
        println!("{:?} {} to {}", i.role, bitboard_to_string_move(i.from_square), bitboard_to_string_move(i.to_square));
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}