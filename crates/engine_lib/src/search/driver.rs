use crate::{
    Board, Move, SearchResult,
    search::{
        SearchCore, SearchReporter, Searcher,
        control::{SearchConstraint, SearchControl},
        metrics::SearchMetrics,
        zobrist::{SearchContext, TranspositionTable, ZobristTable},
    },
};

pub struct SearchDriver<S: SearchCore> {
    core_searcher: S,
    iterative_mode: IterativeMode,
    context: SearchContext,
}

pub enum IterativeMode {
    Iterative,
    FixedDepth,
}

pub enum TTMode {
    WithTT,
    WithoutTT,
}

const DEFAULT_DEEPENING_DEPTH: u8 = 4;
const DEFAULT_TT_SIZE: usize = 256;

impl<S: SearchCore> SearchDriver<S> {
    pub fn new(core_searcher: S, iterative_mode: IterativeMode, tt_mode: TTMode) -> Self {
        Self {
            core_searcher,
            iterative_mode,
            context: {
                match tt_mode {
                    TTMode::WithTT => SearchContext {
                        tt: Some(TranspositionTable::new(DEFAULT_TT_SIZE)),
                        zobrist: Some(ZobristTable::new()),
                    },

                    TTMode::WithoutTT => SearchContext {
                        tt: None,
                        zobrist: None,
                    },
                }
            },
        }
    }

    pub fn iterative_tt(core_searcher: S) -> Self {
        Self::new(core_searcher, IterativeMode::Iterative, TTMode::WithTT)
    }

    pub fn fixed_tt(core_searcher: S) -> Self {
        Self::new(core_searcher, IterativeMode::FixedDepth, TTMode::WithTT)
    }

    pub fn iterative_no_tt(core_searcher: S) -> Self {
        Self::new(core_searcher, IterativeMode::Iterative, TTMode::WithoutTT)
    }

    pub fn fixed_no_tt(core_searcher: S) -> Self {
        Self::new(core_searcher, IterativeMode::FixedDepth, TTMode::WithoutTT)
    }

    pub fn is_iterative(&self) -> bool {
        match self.iterative_mode {
            IterativeMode::Iterative => true,
            IterativeMode::FixedDepth => false,
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

        if let Some(zobrist) = &self.context.zobrist {
            board.set_hash(zobrist.compute_from_scratch(board));
        }

        let requested_depth_limit = match constraint.max_depth {
            Some(depth) => depth,
            None => match self.iterative_mode {
                IterativeMode::Iterative => u8::MAX,
                IterativeMode::FixedDepth => DEFAULT_DEEPENING_DEPTH,
            },
        };

        // For just one depth
        if !(self.is_iterative()) {
            let result = self.core_searcher.search_at_depth(
                board,
                &mut self.context,
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

            let new_result = self.core_searcher.search_at_depth(
                board,
                &mut self.context,
                current_depth,
                control,
                &mut metrics,
            );

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
