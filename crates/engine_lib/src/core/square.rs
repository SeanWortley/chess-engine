pub struct Square(u8);

impl Square {
    pub fn new(file: u8, rank: u8) -> Self {
        Square(rank * 8 + file)
    }
    pub fn file(self) -> u8 {
        self.0 % 8
    }
    pub fn rank(self) -> u8 {
        self.0 / 8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let square = Square::new(4, 2);
        assert_eq!(square.0, 12);
        assert_eq!(square.file(), 4);
        assert_eq!(square.rank(), 2);
    }
}
