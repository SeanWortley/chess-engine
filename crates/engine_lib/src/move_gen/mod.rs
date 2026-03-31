use crate::core::board::Board;
use crate::core::move_list::MoveList;
use crate::core::moves::Move;

pub mod attacks;
pub mod legality;
pub mod naive;

pub trait MoveGenerator {
    fn generate_moves(&self, board: &Board, moves: &mut MoveList);
}

pub trait LegalityStrategy {
    fn generate_legal_moves(&self, board: &Board, moves: &mut MoveList);
}
