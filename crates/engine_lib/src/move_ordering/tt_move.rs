use crate::{
    Board, Move,
    move_ordering::{OrderingContext, scorer::MoveScorer},
};

#[derive(Clone, Copy)]
pub struct TtMoveScorer;

impl MoveScorer for TtMoveScorer {
    fn score_move(&self, _board: &Board, mv: Move, context: OrderingContext) -> Option<i32> {
        if context.tt_move == Some(mv) {
            return Some(0);
        }
        None
    }
    fn on_beta_cutoff(&mut self, _mv: Move, _root_distance: u8, _depth: u8) {}
}
