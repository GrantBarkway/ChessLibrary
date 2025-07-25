// Just while testing
#![allow(dead_code, unused_variables)]

use crate::board::{Board};
use crate::colour::{Colour};
use crate::bitboard::{Bitboard, EMPTY_BITBOARD};
use crate::mv::Move;
use crate::square::{EIGHTH_RANK, FILE_A, FILE_H, FIRST_RANK, SECOND_RANK, SEVENTH_RANK};
use crate::magic::{bishop_attacks, rook_attacks};

// H1, G1, F1, E1, D1, C1, B1, A1
// H2 ...
const KNIGHT_ATTACKS: [Bitboard; 64] = [
    Bitboard(0x0000000000020400), Bitboard(0x0000000000050800), Bitboard(0x00000000000a1100), Bitboard(0x0000000000142200), Bitboard(0x0000000000284400), Bitboard(0x0000000000508800), Bitboard(0x0000000000a01000), Bitboard(0x0000000000402000),
    Bitboard(0x0000000002040004), Bitboard(0x0000000005080008), Bitboard(0x000000000a110011), Bitboard(0x0000000014220022), Bitboard(0x0000000028440044), Bitboard(0x0000000050880088), Bitboard(0x00000000a0100010), Bitboard(0x0000000040200020),
    Bitboard(0x0000000204000402), Bitboard(0x0000000508000805), Bitboard(0x0000000a1100110a), Bitboard(0x0000001422002214), Bitboard(0x0000002844004428), Bitboard(0x0000005088008850), Bitboard(0x000000a0100010a0), Bitboard(0x0000004020002040),
    Bitboard(0x0000020400040200), Bitboard(0x0000050800080500), Bitboard(0x00000a1100110a00), Bitboard(0x0000142200221400), Bitboard(0x0000284400442800), Bitboard(0x0000508800885000), Bitboard(0x0000a0100010a000), Bitboard(0x0000402000204000),
    Bitboard(0x0002040004020000), Bitboard(0x0005080008050000), Bitboard(0x000a1100110a0000), Bitboard(0x0014220022140000), Bitboard(0x0028440044280000), Bitboard(0x0050880088500000), Bitboard(0x00a0100010a00000), Bitboard(0x0040200020400000),
    Bitboard(0x0204000402000000), Bitboard(0x0508000805000000), Bitboard(0x0a1100110a000000), Bitboard(0x1422002214000000), Bitboard(0x2844004428000000), Bitboard(0x5088008850000000), Bitboard(0xa0100010a0000000), Bitboard(0x4020002040000000),
    Bitboard(0x0400040200000000), Bitboard(0x0800080500000000), Bitboard(0x1100110a00000000), Bitboard(0x2200221400000000), Bitboard(0x4400442800000000), Bitboard(0x8800885000000000), Bitboard(0x100010a000000000), Bitboard(0x2000204000000000),
    Bitboard(0x0004020000000000), Bitboard(0x0008050000000000), Bitboard(0x00110a0000000000), Bitboard(0x0022140000000000), Bitboard(0x0044280000000000), Bitboard(0x0088500000000000), Bitboard(0x0010a00000000000), Bitboard(0x0020400000000000)
];

// Pawn shifts for different attacks 
const PAWN_FORWARD_SHIFT: i32 = 8;
const WHITE_PAWN_A_FILE_ATTACK: i32 = 7;
const WHITE_PAWN_H_FILE_ATTACK: i32 = 9;
const BLACK_PAWN_A_FILE_ATTACK: i32 = 9;
const BLACK_PAWN_H_FILE_ATTACK: i32 = 7;

// Generates a vector of legal moves for the side to move
pub fn get_legal_moves(board: &Board) -> Vec<Move> {
    match board.turn {
        Colour::White => return get_white_legal_moves(&board),
        Colour::Black => return get_black_legal_moves(&board)
    }
}

pub fn get_white_legal_moves(board: &Board) -> Vec<Move> {
    let mut legal_moves: Vec<Move> = Vec::new();
    legal_moves.extend(get_white_pawn_moves(&board));
    legal_moves.extend(get_white_knight_moves(&board));
    legal_moves.extend(get_white_bishop_moves(&board));
    legal_moves.extend(get_white_rook_moves(&board));
    legal_moves.extend(get_white_queen_moves(&board));
    legal_moves.extend(get_white_king_moves(&board));
    return legal_moves
}

pub fn get_black_legal_moves(board: &Board) -> Vec<Move> {
    let mut legal_moves: Vec<Move> = Vec::new();
    legal_moves.extend(get_black_pawn_moves(&board));
    legal_moves.extend(get_black_knight_moves(&board));
    legal_moves.extend(get_black_bishop_moves(&board));
    legal_moves.extend(get_black_rook_moves(&board));
    legal_moves.extend(get_black_queen_moves(&board));
    legal_moves.extend(get_black_king_moves(&board));
    return legal_moves
}

