use crate::{Board, Move};
use crate::search::zobrist::ZobristTable;

pub mod copy_make;

pub use copy_make::CopyMakeTransition;

pub trait TransitionManager {
    fn make(&mut self, board: &mut Board, mv: Move, zobrist: Option<&ZobristTable>);
    fn unmake(&mut self, board: &mut Board, mv: Move);
}
