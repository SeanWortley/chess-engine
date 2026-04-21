use crate::{
    Board, Move, SearchResult,
    search::{
        SearchCore, Searcher,
        control::{SearchConstraint, SearchControl},
    },
};

pub struct SearchDriver<S: SearchCore> {
    core_searcher: S,
    mode: SearchMode,
}

pub enum SearchMode {
    Iterative,
    FixedDepth,
}

const DEFAULT_DEEPENING_DEPTH: u8 = 4;

impl<S: SearchCore> SearchDriver<S> {
    pub fn new(core_searcher: S, mode: SearchMode) -> Self {
        Self {
            core_searcher,
            mode,
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
    ) -> SearchResult {
        let requested_depth_limit = match constraint.max_depth {
            Some(depth) => depth,
            None => match self.mode {
                SearchMode::Iterative => u8::MAX,
                SearchMode::FixedDepth => DEFAULT_DEEPENING_DEPTH,
            },
        };

        if !(self.is_iterative()) {
            return self
                .core_searcher
                .search_at_depth(board, requested_depth_limit, control);
        }

        let mut best_result = SearchResult {
            best_move: None,
            score: i16::MIN,
        };

        for current_depth in 1..=requested_depth_limit {
            if control.should_stop() {
                break;
            }

            let new_result = self
                .core_searcher
                .search_at_depth(board, current_depth, control);

            if !control.should_stop() {
                best_result = new_result;
            }
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
