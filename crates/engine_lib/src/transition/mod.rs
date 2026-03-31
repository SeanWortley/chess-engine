use crate::core::board::Board;
use crate::core::moves::Move;

pub mod copy_make;
// pub mod make_unmake;

pub trait TransitionManager {
    type BoardReference;
    fn make(&mut self, board: &mut Board, mv: Move) -> Self::BoardReference; // The return allows copy-make to return a new Board, and make-unmake to mutate in place :)
    // fn unmake(&mut self, board: &mut Board, mv: Move);
}
