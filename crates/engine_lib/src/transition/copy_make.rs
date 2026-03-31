use super::TransitionManager;
use crate::core::board::Board;
use crate::core::moves::Move;

pub struct CopyMakeTransition;

impl TransitionManager for CopyMakeTransition {
    type BoardReference = Board; // Returns a copy

    // applies a move to a copy of the board, and returns the altered copy.
    fn make(&mut self, board: &mut Board, mv: Move) -> Board {
        let mut copy = board.clone();
        copy.apply(mv);
        copy
    }
}
