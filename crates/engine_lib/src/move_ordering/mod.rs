pub mod mvv_lva;
pub mod no_ordering;

pub use mvv_lva::MvvLva;
pub use no_ordering::NoOrdering;

use crate::{Board, MoveList};

pub trait OrderingPolicy {
    fn order_moves(&mut self, board: &mut Board, moves: &mut MoveList);
}
