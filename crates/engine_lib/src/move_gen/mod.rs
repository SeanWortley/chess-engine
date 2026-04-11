use crate::{Board, MoveList};

pub mod attacks;
pub mod naive;

pub use attacks::IsAttackedFn;
pub use naive::NaiveMoveGenerator;

pub trait MoveGenerator {
    fn generate_moves(&mut self, board: &mut Board, moves: &mut MoveList);
}
