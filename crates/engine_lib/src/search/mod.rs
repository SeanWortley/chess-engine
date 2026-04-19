use crate::search::control::{SearchConstraint, SearchControl};
use crate::{Board, Move};

pub mod control;
pub mod deepening_search;
pub mod negamax;

pub use deepening_search::DeepeningSearcher;
pub use negamax::PureNegamaxSearcher;

#[derive(Clone, Copy)]
pub struct EndGameFlag {}

pub struct SearchResult {
    pub best_move: Option<Move>,
    pub score: i16,
    //pub nodes: u64,
}

pub trait Searcher {
    fn start_search(
        &mut self,
        board: &mut Board,
        constraint: SearchConstraint,
        control: &SearchControl,
    ) -> SearchResult;

    // Passed up from transition manager, so engine can use them directly :)
    fn make(&mut self, board: &mut Board, mv: Move);
    fn unmake(&mut self, board: &mut Board, mv: Move);
}
