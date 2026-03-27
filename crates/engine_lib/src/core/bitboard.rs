use crate::core::square::Square;

#[derive(Copy, Clone)]
pub struct Bitboard(u64);

impl Bitboard {
    pub fn bit(self) -> u64 {
        self.0
    }
    pub fn empty(self) -> Self {
        Bitboard(0)
    }
}
