pub mod mvv_lva;
pub mod no_ordering;
pub mod scored_ordering;
pub mod scorer;
pub mod tt_move;

pub use mvv_lva::MvvLva;
pub use no_ordering::NoOrdering;

use crate::{Board, Move, MoveList};

pub trait OrderingPolicy {
    fn order_moves(&mut self, board: &mut Board, moves: &mut MoveList, context: OrderingContext);

    fn on_beta_cutoff(&mut self, _mv: Move, _root_distance: u8, _depth: u8) {}
}

#[derive(Clone, Copy)]
pub struct OrderingContext {
    pub tt_move: Option<Move>,
    pub root_distance: u8,
}

impl OrderingContext {
    pub fn new(tt_move: Option<Move>, root_distance: u8) -> Self {
        OrderingContext {
            tt_move,
            root_distance,
        }
    }

    pub fn empty() -> Self {
        OrderingContext {
            tt_move: None,
            root_distance: 0,
        }
    }
}
