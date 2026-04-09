use crate::core::square::Square;
use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Bitboard(u64);

pub struct BitboardIter(Bitboard);

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // In reverse, so top to bottom
        for rank in (0..8).rev() {
            write!(f, "{} |", rank + 1)?;
            for file in 0..8 {
                if self.has_square(Square::new(file, rank)) {
                    write!(f, "1 ")?;
                } else {
                    write!(f, ". ")?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f, "   ---------------")?;
        write!(f, "   A B C D E F G H")?;
        Ok(())
    }
}

impl BitAnd for Bitboard {
    type Output = Bitboard;
    fn bitand(self, other: Bitboard) -> Bitboard {
        Bitboard(self.0 & other.0)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, other: Bitboard) {
        self.0 &= other.0;
    }
}

impl BitOr for Bitboard {
    type Output = Bitboard;
    fn bitor(self, other: Bitboard) -> Bitboard {
        Bitboard(self.0 | other.0)
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, other: Bitboard) {
        self.0 |= other.0;
    }
}

impl BitXor for Bitboard {
    type Output = Bitboard;
    fn bitxor(self, other: Bitboard) -> Bitboard {
        Bitboard(self.0 ^ other.0)
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, other: Self) {
        self.0 ^= other.0;
    }
}

impl Not for Bitboard {
    type Output = Bitboard;
    fn not(self) -> Bitboard {
        Bitboard(!self.0)
    }
}

impl Iterator for BitboardIter {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_lsb()
    }
}

impl Bitboard {
    pub const EMPTY: Bitboard = Bitboard(0);
    pub const FULL: Bitboard = Bitboard(u64::MAX);
    pub fn inner(self) -> u64 {
        self.0
    }
    pub fn from_square(square: Square) -> Self {
        debug_assert!(square.index() < 64); // Don't trust squares >:(
        Bitboard(1u64 << square.index())
    }
    pub fn add_square(&mut self, square: Square) {
        *self |= Bitboard::from_square(square);
    }
    pub fn remove_square(&mut self, square: Square) {
        *self &= !Bitboard::from_square(square);
    }
    pub fn has_square(self, square: Square) -> bool {
        (self & Bitboard::from_square(square)) != Bitboard::EMPTY
    }
    pub fn count(self) -> u32 {
        self.0.count_ones()
    }
    pub fn is_empty(self) -> bool {
        self == Bitboard::EMPTY
    }
    pub fn lsb(self) -> Option<u8> {
        if self.0 == 0 {
            return None;
        }
        Some(self.0.trailing_zeros() as u8)
    }
    pub fn pop_lsb(&mut self) -> Option<u8> {
        if self.0 == 0 {
            return None;
        }
        let index = self.0.trailing_zeros() as u8;
        self.0 &= self.0 - 1; // -1 certainly removes last bit, and potentially generates new bits beneath that, so &ing the two removes exactly 1 bit
        Some(index)
    }
    pub fn iter(self) -> BitboardIter {
        BitboardIter(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(Bitboard::EMPTY.inner(), 0);
    }

    #[test]
    fn test_full() {
        assert_eq!(Bitboard::FULL.inner(), u64::MAX);
    }

    #[test]
    fn test_from_square() {
        let square = Square::new(0, 0);

        // Does from_square set the correct bits?
        assert_eq!(Bitboard::from_square(square).inner(), 1);
    }

    #[test]
    fn test_add_square() {
        let mut bb = Bitboard::from_square(Square::new(0, 0));
        let square = Square::new(1, 0);
        bb.add_square(square);

        // Does add_square add the correct bit?
        assert_eq!(bb.inner(), 3);

        // Does calling it twice break something?
        bb.add_square(square);
        assert_eq!(bb.inner(), 3);
    }

    #[test]
    fn test_remove_square() {
        let square = Square::new(0, 0);
        let mut bb = Bitboard::from_square(square);
        bb.remove_square(square);

        // Does it set the right bit to zero?
        assert_eq!(bb, Bitboard::EMPTY);

        // Does calling it on an already empty bit break something?
        bb.remove_square(square);
        assert_eq!(bb, Bitboard::EMPTY)
    }

    #[test]
    fn test_has_square() {
        let square = Square::new(0, 0);
        let bb = Bitboard::from_square(square);

        // Does it return true correctly?
        assert!(bb.has_square(square));

        // Does it return false correctly?
        assert!(!bb.has_square(Square::new(1, 1)));
    }

    #[test]
    fn test_count() {
        let mut bb = Bitboard::EMPTY;
        for i in 0u8..8 {
            bb.add_square(Square::new(i, i));
        }

        // Does it identify 8 bits correctly?
        assert_eq!(bb.count(), 8);
    }

    #[test]
    fn test_is_empty() {
        let square = Square::new(0, 0);
        let mut bb = Bitboard::from_square(square);

        // Does it return false correctly?
        assert!(!bb.is_empty());

        // Does it return true correctly?
        bb.remove_square(square);
        assert!(bb.is_empty());
    }

    #[test]
    fn test_lsb() {
        let mut bb = Bitboard::EMPTY;
        bb.add_square(Square::new(0, 0));
        bb.add_square(Square::new(1, 1));

        // Does lsb return the least significant bit without modifying the bitboard?
        assert_eq!(bb.lsb(), Some(0));
        assert!(!bb.is_empty());
    }

    #[test]
    fn test_pop_lsb() {
        let mut bb = Bitboard::EMPTY;
        bb.add_square(Square::new(0, 0));
        bb.add_square(Square::new(1, 1));

        let mut popped = vec![];
        popped.push(bb.pop_lsb());
        popped.push(bb.pop_lsb());

        // Does popping return?
        assert_eq!(popped.len(), 2);

        // Does popping remove?
        assert!(bb.is_empty())
    }

    #[test]
    fn test_iter() {
        let mut bb = Bitboard::EMPTY;
        bb.add_square(Square::new(0, 0));
        bb.add_square(Square::new(1, 1));

        let mut popped = vec![];
        for square in bb.iter() {
            popped.push(square);
        }
        // Does iter() cover all 1 bits?
        assert_eq!(popped.len(), 2);
    }
}
