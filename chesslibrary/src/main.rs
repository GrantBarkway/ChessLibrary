use chesslibrary::board::Board;
use chesslibrary::mv::Move;
use chesslibrary::square::Square;
use chesslibrary::movegen::get_legal_moves;
use chesslibrary::colour::Colour;

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let mut board = Board::new();
    let mov = Move::new(&board,&Square::E2, &Square::E4);
    board.make_move(mov);
    let king_e2 = Move::new(&board, &Square::E1, &Square::E2);
    board.make_move(king_e2);
    board.display_board();
    println!("Legal moves: {:?}", get_legal_moves(board, Colour::White));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}