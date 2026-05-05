use crate::{
    Board, Move, SearchResult,
    search::{
        SearchCore, SearchReporter, Searcher,
        control::{SearchConstraint, SearchControl},
        metrics::SearchMetrics,
        zobrist::SearchContext,
    },
};

pub struct SearchDriver<S: SearchCore> {
    core_searcher: S,
    mode: SearchMode,
    context: SearchContext,
}

pub enum SearchMode {
    Iterative,
    FixedDepth,
}

const DEFAULT_DEEPENING_DEPTH: u8 = 4;
const DEFAULT_TT_SIZE: usize = 256;

impl<S: SearchCore> SearchDriver<S> {
    pub fn new(core_searcher: S, mode: SearchMode) -> Self {
        Self {
            core_searcher,
            mode,
            context: SearchContext::with_tt(DEFAULT_TT_SIZE),
        }
    }

    pub fn iterative(core_searcher: S) -> Self {
        Self::new(core_searcher, SearchMode::Iterative)
    }

    pub fn fixed(core_searcher: S) -> Self {
        Self::new(core_searcher, SearchMode::FixedDepth)
    }

    pub fn is_iterative(&self) -> bool {
        match self.mode {
            SearchMode::Iterative => true,
            SearchMode::FixedDepth => false,
        }
    }
}

impl<S: SearchCore> Searcher for SearchDriver<S> {
    fn start_search(
        &mut self,
        board: &mut Board,
        constraint: SearchConstraint,
        control: &SearchControl,
        reporter: &dyn SearchReporter,
    ) -> SearchResult {
        let mut metrics = SearchMetrics::new();

        let requested_depth_limit = match constraint.max_depth {
            Some(depth) => depth,
            None => match self.mode {
                SearchMode::Iterative => u8::MAX,
                SearchMode::FixedDepth => DEFAULT_DEEPENING_DEPTH,
            },
        };

        // For just one depth
        if !(self.is_iterative()) {
            let result = self.core_searcher.search_at_depth(
                board,
                requested_depth_limit,
                control,
                &mut metrics,
            );

            reporter.report_depth(
                requested_depth_limit,
                metrics.total(),
                control.elapsed().as_millis(),
                result.score,
                result.best_move,
            );

            return result;
        }

        // Prepare for iteration
        let mut best_result = SearchResult {
            best_move: None,
            score: i16::MIN,
        };

        for current_depth in 1..=requested_depth_limit {
            if control.should_stop(&mut metrics) {
                break;
            }

            let new_result =
                self.core_searcher
                    .search_at_depth(board, current_depth, control, &mut metrics);

            if !control.should_stop(&mut metrics) {
                best_result = new_result;
            }

            reporter.report_depth(
                current_depth,
                metrics.total(),
                control.elapsed().as_millis(),
                best_result.score,
                best_result.best_move,
            );
        }

        best_result
    }

    fn make(&mut self, board: &mut Board, mv: Move) {
        self.core_searcher.make(board, mv);
    }

    fn unmake(&mut self, board: &mut Board, mv: Move) {
        self.core_searcher.unmake(board, mv);
    }
}
