use crate::search::control::{SearchConstraint, SearchControl};
use crate::{Board, Move};

pub mod alpha_beta;
pub mod control;
pub mod driver;
pub mod kernel;
pub mod negamax;
pub mod static_leaf;

pub use driver::SearchDriver;
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

pub trait SearchCore {
    fn search_at_depth(
        &mut self,
        board: &mut Board,
        depth: u8,
        control: &SearchControl,
    ) -> SearchResult;

    // Passed up from transition manager, so engine can use them directly :)
    fn make(&mut self, board: &mut Board, mv: Move);
    fn unmake(&mut self, board: &mut Board, mv: Move);
}

pub trait LeafPolicy {
    fn evaluate_leaf(&mut self, board: &mut Board, alpha: i16, beta: i16) -> i16;
}
