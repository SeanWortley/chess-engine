use super::TransitionManager;
use crate::core::board::Board;
use crate::core::moves::Move;

pub struct CopyMakeTransition {
    history: Vec<Board>,
}

impl CopyMakeTransition {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }
}

impl Default for CopyMakeTransition {
    fn default() -> Self {
        Self::new()
    }
}

impl TransitionManager for CopyMakeTransition {
    #[inline]
    fn make(&mut self, board: &mut Board, mv: Move) {
        self.history.push(board.clone());
        let mut copy = board.clone();
        copy.apply(mv);
        *board = copy;
    }

    #[inline]
    fn unmake(&mut self, board: &mut Board, _mv: Move) {
        let previous = self
            .history
            .pop()
            .expect("CopyMakeTransition::unmake called without matching make");
        *board = previous;
    }
}
