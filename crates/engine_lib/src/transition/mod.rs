use crate::core::board::Board;
use crate::core::moves::Move;

pub mod copy_make;
pub mod make_unmake;

pub trait StateManager {
    fn make(&mut self, board: &mut Board, mv: Move);
    fn unmake(&mut self, board: &mut Board, mv: Move);
}
