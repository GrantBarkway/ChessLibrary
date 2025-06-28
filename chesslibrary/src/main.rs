use chesslibrary::board::Board;
use chesslibrary::mv::Move;
use chesslibrary::square::Square;
use chesslibrary::movegen::get_legal_moves;

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let mut board = Board::new();
    let mov = Move::new(&board,&Square::E2, &Square::E4);
    board.make_move(mov);
    board.display_board();
    println!("Legal moves: {:?}", get_legal_moves(board));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}