pub fn get_king_moves(board: &Board) -> Vec<Move> {
    let mut move_vector: Vec<Move> = Vec::new();
    let king_bitboard: Bitboard;
    let turn_colour: Bitboard;
    let mut move_bitboard: Bitboard = Bitboard(0);
    
    match board.turn {
        Colour::White => (king_bitboard, turn_colour) = (board.colour.white & board.role.king, board.colour.white),
        Colour::Black => (king_bitboard, turn_colour) = (board.colour.black & board.role.king, board.colour.black),
    }
    
    move_bitboard |= (king_bitboard & !FILE_A) << 1;
    move_bitboard |= (king_bitboard & !FILE_H) >> 1;
    move_bitboard |= (king_bitboard & !FIRST_RANK) >> 8;
    move_bitboard |= (king_bitboard & !EIGHTH_RANK) << 8;
    
    move_bitboard |= (king_bitboard & !(FILE_A|FIRST_RANK)) >> 7;
    move_bitboard |= (king_bitboard & !(FILE_A|EIGHTH_RANK)) << 9;
    move_bitboard |= (king_bitboard & !(FILE_H|FIRST_RANK)) >> 9;
    move_bitboard |= (king_bitboard & !(FILE_H|EIGHTH_RANK)) << 7;
    
    // Need to also add checks for if the square is attacked
    for single_move in move_bitboard.get_component_bitboards() {
        if (single_move & turn_colour).count_ones() == 0 {
            move_vector.push(Move::new(&board, &king_bitboard, &single_move, &EMPTY_BITBOARD, &false, None));
        }
    }
    
    return move_vector;
}

pub fn get_white_king_moves(board: &Board) -> Vec<Move> {
    let mut move_vector: Vec<Move> = Vec::new();
    let king_bitboard: Bitboard = board.colour.white & board.role.king;
    let turn_colour: Bitboard = board.colour.white;
    let mut move_bitboard: Bitboard = Bitboard(0);
    
    move_bitboard |= (king_bitboard & !FILE_A) << 1;
    move_bitboard |= (king_bitboard & !FILE_H) >> 1;
    move_bitboard |= (king_bitboard & !FIRST_RANK) >> 8;
    move_bitboard |= (king_bitboard & !EIGHTH_RANK) << 8;
    
    move_bitboard |= (king_bitboard & !(FILE_A|FIRST_RANK)) >> 7;
    move_bitboard |= (king_bitboard & !(FILE_A|EIGHTH_RANK)) << 9;
    move_bitboard |= (king_bitboard & !(FILE_H|FIRST_RANK)) >> 9;
    move_bitboard |= (king_bitboard & !(FILE_H|EIGHTH_RANK)) << 7;
    
    // Need to also add checks for if the square is attacked
    for single_move in move_bitboard.get_component_bitboards() {
        if (single_move & turn_colour).count_ones() == 0 {
            move_vector.push(Move::new(&board, &king_bitboard, &single_move, &EMPTY_BITBOARD, &false, None));
        }
    }
    
    return move_vector;
}

pub fn get_black_king_moves(board: &Board) -> Vec<Move> {
    let mut move_vector: Vec<Move> = Vec::new();
    let king_bitboard: Bitboard = board.colour.black & board.role.king;
    let turn_colour: Bitboard = board.colour.black;
    let mut move_bitboard: Bitboard = Bitboard(0);
    
    move_bitboard |= (king_bitboard & !FILE_A) << 1;
    move_bitboard |= (king_bitboard & !FILE_H) >> 1;
    move_bitboard |= (king_bitboard & !FIRST_RANK) >> 8;
    move_bitboard |= (king_bitboard & !EIGHTH_RANK) << 8;
    
    move_bitboard |= (king_bitboard & !(FILE_A|FIRST_RANK)) >> 7;
    move_bitboard |= (king_bitboard & !(FILE_A|EIGHTH_RANK)) << 9;
    move_bitboard |= (king_bitboard & !(FILE_H|FIRST_RANK)) >> 9;
    move_bitboard |= (king_bitboard & !(FILE_H|EIGHTH_RANK)) << 7;
    
    // Need to also add checks for if the square is attacked
    for single_move in move_bitboard.get_component_bitboards() {
        if (single_move & turn_colour).count_ones() == 0 {
            move_vector.push(Move::new(&board, &king_bitboard, &single_move, &EMPTY_BITBOARD, &false, None));
        }
    }
    
    return move_vector;
}

