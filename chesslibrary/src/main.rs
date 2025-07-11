use chesslibrary::board::Board;
use chesslibrary::mv::Move;
use chesslibrary::square::Square;
use chesslibrary::movegen::get_legal_moves;
use chesslibrary::colour::Colour;
use chesslibrary::bitboard::bitboard_to_string_move;

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let mut board = Board::new();
    /*let mov = Move::new(&board,&Square::E2, &Square::E4);
    board.make_move(mov);
    let pawn_a3 = Move::new(&board, &Square::A7, &Square::A3);
    board.make_move(pawn_a3);
    board.display_board();*/
    for i in 0..10000000 {
        get_legal_moves(&board, Colour::White);
    }
    /*for i in get_legal_moves(&board, Colour::White) {
        println!("{:?} {} to {}", i.role, bitboard_to_string_move(i.from_square), bitboard_to_string_move(i.to_square));
    }*/
    //println!("Legal moves: {:?}", get_legal_moves(board, Colour::White));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}