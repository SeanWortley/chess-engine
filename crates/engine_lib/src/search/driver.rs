use crate::{
    Board, Move, SearchResult,
    core::zobrist::ZOBRIST,
    search::{
        SearchCore, SearchReporter, Searcher,
        control::{SearchConstraint, SearchControl},
        metrics::SearchMetrics,
        tt::{SearchContext, TranspositionTable},
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
                        history: Vec::new(),
                    },

                    TTMode::WithoutTT => SearchContext {
                        tt: None,
                        history: Vec::new(),
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
        game_history: &[u64],
        constraint: SearchConstraint,
        control: &SearchControl,
        reporter: &dyn SearchReporter,
    ) -> SearchResult {
        let mut metrics = SearchMetrics::new();

        assert_eq!(board.hash(), ZOBRIST.compute_from_scratch(board));

        self.context.history.clear();
        self.context.history.extend_from_slice(game_history);
        // Convention: history always ends with the current (root) position.
        // Covers callers that pass an empty or root-less history.
        if self.context.history.last() != Some(&board.hash()) {
            self.context.history.push(board.hash());
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
            if current_depth > 1 && control.should_stop(&mut metrics) {
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
            } else if best_result.best_move.is_none() {
                best_result = new_result;

                reporter.report_depth(
                    current_depth,
                    metrics.total(),
                    control.elapsed().as_millis(),
                    best_result.score,
                    best_result.best_move,
                );
                return best_result;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        CopyMakeTransition, NaiveMoveGenerator,
        eval::pesto::PestoEvaluator,
        move_gen::attacks::ray_is_attacked,
        search::{
            alpha_beta::AlphaBetaSearcher, control::SearchConstraint, static_leaf::StaticLeaf,
        },
    };
    use crate::move_ordering::MvvLva;

    struct NullReporter;
    impl SearchReporter for NullReporter {
        fn report_depth(&self, _: u8, _: u64, _: u128, _: i16, _: Option<Move>) {}
    }

    // A search that times out before any depth completes must still return a
    // legal move — a None best_move becomes "bestmove 0000" at the UCI layer,
    // which cutechess scores as an illegal-move forfeit.
    #[test]
    fn test_returns_move_when_stopped_before_first_depth() {
        let mut board = Board::starting_position();

        let core = AlphaBetaSearcher::new(
            CopyMakeTransition::new(),
            NaiveMoveGenerator::new(CopyMakeTransition::new(), ray_is_attacked),
            StaticLeaf::new(PestoEvaluator::new()),
            MvvLva {},
            ray_is_attacked,
        );
        let mut driver = SearchDriver::iterative_tt(core);

        let control = SearchControl::new(SearchConstraint::fixed_depth(5));
        control.stop(); // simulate the timeout having already expired

        let result = driver.start_search(
            &mut board,
            &[],
            SearchConstraint::fixed_depth(5),
            &control,
            &NullReporter,
        );

        assert!(
            result.best_move.is_some(),
            "stopped search returned no move; this becomes an illegal 'bestmove 0000' forfeit"
        );
    }
}
