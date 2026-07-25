use crate::search::control::{SearchConstraint, SearchControl};
use crate::search::metrics::SearchMetrics;
use crate::search::tt::SearchContext;
use crate::{Board, Move};

pub mod alpha_beta;
pub mod control;
pub mod driver;
pub mod kernel;
pub mod metrics;
pub mod negamax;
pub mod quiescent_leaf;
pub mod static_leaf;
pub mod tt;

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
    /// `game_history` holds the zobrist hash of every position the game has
    /// visited, oldest first, ending with the current (root) position.
    fn start_search(
        &mut self,
        board: &mut Board,
        game_history: &[u64],
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
        context: &mut SearchContext,
        depth: u8,
        control: &SearchControl,
        metrics: &mut SearchMetrics,
    ) -> SearchResult;

    // Passed up from transition manager, so engine can use them directly :)
    fn make(&mut self, board: &mut Board, mv: Move);
    fn unmake(&mut self, board: &mut Board, mv: Move);
}

pub trait LeafPolicy {
    fn evaluate_leaf(
        &mut self,
        board: &mut Board,
        alpha: i16,
        beta: i16,
        control: &SearchControl,
        metrics: &mut SearchMetrics,
    ) -> i16;
}
