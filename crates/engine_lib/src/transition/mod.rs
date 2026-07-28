use crate::{Board, Move};

pub mod copy_make;
pub mod make_unmake;

pub use copy_make::CopyMakeTransition;

pub trait TransitionManager {
    fn make(&mut self, board: &mut Board, mv: Move);
    fn unmake(&mut self, board: &mut Board, mv: Move);
}
