use crate::board::{Board};
use crate::castle::{BLACK_KINGSIDE_CASTLE_CHECK_SQUARES, BLACK_KINGSIDE_CASTLE_UNOCCUPIED_SQUARES, BLACK_QUEENSIDE_CASTLE_CHECK_SQUARES, BLACK_QUEENSIDE_CASTLE_UNOCCUPIED_SQUARES, WHITE_KINGSIDE_CASTLE_CHECK_SQUARES, WHITE_KINGSIDE_CASTLE_UNOCCUPIED_SQUARES, WHITE_QUEENSIDE_CASTLE_CHECK_SQUARES, WHITE_QUEENSIDE_CASTLE_UNOCCUPIED_SQUARES};
use crate::colour::{Colour};
use crate::bitboard::{Bitboard, EMPTY_BITBOARD};
use crate::mv::Move;
use crate::square::{Square, EIGHTH_RANK, FILE_A, FILE_H, FIRST_RANK, SECOND_RANK, SEVENTH_RANK};
use crate::magic::{bishop_attacks, rook_attacks};
use crate::role::Role;
use arrayvec::ArrayVec;

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
pub fn get_legal_moves(board: &Board) -> ArrayVec<Move, 218> {
    let mut move_vector = ArrayVec::<Move, 218>::new();
    
    match board.turn {
        Colour::White => get_white_moves(board, &mut move_vector),
        Colour::Black => get_black_moves(board, &mut move_vector),
    }
    
    let mut legal_move_vector = ArrayVec::<Move, 218>::new();
    
    for mv in move_vector {
        let mut board_copy = board.clone();
        board_copy.play_unsafe(mv);
        if board_copy.is_check(&board.turn) == false {
            legal_move_vector.push(mv);
        }
    }
    
    return legal_move_vector;
}

// Accepts and mutates an ArrayVec with a vector of all of whites legal moves
pub fn get_white_moves(board: &Board, move_list: &mut ArrayVec<Move, 218>) {
    get_white_pawn_moves(board, move_list);
    get_white_knight_moves(board, move_list);
    get_white_bishop_moves(board, move_list);
    get_white_rook_moves(board, move_list);
    get_white_queen_moves(board, move_list);
    get_white_king_moves(board, move_list);
}

// Accepts and mutates a bitboard of all of whites legal attacks
pub fn get_white_attacks(board: &Board) -> Bitboard {
    let mut attack_bitboard = Bitboard(0);
    attack_bitboard |= get_white_king_attacks(board);
    attack_bitboard |= get_white_pawn_attacks(board, &(board.colour.white & board.role.pawn));
    attack_bitboard |= get_knight_attacks(board, &(board.colour.white & board.role.knight));
    attack_bitboard |= get_bishop_attacks(board, &(board.colour.white & board.role.bishop));
    attack_bitboard |= get_rook_attacks(board, &(board.colour.white & board.role.rook));
    attack_bitboard |= get_queen_attacks(board, &(board.colour.white & board.role.queen));
    return attack_bitboard;
}

// Accepts and mutates an ArrayVec with a vector of all of blacks legal moves
pub fn get_black_moves(board: &Board, move_list: &mut ArrayVec<Move, 218>) {
    get_black_pawn_moves(board, move_list);
    get_black_knight_moves(board, move_list);
    get_black_bishop_moves(board, move_list);
    get_black_rook_moves(board, move_list);
    get_black_queen_moves(board, move_list);
    get_black_king_moves(board, move_list);
}

// Accepts and mutates a bitboard of all of blacks legal attacks
pub fn get_black_attacks(board: &Board) -> Bitboard {
    let mut attack_bitboard = Bitboard(0);
    attack_bitboard |= get_black_king_attacks(board);
    attack_bitboard |= get_black_pawn_attacks(board, &(board.colour.black & board.role.pawn));
    attack_bitboard |= get_knight_attacks(board, &(board.colour.black & board.role.knight));
    attack_bitboard |= get_bishop_attacks(board, &(board.colour.black & board.role.bishop));
    attack_bitboard |= get_rook_attacks(board, &(board.colour.black & board.role.rook));
    attack_bitboard |= get_queen_attacks(board, &(board.colour.black & board.role.queen));
    return attack_bitboard;
}

