use crate::{Board, MoveList, move_ordering::OrderingPolicy};

#[derive(Clone, Copy)]
pub struct NoOrdering;

impl OrderingPolicy for NoOrdering {
    fn order_moves(&mut self, _board: &mut Board, _moves: &mut MoveList) {}
}
