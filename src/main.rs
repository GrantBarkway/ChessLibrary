use chesslibrary::board::Board;
use chesslibrary::movegen::{get_legal_moves};
use chesslibrary::bitboard::{bitboard_to_string_move, Bitboard, EMPTY_BITBOARD};
use chesslibrary::mv::Move;
use chesslibrary::square::Square;

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let mut board = Board::new();
    board.play_unsafe(Move::new(&board, &Square::B2, &Square::B5, &EMPTY_BITBOARD, &false, &false, None));
    board.play_unsafe(Move::new(&board, &Square::C7, &Square::C5, &Bitboard(35184372088832), &false, &false, None));
    board.play_unsafe(Move::new(&board, &Square::B5, &Square::C6, &EMPTY_BITBOARD, &true, &false, None));
    board.display_board();
    for _i in 0..99999 {
        get_legal_moves(&mut board);
    }
    
    for i in get_legal_moves(&mut board) {
        println!("{:?} {} to {} (castle: {}, en_passant_target_square: {})", i.role, bitboard_to_string_move(i.from_square), bitboard_to_string_move(i.to_square), i.castle, i.en_passant_target);
    }
    
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}