// Accepts and mutates an ArrayVec with all of whites legal king moves
pub fn get_white_king_moves(board: &Board, move_vector: &mut ArrayVec<Move, 218>) {
    let king_bitboard: Bitboard = board.colour.white & board.role.king;
    let turn_colour: Bitboard = board.colour.white;
    
    let white_king_attacks = get_white_king_attacks(board);
    for single_move in white_king_attacks.get_component_bitboards() {
        if (single_move & turn_colour).count_ones() == 0 {
            move_vector.push(Move::new(&board, &king_bitboard, &single_move, &EMPTY_BITBOARD, false, false, None));
        }
    }
    
    let black_attack_bitboard = get_black_attacks(board);
    if (board.castling_rights.white.kingside == true) & (((WHITE_KINGSIDE_CASTLE_UNOCCUPIED_SQUARES & board.occupied) == EMPTY_BITBOARD) & ((WHITE_KINGSIDE_CASTLE_CHECK_SQUARES & black_attack_bitboard) == EMPTY_BITBOARD)) {
        move_vector.push(Move::new(&board, &king_bitboard, &Square::G1, &EMPTY_BITBOARD, false, true, None))
    }
    
    if (board.castling_rights.white.queenside == true) & (((WHITE_QUEENSIDE_CASTLE_UNOCCUPIED_SQUARES & board.occupied) == EMPTY_BITBOARD) & ((WHITE_QUEENSIDE_CASTLE_CHECK_SQUARES & black_attack_bitboard) == EMPTY_BITBOARD)) {
        move_vector.push(Move::new(&board, &king_bitboard, &Square::C1, &EMPTY_BITBOARD, false, true, None))
    }
}

// Accepts and mutates a Bitboard with all of whites king attacks
pub fn get_white_king_attacks(board: &Board) -> Bitboard {
    let mut move_bitboard = Bitboard(0);
    let king_bitboard: Bitboard = board.colour.white & board.role.king;
    
    move_bitboard |= (king_bitboard & !FILE_A) << 1;
    move_bitboard |= (king_bitboard & !FILE_H) >> 1;
    move_bitboard |= (king_bitboard & !FIRST_RANK) >> 8;
    move_bitboard |= (king_bitboard & !EIGHTH_RANK) << 8;
    
    move_bitboard |= (king_bitboard & !(FILE_A|FIRST_RANK)) >> 7;
    move_bitboard |= (king_bitboard & !(FILE_A|EIGHTH_RANK)) << 9;
    move_bitboard |= (king_bitboard & !(FILE_H|FIRST_RANK)) >> 9;
    move_bitboard |= (king_bitboard & !(FILE_H|EIGHTH_RANK)) << 7;

    return move_bitboard;
}

// Accepts and mutates an ArrayVec with all of blacks legal king moves
pub fn get_black_king_moves(board: &Board, move_vector: &mut ArrayVec<Move, 218>) {
    let king_bitboard: Bitboard = board.colour.black & board.role.king;
    let turn_colour: Bitboard = board.colour.black;
    
    let black_king_attacks = get_black_king_attacks(board);
    for single_move in black_king_attacks.get_component_bitboards() {
        if (single_move & turn_colour).count_ones() == 0 {
            move_vector.push(Move::new(&board, &king_bitboard, &single_move, &EMPTY_BITBOARD, false, false, None));
        }
    }
    
    let white_attack_bitboard = get_white_attacks(board);
    if (board.castling_rights.black.kingside == true) & (((BLACK_KINGSIDE_CASTLE_UNOCCUPIED_SQUARES & board.occupied) == EMPTY_BITBOARD) & ((BLACK_KINGSIDE_CASTLE_CHECK_SQUARES & white_attack_bitboard) == EMPTY_BITBOARD)) {
        move_vector.push(Move::new(&board, &king_bitboard, &Square::G8, &EMPTY_BITBOARD, false, true, None))
    }
    
    if (board.castling_rights.black.queenside == true) & (((BLACK_QUEENSIDE_CASTLE_UNOCCUPIED_SQUARES & board.occupied) == EMPTY_BITBOARD) & ((BLACK_QUEENSIDE_CASTLE_CHECK_SQUARES & white_attack_bitboard) == EMPTY_BITBOARD)) {
        move_vector.push(Move::new(&board, &king_bitboard, &Square::C1, &EMPTY_BITBOARD, false, true, None))
    }
}

