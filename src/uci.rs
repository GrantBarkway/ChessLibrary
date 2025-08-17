use crate::mv::Move;
use crate::bitboard::{Bitboard, EMPTY_BITBOARD};
use crate::square::{EIGHTH_RANK, FIFTH_RANK, FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H, FIRST_RANK, FOURTH_RANK, SECOND_RANK, SEVENTH_RANK, SIXTH_RANK, THIRD_RANK};
use crate::role::Role;

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
        square_string.push_str("A");
    } else if sq & FILE_B != EMPTY_BITBOARD {
        square_string.push_str("B");
    } else if sq & FILE_C != EMPTY_BITBOARD {
        square_string.push_str("C");
    } else if sq & FILE_D != EMPTY_BITBOARD {
        square_string.push_str("D");
    } else if sq & FILE_E != EMPTY_BITBOARD {
        square_string.push_str("E");
    } else if sq & FILE_F != EMPTY_BITBOARD {
        square_string.push_str("F");
    } else if sq & FILE_G != EMPTY_BITBOARD {
        square_string.push_str("G");
    } else if sq & FILE_H != EMPTY_BITBOARD {
        square_string.push_str("H");
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