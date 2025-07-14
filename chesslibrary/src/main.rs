use chesslibrary::board::Board;
//use chesslibrary::mv::Move;
//use chesslibrary::square::Square;
use chesslibrary::movegen::{get_legal_moves, get_rook_masks};
use chesslibrary::colour::Colour;
use chesslibrary::bitboard::{Bitboard, bitboard_to_string_move};

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let board = Board::new();
    /*let mov = Move::new(&board,&Square::E2, &Square::E4);
    board.make_move(mov);
    let pawn_a3 = Move::new(&board, &Square::A7, &Square::A3);
    board.make_move(pawn_a3);
    board.display_board();*/
    let mut first_square = Bitboard(0b1);
    for i in (1..65).rev() {
        eprintln!("{:?}", get_rook_masks(first_square));
        first_square = first_square << 1;
    }
    
    for _i in 0..10000000 {
        get_legal_moves(&board, Colour::White);
    }
    for i in get_legal_moves(&board, Colour::White) {
        println!("{:?} {} to {}", i.role, bitboard_to_string_move(i.from_square), bitboard_to_string_move(i.to_square));
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}