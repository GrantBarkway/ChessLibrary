// Just while testing
#![allow(dead_code, unused_variables)]

use crate::board::{Bitboard};

// A8, B8, C8, D8, E8, F8, G8, H8,
// ...
// ... 
// ...
// ...
// ...
// ...
// A1, B1, C1, D1, E1, F1, G1, H1

// Bitboard read in order (by which bit is set)
// {1,2,3,4,5,6,7,8}
// {9,10,11,12,13,14,15,16}
// {17,18,19,20,21,22,23,24}
// ...
// ...
// ...
// ...
// {57,58,59,60,61,62,63,64}

pub struct Square {
    a8: Bitboard,
    b8: Bitboard,
    c8: Bitboard,
    d8: Bitboard,
    e8: Bitboard,
    f8: Bitboard,
    g8: Bitboard,
    h8: Bitboard,
    a7: Bitboard,
    b7: Bitboard,
    c7: Bitboard,
    d7: Bitboard,
    e7: Bitboard,
    f7: Bitboard,
    g7: Bitboard,
    h7: Bitboard,
    a6: Bitboard,
    b6: Bitboard,
    c6: Bitboard,
    d6: Bitboard,
    e6: Bitboard,
    f6: Bitboard,
    g6: Bitboard,
    h6: Bitboard,  
    a5: Bitboard,
    b5: Bitboard,
    c5: Bitboard,
    d5: Bitboard,
    e5: Bitboard,
    f5: Bitboard,
    g5: Bitboard,
    h5: Bitboard,
    a4: Bitboard,
    b4: Bitboard,
    c4: Bitboard,
    d4: Bitboard,
    e4: Bitboard,
    f4: Bitboard,
    g4: Bitboard,
    h4: Bitboard,
    a3: Bitboard,
    b3: Bitboard,
    c3: Bitboard,
    d3: Bitboard,
    e3: Bitboard,
    f3: Bitboard,
    g3: Bitboard,
    h3: Bitboard, 
    a2: Bitboard,
    b2: Bitboard,
    c2: Bitboard,
    d2: Bitboard,
    e2: Bitboard,
    f2: Bitboard,
    g2: Bitboard,
    h2: Bitboard,
    a1: Bitboard,
    b1: Bitboard,
    c1: Bitboard,
    d1: Bitboard,
    e1: Bitboard,
    f1: Bitboard,
    g1: Bitboard,
    h1: Bitboard,
}