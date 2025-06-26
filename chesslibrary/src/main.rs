use chesslibrary::board::Board;
use chesslibrary::mv::Move;

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let mut board = Board::new();
    let mov = Move::new();
    board.make_move(mov);
    board.display_board();
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}