// Accepts and mutates a Bitboard with all of blacks king attacks
pub fn get_black_king_attacks(board: &Board) -> Bitboard {
    let mut move_bitboard = Bitboard(0);
    let king_bitboard: Bitboard = board.colour.black & board.role.king;
    
    move_bitboard |= (king_bitboard & !FILE_A) << 1;
    move_bitboard |= (king_bitboard & !FILE_H) >> 1;
    move_bitboard |= (king_bitboard & !FIRST_RANK) >> 8;
    move_bitboard |= (king_bitboard & !EIGHTH_RANK) << 8;
    
    move_bitboard |= (king_bitboard & !(FILE_A|FIRST_RANK)) >> 7;
    move_bitboard |= (king_bitboard & !(FILE_A|EIGHTH_RANK)) << 9;
    move_bitboard |= (king_bitboard & !(FILE_H|FIRST_RANK)) >> 9;
    move_bitboard |= (king_bitboard & !(FILE_H|EIGHTH_RANK)) << 7;

    return move_bitboard;
}

// Accepts and mutates an ArrayVec with all of whites legal pawn moves
pub fn get_white_pawn_moves(board: &Board, move_vector: &mut ArrayVec<Move,218>) {
    let pawn_bitboard: Bitboard = board.colour.white & board.role.pawn;
    let opponent_colour: Bitboard = board.colour.black;
    let mut en_passant_target_square: Option<Bitboard> = None;
    
    if let Some(last_move) = board.last_move {
        if last_move.en_passant_target != EMPTY_BITBOARD {
            en_passant_target_square = Some(last_move.en_passant_target);
        }
    }
    
    for single_pawn in pawn_bitboard.get_component_bitboards() {
        
        let a_file_attack_move = single_pawn << WHITE_PAWN_A_FILE_ATTACK;
        if (a_file_attack_move & !FILE_A & opponent_colour) != EMPTY_BITBOARD {
            move_vector.push(Move::new(&board, &single_pawn, &a_file_attack_move, &EMPTY_BITBOARD, false, false, None));
        }
        
        let h_file_attack_move = single_pawn << WHITE_PAWN_H_FILE_ATTACK;
        if (h_file_attack_move & !FILE_H & opponent_colour) != EMPTY_BITBOARD {
            move_vector.push(Move::new(&board, &single_pawn, &h_file_attack_move, &EMPTY_BITBOARD, false, false, None));
        }
        
        if let Some(ep_target) = en_passant_target_square {
            
            if (a_file_attack_move & !FILE_A & ep_target) != EMPTY_BITBOARD {
                move_vector.push(Move::new(&board, &single_pawn, &a_file_attack_move, &EMPTY_BITBOARD, true, false, None));
            }
            
            if (h_file_attack_move & !FILE_H & ep_target) != EMPTY_BITBOARD {
                move_vector.push(Move::new(&board, &single_pawn, &h_file_attack_move, &EMPTY_BITBOARD, true, false, None));
            }
        
        }
        
        // Single and double pawn moves
        let one_forward = single_pawn << PAWN_FORWARD_SHIFT;
        let two_forward = one_forward << PAWN_FORWARD_SHIFT;
        if (one_forward & !board.occupied) != EMPTY_BITBOARD {
            move_vector.push(Move::new(&board, &single_pawn, &one_forward, &EMPTY_BITBOARD, false, false, None));
            if (one_forward & EIGHTH_RANK) != EMPTY_BITBOARD {
                move_vector.push(Move::new(&board, &single_pawn, &one_forward, &EMPTY_BITBOARD, false, false, Some(Role::Queen)));
                move_vector.push(Move::new(&board, &single_pawn, &one_forward, &EMPTY_BITBOARD, false, false, Some(Role::Rook)));
                move_vector.push(Move::new(&board, &single_pawn, &one_forward, &EMPTY_BITBOARD, false, false, Some(Role::Bishop)));
                move_vector.push(Move::new(&board, &single_pawn, &one_forward, &EMPTY_BITBOARD, false, false, Some(Role::Knight)));
            }
            if ((single_pawn & SECOND_RANK) != EMPTY_BITBOARD) & ((two_forward & !board.occupied) != EMPTY_BITBOARD) {
                move_vector.push(Move::new(&board, &single_pawn, &two_forward, &one_forward, false, false, None));
            }
        }
    }
}

