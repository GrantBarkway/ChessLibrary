use crate::mv::Move;
use crate::board::Board;
use crate::bitboard::{Bitboard, EMPTY_BITBOARD};
use crate::colour::{Colour, get_colour};
use crate::square::{EIGHTH_RANK, FIFTH_RANK, FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H, FIRST_RANK, FOURTH_RANK, SECOND_RANK, SEVENTH_RANK, SIXTH_RANK, THIRD_RANK};
use crate::role::{Role, get_role};

pub fn to_uci(mv: Option<Move>) -> String {
    let mut uci_string = "".to_string();

    if let Some(mv) = mv {
        
        uci_string.push_str(&get_square_string(mv.from_square));
        uci_string.push_str(&get_square_string(mv.to_square));
        
        if !mv.castle {
            if let Some(promotion) = mv.promotion {
                match promotion {
                    Role::Knight => uci_string.push_str("n"),
                    Role::Bishop => uci_string.push_str("b"),
                    Role::Rook => uci_string.push_str("r"),
                    Role::Queen => uci_string.push_str("q"),
                    _ => (),
                }
            }
        }
    }
    
    return uci_string;
}

pub fn get_square_string(sq: Bitboard) -> String {
    let mut square_string: String = "".to_string();

    if sq & FILE_A != EMPTY_BITBOARD {
        square_string.push_str("a");
    } else if sq & FILE_B != EMPTY_BITBOARD {
        square_string.push_str("b");
    } else if sq & FILE_C != EMPTY_BITBOARD {
        square_string.push_str("c");
    } else if sq & FILE_D != EMPTY_BITBOARD {
        square_string.push_str("d");
    } else if sq & FILE_E != EMPTY_BITBOARD {
        square_string.push_str("e");
    } else if sq & FILE_F != EMPTY_BITBOARD {
        square_string.push_str("f");
    } else if sq & FILE_G != EMPTY_BITBOARD {
        square_string.push_str("g");
    } else if sq & FILE_H != EMPTY_BITBOARD {
        square_string.push_str("h");
    }

    if sq & FIRST_RANK != EMPTY_BITBOARD {
        square_string.push_str("1");
    } else if sq & SECOND_RANK != EMPTY_BITBOARD {
        square_string.push_str("2");
    } else if sq & THIRD_RANK != EMPTY_BITBOARD {
        square_string.push_str("3");
    } else if sq & FOURTH_RANK != EMPTY_BITBOARD {
        square_string.push_str("4");
    } else if sq & FIFTH_RANK != EMPTY_BITBOARD {
        square_string.push_str("5");
    } else if sq & SIXTH_RANK != EMPTY_BITBOARD {
        square_string.push_str("6");
    } else if sq & SEVENTH_RANK != EMPTY_BITBOARD {
        square_string.push_str("7");
    } else if sq & EIGHTH_RANK != EMPTY_BITBOARD {
        square_string.push_str("8");
    }
    
    return square_string;
}

pub fn get_uci_square_bitboard(uci_move: &str) -> Bitboard {
    let mut file = EMPTY_BITBOARD;
    let mut rank = EMPTY_BITBOARD;
    for i in uci_move.chars() {
        
        if file == EMPTY_BITBOARD {
            if i == 'a' {
                file = FILE_A;
            } else if i == 'b' {
                file = FILE_B;
            } else if i == 'c' {
                file = FILE_C;
            } else if i == 'd' {
                file = FILE_D;
            } else if i == 'e' {
                file = FILE_E;
            } else if i == 'f' {
                file = FILE_F;
            } else if i == 'g' {
                file = FILE_G;
            } else if i == 'h' {
                file = FILE_H;
            }
        }
        
        if rank == EMPTY_BITBOARD {
            if i == '1' {
                rank = FIRST_RANK;
            } else if i == '2' {
                rank = SECOND_RANK;
            } else if i == '3' {
                rank = THIRD_RANK;
            } else if i == '4' {
                rank = FOURTH_RANK;
            } else if i == '5' {
                rank = FIFTH_RANK;
            } else if i == '6' {
                rank = SIXTH_RANK;
            } else if i == '7' {
                rank = SEVENTH_RANK;
            } else if i == '8' {
                rank = EIGHTH_RANK;
            }
        }
    }
    
    return file & rank;
}

// Returns a role if a uci move is a promotion, else none
pub fn is_uci_promotion(uci_move: &str) -> Option<Role> {
    let promotion_candidate = uci_move.chars().last().unwrap();
    if promotion_candidate.is_alphabetic() {
        if promotion_candidate == 'q' {
            return Some(Role::Queen);
        } else if promotion_candidate == 'r' {
            return Some(Role::Rook);
        } else if promotion_candidate == 'b' {
            return Some(Role::Bishop);
        } else if promotion_candidate == 'n' {
            return Some(Role::Knight);
        }
    }
    return None;
}

impl Move {
    pub fn from_uci(board: &Board, uci: String) -> Move {
        let to_square = get_uci_square_bitboard(&uci[2..4]);
        let from_square = get_uci_square_bitboard(&uci[0..2]);
        match uci.as_str() {
            "e1g1" => return Move {
                role: Some(Role::King),
                colour: Some(Colour::White),
                from_square: from_square,
                to_square: to_square,
                en_passant: false,
                en_passant_target: EMPTY_BITBOARD,
                castle: true,
                promotion: None,
                capture: None,
            },
            "e1c1" => return Move {
                role: Some(Role::King),
                colour: Some(Colour::White),
                from_square: from_square,
                to_square: to_square,
                en_passant: false,
                en_passant_target: EMPTY_BITBOARD,
                castle: true,
                promotion: None,
                capture: None,
            },
            "e8g8" => return Move {
                role: Some(Role::King),
                colour: Some(Colour::Black),
                from_square: from_square,
                to_square: to_square,
                en_passant: false,
                en_passant_target: EMPTY_BITBOARD,
                castle: true,
                promotion: None,
                capture: None,
            },
            "e8c8" => return Move {
                role: Some(Role::King),
                colour: Some(Colour::Black),
                from_square: from_square,
                to_square: to_square,
                en_passant: false,
                en_passant_target: EMPTY_BITBOARD,
                castle: true,
                promotion: None,
                capture: None,
            },
            _ => {
                return Move {
                    role: if let Some(get_role) = get_role(&board, &from_square) {
                        Some(get_role)
                    } else {
                        None},
                    colour: if let Some(get_colour) = get_colour(&board, &from_square) {
                        Some(get_colour)
                    } else {
                        None},
                    from_square: from_square,
                    to_square: to_square,
                    en_passant_target: if (from_square & SECOND_RANK != EMPTY_BITBOARD) & (to_square & FOURTH_RANK != EMPTY_BITBOARD) {
                        from_square.get_file() & THIRD_RANK
                    } else if (from_square & SEVENTH_RANK != EMPTY_BITBOARD) & (to_square & FIFTH_RANK != EMPTY_BITBOARD) {
                        from_square.get_file() & SIXTH_RANK
                    } else {
                        EMPTY_BITBOARD
                    },
                    en_passant: if to_square == board.en_passant_target_square {
                        true
                    } else {
                        false
                    },
                    castle: false,
                    promotion: is_uci_promotion(&uci),
                    capture: if let Some(piece) = get_role(&board, &to_square) {
                        Some(piece)
                    } else if to_square == board.en_passant_target_square {
                        Some(Role::Pawn)
                    } else {
                        None}
                
                };
            }
        }
    }
}