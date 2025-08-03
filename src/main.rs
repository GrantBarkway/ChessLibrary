use chesslibrary::board::Board;
use chesslibrary::movegen::{get_legal_moves};
use chesslibrary::bitboard::{bitboard_to_string_move, Bitboard, EMPTY_BITBOARD};
use chesslibrary::mv::Move;
use chesslibrary::square::Square;
use chesslibrary::role::Role;

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let mut board = Board::new();
    board.display_board();
    for _i in 0..99999 {
        get_legal_moves(&mut board);
    }
    
    for i in get_legal_moves(&mut board) {
        println!("{:?} {} to {} (castle: {}, en_passant_target_square: {}, promotion: {:?})", i.role, bitboard_to_string_move(i.from_square), bitboard_to_string_move(i.to_square), i.castle, i.en_passant_target, i.promotion);
    }
    
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}