// Accepts and mutates a Bitboard with all of whites pawn attacks
pub fn get_white_pawn_attacks(_board: &Board, pawn_bitboard: &Bitboard) -> Bitboard {
    let mut pawn_attack_bitboard = Bitboard(0);
    
    for individual_pawn in pawn_bitboard.get_component_bitboards() {
        let mut attack_moves = individual_pawn << WHITE_PAWN_A_FILE_ATTACK;
        if (attack_moves & !FILE_A) != EMPTY_BITBOARD {
            pawn_attack_bitboard |= attack_moves;
        }
        
        attack_moves = individual_pawn << WHITE_PAWN_H_FILE_ATTACK;
        if (attack_moves & !FILE_H) != EMPTY_BITBOARD {
            pawn_attack_bitboard |= attack_moves;
        }
    }

    return pawn_attack_bitboard;
}

// Accepts and mutates an ArrayVec with all of blacks legal pawn moves
pub fn get_black_pawn_moves(board: &Board, move_vector: &mut ArrayVec<Move, 218>) {
    let pawn_bitboard: Bitboard = board.colour.black & board.role.pawn;
    let opponent_colour: Bitboard = board.colour.white;
    let mut en_passant_target_square: Option<Bitboard> = None;
    
    if let Some(last_move) = board.last_move {
        if last_move.en_passant_target != EMPTY_BITBOARD {
            en_passant_target_square = Some(last_move.en_passant_target);
        }
    }
    
    for single_pawn in pawn_bitboard.get_component_bitboards() {

        let a_file_attack_move = single_pawn >> BLACK_PAWN_A_FILE_ATTACK;
        if (a_file_attack_move & !FILE_A & opponent_colour) != EMPTY_BITBOARD {
            move_vector.push(Move::new(&board, &single_pawn, &a_file_attack_move, &EMPTY_BITBOARD, false, false, None));
        }
        
        let h_file_attack_move = single_pawn >> BLACK_PAWN_H_FILE_ATTACK;
        if (h_file_attack_move & !FILE_H & opponent_colour) != EMPTY_BITBOARD {
            move_vector.push(Move::new(&board, &single_pawn, &h_file_attack_move, &EMPTY_BITBOARD, false, false, None));
        }

        if let Some(ep_target) = en_passant_target_square {
            
            if (a_file_attack_move & !FILE_A & ep_target) != EMPTY_BITBOARD {
                move_vector.push(Move::new(&board, &single_pawn, &a_file_attack_move, &EMPTY_BITBOARD, true, false, None));
            }
            
            if (h_file_attack_move & !FILE_H & ep_target) != EMPTY_BITBOARD {
                move_vector.push(Move::new(&board, &single_pawn, &h_file_attack_move, &EMPTY_BITBOARD, true, false, None));
            }
        
        }
        
        // Single and double pawn moves
        let one_forward = single_pawn >> PAWN_FORWARD_SHIFT;
        let two_forward = one_forward >> PAWN_FORWARD_SHIFT;
        if (one_forward & !board.occupied) != EMPTY_BITBOARD {
            move_vector.push(Move::new(&board, &single_pawn, &one_forward, &EMPTY_BITBOARD, false, false, None));
            if (one_forward & FIRST_RANK) != EMPTY_BITBOARD {
                move_vector.push(Move::new(&board, &single_pawn, &one_forward, &EMPTY_BITBOARD, false, false, Some(Role::Queen)));
                move_vector.push(Move::new(&board, &single_pawn, &one_forward, &EMPTY_BITBOARD, false, false, Some(Role::Rook)));
                move_vector.push(Move::new(&board, &single_pawn, &one_forward, &EMPTY_BITBOARD, false, false, Some(Role::Bishop)));
                move_vector.push(Move::new(&board, &single_pawn, &one_forward, &EMPTY_BITBOARD, false, false, Some(Role::Knight)));
            }
            if ((single_pawn & SEVENTH_RANK) != EMPTY_BITBOARD) & ((two_forward & !board.occupied) != EMPTY_BITBOARD) {
                move_vector.push(Move::new(&board, &single_pawn, &two_forward, &one_forward, false, false, None));
            }
        }
    }
}

