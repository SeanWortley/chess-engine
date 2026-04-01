use crate::core::board::Board;
use crate::core::move_list::MoveList;
// use crate::core::moves::Move;

pub mod attacks;
pub mod naive;

pub trait MoveGenerator {
    fn generate_moves(&self, board: &mut Board, moves: &mut MoveList);
}
