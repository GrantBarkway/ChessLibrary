// Just while testing
#![allow(dead_code, unused_variables)]

use crate::board::Bitboard;

pub struct Move {
    pub target_square: Bitboard,
    pub from_square: Bitboard,
}

enum Rank {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth
}

impl Rank {
    pub const fn from_char(ch: char) -> Option<Rank> {
        Some(match ch {
            '1' => Rank::First,
            '2' => Rank::Second,
            '3' => Rank::Third,
            '4' => Rank::Fourth,
            '5' => Rank::Fifth,
            '6' => Rank::Sixth,
            '7' => Rank::Seventh,
            '8' => Rank::Eighth,
            _ => return None,
        })
    }

    pub const fn to_char(self) -> char {
        match self {
            Rank::First => '1',
            Rank::Second => '2',
            Rank::Third => '3',
            Rank::Fourth => '4',
            Rank::Fifth => '5',
            Rank::Sixth => '6',
            Rank::Seventh => '7',
            Rank::Eighth => '8',
        }
    }
}

enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H
}