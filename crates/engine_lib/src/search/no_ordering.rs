use crate::{Board, MoveList, search::OrderingPolicy};

#[derive(Clone, Copy)]
pub struct NoOrdering {}

impl OrderingPolicy for NoOrdering {
    fn order_moves(&mut self, _board: &mut Board, moves: MoveList) -> MoveList {
        moves
    }
}
