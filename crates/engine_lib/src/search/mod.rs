use crate::search::control::{SearchConstraint, SearchControl};
use crate::search::metrics::SearchMetrics;
use crate::{Board, Move, MoveList};

pub mod alpha_beta;
pub mod control;
pub mod driver;
pub mod kernel;
pub mod metrics;
pub mod mvv_lva;
pub mod negamax;
pub mod no_ordering;
pub mod static_leaf;
pub mod zobrist;

pub use driver::SearchDriver;
pub use negamax::PureNegamaxSearcher;

pub trait SearchReporter {
    fn report_depth(
        &self,
        depth: u8,
        nodes: u64,
        time_ms: u128,
        score: i16,
        best_move: Option<Move>,
    );
}

#[derive(Clone, Copy)]
pub struct EndGameFlag {}

pub struct SearchResult {
    pub best_move: Option<Move>,
    pub score: i16,
}

pub trait Searcher {
    fn start_search(
        &mut self,
        board: &mut Board,
        constraint: SearchConstraint,
        control: &SearchControl,
        reporter: &dyn SearchReporter,
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
        metrics: &mut SearchMetrics,
    ) -> SearchResult;

    // Passed up from transition manager, so engine can use them directly :)
    fn make(&mut self, board: &mut Board, mv: Move);
    fn unmake(&mut self, board: &mut Board, mv: Move);
}

pub trait LeafPolicy {
    fn evaluate_leaf(&mut self, board: &mut Board, alpha: i16, beta: i16) -> i16;
}

pub trait OrderingPolicy {
    fn order_moves(&mut self, board: &mut Board, moves: &mut MoveList);
}
