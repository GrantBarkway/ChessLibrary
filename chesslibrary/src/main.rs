use chesslibrary::board::Board;

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let board = Board::new();
    board.display_board();
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}