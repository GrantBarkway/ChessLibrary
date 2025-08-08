use crate::board::Board;
use crate::colour::Colour;

pub fn evaluate(board: &Board, colour: &Colour) -> i32 {
    
    let white_pawns = board.role.pawn & board.colour.white;
    let white_knights = board.role.knight & board.colour.white;
    let white_bishops = board.role.bishop & board.colour.white;
    let white_rooks = board.role.rook & board.colour.white;
    let white_queens = board.role.queen & board.colour.white;

    let black_pawns = board.role.pawn & board.colour.black;
    let black_knights = board.role.knight & board.colour.black;
    let black_bishops = board.role.bishop & board.colour.black;
    let black_rooks = board.role.rook & board.colour.black;
    let black_queens = board.role.queen & board.colour.black;
    
    let mut evaluation: i32 = 0;
    
    evaluation += white_pawns.count_ones() as i32;
    evaluation += white_knights.count_ones() as i32 * 3;
    evaluation += white_bishops.count_ones() as i32 * 3;
    evaluation += white_rooks.count_ones() as i32 * 5;
    evaluation += white_queens.count_ones() as i32 * 9;
    
    evaluation -= black_pawns.count_ones() as i32;
    evaluation -= black_knights.count_ones() as i32 * 3;
    evaluation -= black_bishops.count_ones() as i32 * 3;
    evaluation -= black_rooks.count_ones() as i32 * 5;
    evaluation -= black_queens.count_ones() as i32 * 9;
    
    match colour {
        Colour::White => return evaluation,
        Colour::Black => return -evaluation,
    }
}