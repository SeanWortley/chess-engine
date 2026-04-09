use crate::core::board::Board;
use crate::core::moves::Move;

pub mod negamax;

#[derive(Clone, Copy)]
pub struct EndGameFlag {}

pub struct SearchResult {
    pub best_move: Option<Move>,
    pub score: i16,
    //pub nodes: u64,
}

pub trait Searcher {
    fn start_search(&mut self, board: &mut Board) -> SearchResult;
}
