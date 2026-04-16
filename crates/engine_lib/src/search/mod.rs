use crate::{Board, Move};

pub mod negamax;

pub use negamax::PureNegamaxSearcher;

#[derive(Clone, Copy)]
pub struct EndGameFlag {}

pub struct SearchResult {
    pub best_move: Option<Move>,
    pub score: i16,
    //pub nodes: u64,
}

pub trait Searcher {
    fn start_search(&mut self, board: &mut Board) -> SearchResult;
    fn make(&mut self, board: &mut Board, mv: Move);
    fn unmake(&mut self, board: &mut Board, mv: Move);
}
