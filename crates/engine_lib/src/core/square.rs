use core::panic;
use std::fmt;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Square(u8);

impl Square {
    pub const A1: Self = Self(0);
    pub const B1: Self = Self(1);
    pub const C1: Self = Self(2);
    pub const D1: Self = Self(3);
    pub const E1: Self = Self(4);
    pub const F1: Self = Self(5);
    pub const G1: Self = Self(6);
    pub const H1: Self = Self(7);

    pub const A8: Self = Self(56);
    pub const B8: Self = Self(57);
    pub const C8: Self = Self(58);
    pub const D8: Self = Self(59);
    pub const E8: Self = Self(60);
    pub const F8: Self = Self(61);
    pub const G8: Self = Self(62);
    pub const H8: Self = Self(63);

    pub fn new(file: u8, rank: u8) -> Self {
        Square(rank * 8 + file)
    }
    pub fn from_index(index: u8) -> Self {
        Square(index)
    }
    pub fn from_name(name: &str) -> Self {
        let mut chars = name.chars();

        let file_char = match chars.next() {
            Some(character) => character,
            None => {
                panic!("Null File Char")
            }
        };

        let rank_char = match chars.next() {
            Some(character) => character,
            None => {
                panic!("Null Rank Char")
            }
        };

        let file = file_char as u8 - b'a';
        let rank = rank_char as u8 - b'1';

        Square::new(file, rank)
    }
    pub fn to_name(self) -> &str {}
    pub fn index(self) -> u8 {
        self.0
    }
    pub fn file(self) -> u8 {
        self.0 % 8
    }
    pub fn rank(self) -> u8 {
        self.0 / 8
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let file = (b'a' + self.file()) as char;
        let rank = self.rank() + 1;
        write!(f, "{}{}", file, rank)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let square = Square::new(4, 2);
        assert_eq!(square.file(), 4);
        assert_eq!(square.rank(), 2);
        assert_eq!(square.index(), 20);
    }

    #[test]
    fn test_from_index() {
        let square = Square::from_index(0);
        assert_eq!(square.index(), 0);
    }

    #[test]
    fn test_from_name() {
        let mut square = Square::from_name("a1");
        assert_eq!(square.index(), 0);
        square = Square::from_name("h8");
        assert_eq!(square.index(), 63);
    }

    #[test]
    fn test_display() {
        let a1 = Square::new(0, 0);
        let h8 = Square::new(7, 7);
        assert_eq!(a1.to_string(), "a1");
        assert_eq!(h8.to_string(), "h8");
    }
    #[test]
    fn test_recycled() {
        let old = Square::new(4, 7);
        let new = Square::new(old.file(), old.rank());
        assert_eq!(old, new);
    }
}
