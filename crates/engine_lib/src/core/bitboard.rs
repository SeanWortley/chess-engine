use crate::core::square::Square;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};

#[derive(Copy, Clone)]
pub struct Bitboard(u64);

impl BitAnd for Bitboard {
    type Output = Bitboard;
    fn bitand(self, other: Bitboard) -> Bitboard {
        Bitboard(self.0 & other.0)
    }
}

impl Bitboard {
    pub const EMPTY: Bitboard = Bitboard(0);
    pub const FULL: Bitboard = Bitboard(u64::MAX);
    pub fn bit(self) -> u64 {
        self.0
    }
    pub fn from_square(square: Square) -> Self {
        Bitboard(1 << square.index())
    }
}