pub fn get_white_pawn_moves(board: &Board) -> Vec<Move> {
    let mut move_vector: Vec<Move> = Vec::new();
    let pawn_bitboard: Bitboard = board.colour.white & board.role.pawn;
    let opponent_colour: Bitboard = board.colour.black;
    let starting_rank: Bitboard = SECOND_RANK;
    
    for single_pawn in pawn_bitboard.get_component_bitboards() {
        let mut attack_moves = single_pawn << WHITE_PAWN_A_FILE_ATTACK;
        if (attack_moves & !FILE_A & opponent_colour) != EMPTY_BITBOARD {
            move_vector.push(Move::new(&board, &single_pawn, &attack_moves, &EMPTY_BITBOARD, &false, None));
        }
        attack_moves = single_pawn << WHITE_PAWN_H_FILE_ATTACK;
        if (attack_moves & !FILE_H & opponent_colour) != EMPTY_BITBOARD {
            move_vector.push(Move::new(&board, &single_pawn, &attack_moves, &EMPTY_BITBOARD, &false, None));
        }
        
        // Single and double pawn moves
        let one_forward = single_pawn << PAWN_FORWARD_SHIFT;
        let two_forward = one_forward << PAWN_FORWARD_SHIFT;
        if (one_forward & !board.occupied) != EMPTY_BITBOARD {
            move_vector.push(Move::new(&board, &single_pawn, &one_forward, &EMPTY_BITBOARD, &false, None));
            if ((single_pawn & starting_rank) != EMPTY_BITBOARD) & ((two_forward & !board.occupied) != EMPTY_BITBOARD) {
                move_vector.push(Move::new(&board, &single_pawn, &two_forward, &EMPTY_BITBOARD, &false, None));
            }
        }
    }
    
    return move_vector;
}

pub fn get_black_pawn_moves(board: &Board) -> Vec<Move> {
    let mut move_vector: Vec<Move> = Vec::new();
    let pawn_bitboard: Bitboard = board.colour.black & board.role.pawn;
    let opponent_colour: Bitboard = board.colour.white;
    let starting_rank: Bitboard = SEVENTH_RANK;
    
    for single_pawn in pawn_bitboard.get_component_bitboards() {
        let mut attack_moves = single_pawn >> BLACK_PAWN_A_FILE_ATTACK;
        if (attack_moves & !FILE_A & opponent_colour) != EMPTY_BITBOARD {
            move_vector.push(Move::new(&board, &single_pawn, &attack_moves, &EMPTY_BITBOARD, &false, None));
        }
        attack_moves = single_pawn >> BLACK_PAWN_H_FILE_ATTACK;
        if (attack_moves & !FILE_H & opponent_colour) != EMPTY_BITBOARD {
            move_vector.push(Move::new(&board, &single_pawn, &attack_moves, &EMPTY_BITBOARD, &false, None));
        }
        
        // Single and double pawn moves
        let one_forward = single_pawn >> PAWN_FORWARD_SHIFT;
        let two_forward = one_forward >> PAWN_FORWARD_SHIFT;
        if (one_forward & !board.occupied) != EMPTY_BITBOARD {
            move_vector.push(Move::new(&board, &single_pawn, &one_forward, &EMPTY_BITBOARD, &false, None));
            if ((single_pawn & starting_rank) != EMPTY_BITBOARD) & ((two_forward & !board.occupied) != EMPTY_BITBOARD) {
                move_vector.push(Move::new(&board, &single_pawn, &two_forward, &EMPTY_BITBOARD, &false, None));
            }
        }
    }
    
    return move_vector;
}

pub fn get_white_knight_moves(board: &Board) -> Vec<Move> {
    let mut move_vector: Vec<Move> = Vec::new();
    let knight_bitboard = board.colour.white & board.role.knight;
    let turn_colour: Bitboard = board.colour.white;
    
    for individual_knight in knight_bitboard.get_component_bitboards() {
        for knight_move in KNIGHT_ATTACKS[individual_knight.0.trailing_zeros() as usize].get_component_bitboards() {
            if (knight_move & turn_colour).count_ones() == 0 {
                move_vector.push(Move::new(&board, &individual_knight, &knight_move, &EMPTY_BITBOARD, &false, None));
            }
        }
    }
    
    return move_vector;
}

pub fn get_black_knight_moves(board: &Board) -> Vec<Move> {
    let mut move_vector: Vec<Move> = Vec::new();
    let knight_bitboard = board.colour.black & board.role.knight;
    let turn_colour: Bitboard = board.colour.black;
    
    for individual_knight in knight_bitboard.get_component_bitboards() {
        for knight_move in KNIGHT_ATTACKS[individual_knight.0.trailing_zeros() as usize].get_component_bitboards() {
            if (knight_move & turn_colour).count_ones() == 0 {
                move_vector.push(Move::new(&board, &individual_knight, &knight_move, &EMPTY_BITBOARD, &false, None));
            }
        }
    }
    
    return move_vector;
}

