use crate::core::moves::Move;
const MAX_MOVES: usize = 218; // Maximum number of available moves in any chess position :)
pub struct MoveList {
    moves: [Move; MAX_MOVES],
    len: usize,
}

impl MoveList {
    #[inline]
    pub fn new() -> Self {
        MoveList {
            moves: [Move::default(); MAX_MOVES],
            len: 0,
        }
    }
    #[inline]
    pub fn push(&mut self, new_move: Move) {
        self.moves[self.len] = new_move;
        self.len += 1
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &Move> {
        self.moves[..self.len].iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const ORIGIN: u8 = 0;
    const DESTINATION: u8 = 12;

    #[test]
    fn test_push() {
        let mut move_list = MoveList::new();
        assert!(move_list.is_empty());

        move_list.push(Move::capture(ORIGIN, DESTINATION));
        assert!(!move_list.is_empty());
        assert_eq!(move_list.len(), 1);
    }
    #[test]
    fn test_iter() {
        let mut move_list = MoveList::new();

        for _i in 0..3 {
            move_list.push(Move::capture(ORIGIN, DESTINATION));
        }
        assert_eq!(move_list.len(), 3);
    }
}
