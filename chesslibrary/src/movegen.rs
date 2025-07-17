// Just while testing
#![allow(dead_code, unused_variables)]

use crate::board::{Board};
use crate::colour::Colour;
use crate::bitboard::{Bitboard, EMPTY_BITBOARD};
use crate::mv::Move;
use crate::square::{ALL_SQUARES, EIGHTH_RANK, FILE_A, FILE_H, FIRST_RANK, SECOND_RANK, SEVENTH_RANK};

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

const ROOK_MASKS: [Bitboard; 64] = [
    Bitboard(72340172838076926), Bitboard(144680345676153597), Bitboard(289360691352306939), Bitboard(578721382704613623), Bitboard(1157442765409226991), Bitboard(2314885530818453727), Bitboard(4629771061636907199), Bitboard(9259542123273814143),
    Bitboard(72340172838141441), Bitboard(144680345676217602), Bitboard(289360691352369924), Bitboard(578721382704674568), Bitboard(1157442765409283856), Bitboard(2314885530818502432), Bitboard(4629771061636939584), Bitboard(9259542123273813888),
    Bitboard(72340172854657281), Bitboard(144680345692602882), Bitboard(289360691368494084), Bitboard(578721382720276488), Bitboard(1157442765423841296), Bitboard(2314885530830970912), Bitboard(4629771061645230144), Bitboard(9259542123273748608),
    Bitboard(72340177082712321), Bitboard(144680349887234562), Bitboard(289360695496279044), Bitboard(578721386714368008), Bitboard(1157442769150545936), Bitboard(2314885534022901792), Bitboard(4629771063767613504), Bitboard(9259542123257036928),
    Bitboard(72341259464802561), Bitboard(144681423712944642), Bitboard(289361752209228804), Bitboard(578722409201797128), Bitboard(1157443723186933776), Bitboard(2314886351157207072), Bitboard(4629771607097753664), Bitboard(9259542118978846848),
    Bitboard(72618349279904001), Bitboard(144956323094725122), Bitboard(289632270724367364), Bitboard(578984165983651848), Bitboard(1157687956502220816), Bitboard(2315095537539358752), Bitboard(4629910699613634624), Bitboard(9259541023762186368),
    Bitboard(143553341945872641), Bitboard(215330564830528002), Bitboard(358885010599838724), Bitboard(645993902138460168), Bitboard(1220211685215703056), Bitboard(2368647251370188832), Bitboard(4665518383679160384), Bitboard(9259260648297103488),
    Bitboard(18302911464433844481), Bitboard(18231136449196065282), Bitboard(18087586418720506884), Bitboard(17800486357769390088), Bitboard(17226286235867156496), Bitboard(16077885992062689312), Bitboard(13781085504453754944), Bitboard(9187484529235886208)
];

const ROOK_MAGICS: [u64; 64] = [
  0x2080020500400f0,
  0x28444000400010,
  0x20000a1004100014,
  0x20010c090202006,
  0x8408008200810004,
  0x1746000808002,
  0x2200098000808201,
  0x12c0002080200041,
  0x104000208e480804,
  0x8084014008281008,
  0x4200810910500410,
  0x100014481c20400c,
  0x4014a4040020808,
  0x401002001010a4,
  0x202000500010001,
  0x8112808005810081,
  0x40902108802020,
  0x42002101008101,
  0x459442200810c202,
  0x81001103309808,
  0x8110000080102,
  0x8812806008080404,
  0x104020000800101,
  0x40a1048000028201,
  0x4100ba0000004081,
  0x44803a4003400109,
  0xa010a00000030443,
  0x91021a000100409,
  0x4201e8040880a012,
  0x22a000440201802,
  0x30890a72000204,
  0x10411402a0c482,
  0x40004841102088,
  0x40230000100040,
  0x40100010000a0488,
  0x1410100200050844,
  0x100090808508411,
  0x1410040024001142,
  0x8840018001214002,
  0x410201000098001,
  0x8400802120088848,
  0x2060080000021004,
  0x82101002000d0022,
  0x1001101001008241,
  0x9040411808040102,
  0x600800480009042,
  0x1a020000040205,
  0x4200404040505199,
  0x2020081040080080,
  0x40a3002000544108,
  0x4501100800148402,
  0x81440280100224,
  0x88008000000804,
  0x8084060000002812,
  0x1840201000108312,
  0x5080202000000141,
  0x1042a180880281,
  0x900802900c01040,
  0x8205104104120,
  0x9004220000440a,
  0x8029510200708,
  0x8008440100404241,
  0x2420001111000bd,
  0x4000882304000041,
];

const ROOK_ATTACKS: [[Bitboard; 64]; 4096] = [[Bitboard(0); 64]; 4096];

pub const fn init_rook_attacks() {
    for single_square in ALL_SQUARES {
        
    }
}