pub fn get_white_bishop_moves(board: &Board) -> Vec<Move> {
    let mut move_vector: Vec<Move> = Vec::new();
    let bishop_bitboard: Bitboard = board.colour.white & board.role.bishop;
    let turn_colour: Bitboard = board.colour.white;
    
    for individual_bishop in bishop_bitboard.get_component_bitboards() {
        for mv in bishop_attacks(individual_bishop, board.occupied).get_component_bitboards() {
            if (turn_colour & mv) == EMPTY_BITBOARD {
                move_vector.push(Move::new(&board, &individual_bishop, &mv, &EMPTY_BITBOARD, &false, None));
            }
        }
    }
    
    return move_vector;
}

pub fn get_black_bishop_moves(board: &Board) -> Vec<Move> {
    let mut move_vector: Vec<Move> = Vec::new();
    let bishop_bitboard: Bitboard = board.colour.black & board.role.bishop;
    let turn_colour: Bitboard = board.colour.black;
    
    for individual_bishop in bishop_bitboard.get_component_bitboards() {
        for mv in bishop_attacks(individual_bishop, board.occupied).get_component_bitboards() {
            if (turn_colour & mv) == EMPTY_BITBOARD {
                move_vector.push(Move::new(&board, &individual_bishop, &mv, &EMPTY_BITBOARD, &false, None));
            }
        }
    }
    
    return move_vector;
}

pub fn get_white_rook_moves(board: &Board) -> Vec<Move> {
    let mut move_vector: Vec<Move> = Vec::new();
    let rook_bitboard: Bitboard = board.colour.white & board.role.rook;
    let turn_colour: Bitboard = board.colour.white;
    
    for individual_rook in rook_bitboard.get_component_bitboards() {
        for mv in rook_attacks(individual_rook, board.occupied).get_component_bitboards() {
            if (turn_colour & mv) == EMPTY_BITBOARD {
                move_vector.push(Move::new(&board, &individual_rook, &mv, &EMPTY_BITBOARD, &false, None));
            }
        }
    }
    
    return move_vector;
}

pub fn get_black_rook_moves(board: &Board) -> Vec<Move> {
    let mut move_vector: Vec<Move> = Vec::new();
    let rook_bitboard: Bitboard = board.colour.black & board.role.rook;
    let turn_colour: Bitboard = board.colour.black;
    
    for individual_rook in rook_bitboard.get_component_bitboards() {
        for mv in rook_attacks(individual_rook, board.occupied).get_component_bitboards() {
            if (turn_colour & mv) == EMPTY_BITBOARD {
                move_vector.push(Move::new(&board, &individual_rook, &mv, &EMPTY_BITBOARD, &false, None));
            }
        }
    }
    
    return move_vector;
}

pub fn get_white_queen_moves(board: &Board) -> Vec<Move> {
    let mut move_vector: Vec<Move> = Vec::new();
    let queen_bitboard: Bitboard = board.colour.white & board.role.queen;
    let turn_colour: Bitboard = board.colour.white;
    
    for individual_queen in queen_bitboard.get_component_bitboards() {
        for mv in rook_attacks(individual_queen, board.occupied).get_component_bitboards() {
            if (turn_colour & mv) == EMPTY_BITBOARD {
                move_vector.push(Move::new(&board, &individual_queen, &mv, &EMPTY_BITBOARD, &false, None));
            }
        }
        
        for mv in bishop_attacks(individual_queen, board.occupied).get_component_bitboards() {
            if (turn_colour & mv) == EMPTY_BITBOARD {
                move_vector.push(Move::new(&board, &individual_queen, &mv, &EMPTY_BITBOARD, &false, None));
            }
        }
    }
    
    return move_vector;
}

pub fn get_black_queen_moves(board: &Board) -> Vec<Move> {
    let mut move_vector: Vec<Move> = Vec::new();
    let queen_bitboard: Bitboard = board.colour.black & board.role.queen;
    let turn_colour: Bitboard = board.colour.black;
    
    for individual_queen in queen_bitboard.get_component_bitboards() {
        for mv in rook_attacks(individual_queen, board.occupied).get_component_bitboards() {
            if (turn_colour & mv) == EMPTY_BITBOARD {
                move_vector.push(Move::new(&board, &individual_queen, &mv, &EMPTY_BITBOARD, &false, None));
            }
        }
        
        for mv in bishop_attacks(individual_queen, board.occupied).get_component_bitboards() {
            if (turn_colour & mv) == EMPTY_BITBOARD {
                move_vector.push(Move::new(&board, &individual_queen, &mv, &EMPTY_BITBOARD, &false, None));
            }
        }
    }
    
    return move_vector;
}