pub fn get_black_pawn_attacks(_board: &Board, pawn_bitboard: &Bitboard) -> Bitboard {
    let mut pawn_attack_bitboard = Bitboard(0);
    
    for individual_pawn in pawn_bitboard.get_component_bitboards() {
        let mut attack_moves = individual_pawn >> BLACK_PAWN_A_FILE_ATTACK;
        if (attack_moves & !FILE_A) != EMPTY_BITBOARD {
            pawn_attack_bitboard |= attack_moves;
        }
        
        attack_moves = individual_pawn >> BLACK_PAWN_H_FILE_ATTACK;
        if (attack_moves & !FILE_H) != EMPTY_BITBOARD {
            pawn_attack_bitboard |= attack_moves;
        }
    }

    return pawn_attack_bitboard;
}

pub fn get_white_knight_moves(board: &Board, move_vector: &mut ArrayVec<Move, 218>) {
    let knight_bitboard = board.colour.white & board.role.knight;
    let turn_colour: Bitboard = board.colour.white;
    
    for individual_knight in knight_bitboard.get_component_bitboards() {
        for knight_move in KNIGHT_ATTACKS[individual_knight.0.trailing_zeros() as usize].get_component_bitboards() {
            if (knight_move & turn_colour).count_ones() == 0 {
                move_vector.push(Move::new(&board, &individual_knight, &knight_move, &EMPTY_BITBOARD, false, false, None));
            }
        }
    }
}

pub fn get_black_knight_moves(board: &Board, move_vector: &mut ArrayVec<Move, 218>) {
    let knight_bitboard = board.colour.black & board.role.knight;
    let turn_colour: Bitboard = board.colour.black;
    
    for individual_knight in knight_bitboard.get_component_bitboards() {
        for knight_move in KNIGHT_ATTACKS[individual_knight.0.trailing_zeros() as usize].get_component_bitboards() {
            if (knight_move & turn_colour).count_ones() == 0 {
                move_vector.push(Move::new(&board, &individual_knight, &knight_move, &EMPTY_BITBOARD, false, false, None));
            }
        }
    }
}

pub fn get_knight_attacks(_board: &Board, knight_bitboard: &Bitboard) -> Bitboard {
    let mut knight_attack_bitboard = Bitboard(0);

    for individual_knight in knight_bitboard.get_component_bitboards() {
        knight_attack_bitboard |= KNIGHT_ATTACKS[individual_knight.0.trailing_zeros() as usize];
    }

    return knight_attack_bitboard;
} 

pub fn get_white_bishop_moves(board: &Board, move_vector: &mut ArrayVec<Move, 218>) {
    let bishop_bitboard: Bitboard = board.colour.white & board.role.bishop;
    let turn_colour: Bitboard = board.colour.white;
    
    for individual_bishop in bishop_bitboard.get_component_bitboards() {
        let bishop_attacks = bishop_attacks(&individual_bishop, &board.occupied);
        for mv in bishop_attacks.get_component_bitboards() {
            if (turn_colour & mv) == EMPTY_BITBOARD {
                move_vector.push(Move::new(&board, &individual_bishop, &mv, &EMPTY_BITBOARD, false, false, None));
            }
        }
    }
}

pub fn get_black_bishop_moves(board: &Board, move_vector: &mut ArrayVec<Move, 218>) {
    let bishop_bitboard: Bitboard = board.colour.black & board.role.bishop;
    let turn_colour: Bitboard = board.colour.black;

    for individual_bishop in bishop_bitboard.get_component_bitboards() {
        let bishop_attacks = bishop_attacks(&individual_bishop, &board.occupied);
        for mv in bishop_attacks.get_component_bitboards() {
            if (turn_colour & mv) == EMPTY_BITBOARD {
                move_vector.push(Move::new(&board, &individual_bishop, &mv, &EMPTY_BITBOARD, false, false, None));
            }
        }
    }
}

pub fn get_bishop_attacks(board: &Board, bishop_bitboard: &Bitboard) -> Bitboard {
    let mut bishop_attack_bitboard = Bitboard(0);
    
    for individual_bishop in bishop_bitboard.get_component_bitboards() {
        bishop_attack_bitboard |= bishop_attacks(&individual_bishop, &board.occupied);
    }

    return bishop_attack_bitboard;
}

