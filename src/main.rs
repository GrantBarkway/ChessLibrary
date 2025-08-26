#![allow(unused_imports)]

use arrayvec::ArrayVec;
use chesslibrary::bitboard::EMPTY_BITBOARD;
use chesslibrary::board::Board;
use chesslibrary::engine::search::{pick_move, NODE_COUNT};
//use chesslibrary::engine::eval::evaluate;
use chesslibrary::movegen::{get_legal_moves, get_white_moves};
use chesslibrary::bitboard::{Bitboard, bitboard_to_string_move};
//use chesslibrary::colour::{Colour};
use chesslibrary::perft::perft_test;
use chesslibrary::role::Role;
use chesslibrary::square::{Square, SECOND_RANK, SEVENTH_RANK, SIXTH_RANK, THIRD_RANK};
use chesslibrary::mv::Move;
use chesslibrary::uci::to_uci;
use chesslibrary::square::{FILE_A,FILE_B,FILE_C,FILE_D,FILE_E,FILE_F,FILE_G,FILE_H};

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    
    let board = Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8".to_string());
    
    let mut move_list = ArrayVec::<Move, 218>::new();
    for i in 0..999999 {
        get_white_moves(&board, &mut move_list);
        move_list.clear();
    }

    //eprintln!("Best move: {:?}", pick_move("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8".to_string(), 5000, "white".to_string()));
    
    /*
    // List of legal moves
    for i in get_legal_moves(&mut board) {
        println!("{:?} {} to {} (castle: {}, en_passant_target_square: {}, promotion: {:?}, capture: {:?})", i.role, bitboard_to_string_move(i.from_square), bitboard_to_string_move(i.to_square), i.castle, i.en_passant_target, i.promotion, i.capture);
    }    

    eprintln!("Boards evaluated: {:?}", perft_test(&mut board, 5));
    */

    let elapsed = now.elapsed();
    
    let boards_per_second = NODE_COUNT.load(std::sync::atomic::Ordering::Relaxed) * 1000000 / elapsed.as_micros() as usize;
    
    println!("Elapsed: {:.2?}", elapsed);
    //println!("Nodes evaluated per second: {:.2?}", boards_per_second);
    //println!("Nodes evaluated: {:?}", NODE_COUNT.load(std::sync::atomic::Ordering::Relaxed));
}