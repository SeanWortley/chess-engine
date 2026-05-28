use super::TransitionManager;
use crate::{Board, Move};

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
    fn make(&mut self, board: &mut Board, mv: Move, zobrist: Option<&crate::search::zobrist::ZobristTable>) {
        self.history.push(board.clone());
        let mut copy = board.clone();
        copy.apply(mv, zobrist);
        *board = copy;
    }

    fn unmake(&mut self, board: &mut Board, _mv: Move) {
        let previous = self
            .history
            .pop()
            .expect("CopyMakeTransition::unmake called without matching make");
        *board = previous;
    }
}