pub fn get_white_rook_moves(board: &Board, move_vector: &mut ArrayVec<Move, 218>) {
    let rook_bitboard: Bitboard = board.colour.white & board.role.rook;
    let turn_colour: Bitboard = board.colour.white;

    for individual_rook in rook_bitboard.get_component_bitboards() {
        let rook_attacks = rook_attacks(&individual_rook, &board.occupied);
        for mv in rook_attacks.get_component_bitboards() {
            if (turn_colour & mv) == EMPTY_BITBOARD {
                move_vector.push(Move::new(&board, &individual_rook, &mv, &EMPTY_BITBOARD, false, false, None));
            }
        }
    }
}

pub fn get_black_rook_moves(board: &Board, move_vector: &mut ArrayVec<Move, 218>) {
    let rook_bitboard: Bitboard = board.colour.black & board.role.rook;
    let turn_colour: Bitboard = board.colour.black;
    
    for individual_rook in rook_bitboard.get_component_bitboards() {
        let rook_attacks = rook_attacks(&individual_rook, &board.occupied);
        for mv in rook_attacks.get_component_bitboards() {
            if (turn_colour & mv) == EMPTY_BITBOARD {
                move_vector.push(Move::new(&board, &individual_rook, &mv, &EMPTY_BITBOARD, false, false, None));
            }
        }
    }
}

pub fn get_rook_attacks(board: &Board, rook_bitboard: &Bitboard) -> Bitboard {
    let mut rook_attack_bitboard = Bitboard(0);

    for individual_rook in rook_bitboard.get_component_bitboards() {
        rook_attack_bitboard |= rook_attacks(&individual_rook, &board.occupied);
    }

    return rook_attack_bitboard;
}

pub fn get_white_queen_moves(board: &Board, move_vector: &mut ArrayVec<Move, 218>) {
    let queen_bitboard: Bitboard = board.colour.white & board.role.queen;
    let turn_colour: Bitboard = board.colour.white;
    
    for individual_queen in queen_bitboard.get_component_bitboards() {
        let rook_attacks = rook_attacks(&individual_queen, &board.occupied);
        for mv in rook_attacks.get_component_bitboards() {
            if (turn_colour & mv) == EMPTY_BITBOARD {
                move_vector.push(Move::new(&board, &individual_queen, &mv, &EMPTY_BITBOARD, false, false, None));
            }
        }
        
        let bishop_attacks = bishop_attacks(&individual_queen, &board.occupied);
        for mv in bishop_attacks.get_component_bitboards() {
            if (turn_colour & mv) == EMPTY_BITBOARD {
                move_vector.push(Move::new(&board, &individual_queen, &mv, &EMPTY_BITBOARD, false, false, None));
            }
        }
    }
}

pub fn get_black_queen_moves(board: &Board, move_vector: &mut ArrayVec<Move,218>) {
    let queen_bitboard: Bitboard = board.colour.black & board.role.queen;
    let turn_colour: Bitboard = board.colour.black;
    
    for individual_queen in queen_bitboard.get_component_bitboards() {
        let rook_attacks = rook_attacks(&individual_queen, &board.occupied);
        for mv in rook_attacks.get_component_bitboards() {
            if (turn_colour & mv) == EMPTY_BITBOARD {
                move_vector.push(Move::new(&board, &individual_queen, &mv, &EMPTY_BITBOARD, false, false, None));
            }
        }
        
        let bishop_attacks = bishop_attacks(&individual_queen, &board.occupied);
        for mv in bishop_attacks.get_component_bitboards() {
            if (turn_colour & mv) == EMPTY_BITBOARD {
                move_vector.push(Move::new(&board, &individual_queen, &mv, &EMPTY_BITBOARD, false, false, None));
            }
        }
    }
}

pub fn get_queen_attacks(board: &Board, queen_bitboard: &Bitboard) -> Bitboard {
    let mut queen_attack_bitboard = Bitboard(0);

    for individual_queen in queen_bitboard.get_component_bitboards() {
        queen_attack_bitboard |= rook_attacks(&individual_queen, &board.occupied);
        queen_attack_bitboard |= bishop_attacks(&individual_queen, &board.occupied);
    }

    return queen_attack_bitboard
}