// Gets rook masks for each square (includes edges)
pub fn get_rook_masks(square: Bitboard) -> Bitboard {
    let mut rook_masks = Bitboard(0);
    rook_masks |= square.get_rank();
    rook_masks |= square.get_file();
    rook_masks &= !square;
    return rook_masks;
}

// Generates a vector of legal moves
pub fn get_legal_moves(board: &Board, colour: Colour) -> Vec<Move> {
    let mut legal_moves: Vec<Move> = Vec::new();
    legal_moves.extend(get_king_moves(&board, &colour));
    legal_moves.extend(get_pawn_moves(&board, &colour));
    legal_moves.extend(get_knight_moves(&board, &colour));
    return legal_moves;
}

pub fn get_king_moves(board: &Board, colour: &Colour) -> Vec<Move> {
    let mut move_vector: Vec<Move> = Vec::with_capacity(8);
    let king_bitboard: Bitboard;
    let turn_colour: Bitboard;
    let mut move_bitboard: Bitboard = Bitboard(0);
    
    match colour {
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
            move_vector.push(Move::new(&board, &king_bitboard, &single_move));
        }
    }
    
    return move_vector;
}

pub fn get_pawn_moves(board: &Board, colour: &Colour) -> Vec<Move> {
    let mut move_vector: Vec<Move> = Vec::with_capacity(32);
    let pawn_bitboard: Bitboard;
    let opponent_colour: Bitboard;
    let starting_rank: Bitboard;
    let forward_shift: i32;
    let a_file_attack_shift: i32;
    let h_file_attack_shift: i32;
    
    match colour {
        Colour::White => (pawn_bitboard, opponent_colour, starting_rank, forward_shift, a_file_attack_shift, h_file_attack_shift) = (board.colour.white & board.role.pawn, board.colour.black, SECOND_RANK, 8, 7, 9),
        Colour::Black => (pawn_bitboard, opponent_colour, starting_rank, forward_shift, a_file_attack_shift, h_file_attack_shift) = (board.colour.black & board.role.pawn, board.colour.white, SEVENTH_RANK, -8, -7, -9),
    }
    
    for single_pawn in pawn_bitboard.get_component_bitboards() {
        
        let mut attack_moves = single_pawn << a_file_attack_shift;
        if (attack_moves & !FILE_A & opponent_colour) != EMPTY_BITBOARD {
            move_vector.push(Move::new(&board, &single_pawn, &attack_moves));
        }
        attack_moves = single_pawn << h_file_attack_shift;
        if (attack_moves & !FILE_H & opponent_colour) != EMPTY_BITBOARD {
            move_vector.push(Move::new(&board, &single_pawn, &attack_moves));
        }
        
        // Single and double pawn moves
        let one_forward = single_pawn << forward_shift;
        let two_forward = one_forward << forward_shift;
        if (one_forward & !board.occupied) != EMPTY_BITBOARD {
            move_vector.push(Move::new(&board, &single_pawn, &one_forward));
            if ((single_pawn & starting_rank) != EMPTY_BITBOARD) & ((two_forward & !board.occupied) != EMPTY_BITBOARD) {
                move_vector.push(Move::new(&board, &single_pawn, &two_forward));
            }
        }
    }

    return move_vector;
}

pub fn get_knight_moves(board: &Board, colour: &Colour) -> Vec<Move> {
    let mut move_vector: Vec<Move> = Vec::with_capacity(80);
    let knight_bitboard: Bitboard;
    let turn_colour: Bitboard;
    
    match colour {
        Colour::White => (knight_bitboard, turn_colour) = (board.colour.white & board.role.knight, board.colour.white),
        Colour::Black => (knight_bitboard, turn_colour) = (board.colour.black & board.role.knight, board.colour.black),
    }

    for individual_knight in knight_bitboard.get_component_bitboards() {
        for knight_move in KNIGHT_ATTACKS[individual_knight.0.trailing_zeros() as usize].get_component_bitboards() {
            if (knight_move & turn_colour).count_ones() == 0 {
                move_vector.push(Move::new(&board, &individual_knight, &knight_move));
            }
        }
    }

    return move_vector;
}

pub fn get_rook_moves(board: &Board, colour: &Colour) {
    
    let rook_bitboard: Bitboard;
    let turn_colour: Bitboard;

    match colour {
        Colour::White => (rook_bitboard, turn_colour) = (board.colour.white & board.role.rook, board.colour.white),
        Colour::Black => (rook_bitboard, turn_colour) = (board.colour.black & board.role.rook, board.colour.black),
    }

    for rook in rook_bitboard.get_component_bitboards() {
        let mut blockers: Bitboard = board.occupied;
        blockers &= ROOK_MASKS[rook.0.trailing_zeros() as usize];
        let u64_key = (blockers.0 * ROOK_MAGICS[0]);
    }
}