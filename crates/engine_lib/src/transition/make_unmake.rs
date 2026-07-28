use super::TransitionManager;
use crate::{Board, Move};

pub struct MakeUnmakeTransition {
    undo_stack: Vec<UndoInfo>,
}

pub struct UndoInfo {}

impl MakeUnmakeTransition {
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
        }
    }
}

impl Default for MakeUnmakeTransition {
    fn default() -> Self {
        Self::new()
    }
}

impl TransitionManager for MakeUnmakeTransition {
    fn make(&mut self, board: &mut Board, mv: Move) {
        self.undo_stack.push();
        board.apply(mv);
    }

    fn unmake(&mut self, board: &mut Board, mv: Move) {}
}
