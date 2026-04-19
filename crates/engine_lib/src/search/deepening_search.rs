use crate::{
    Board, Color, Evaluator, Move, MoveGenerator, PureNegamaxSearcher, SearchResult, Searcher,
    Square, TransitionManager,
    search::control::{SearchConstraint, SearchControl},
};

pub struct DeepeningSearcher<TM: TransitionManager, MG: MoveGenerator, E: Evaluator> {
    core_searcher: PureNegamaxSearcher<TM, MG, E>,
}

const DEFAULT_DEEPENING_DEPTH: u8 = 4;

impl<TM: TransitionManager, MG: MoveGenerator, E: Evaluator> Searcher
    for DeepeningSearcher<TM, MG, E>
{
    // Root Iterative Deeping function
    fn start_search(
        &mut self,
        board: &mut Board,
        constraint: SearchConstraint,
        control: &SearchControl,
    ) -> SearchResult {
        let requested_depth_limit = match constraint.max_depth {
            Some(depth) => depth,
            None if constraint.movetime.is_some() => u8::MAX,
            None => DEFAULT_DEEPENING_DEPTH,
        };

        let mut best_result = SearchResult {
            best_move: None,
            score: i16::MIN,
        };

        for current_depth in 1..=requested_depth_limit {
            // holy fuck I love rust :)
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

impl<TM: TransitionManager, MG: MoveGenerator, E: Evaluator> DeepeningSearcher<TM, MG, E> {
    pub fn new(
        tm: TM,
        mg: MG,
        e: E,
        attacked_fn: fn(board: &Board, square: Square, attacking_color: Color) -> bool,
    ) -> Self {
        let core_searcher = PureNegamaxSearcher::new(tm, mg, e, attacked_fn);

        Self { core_searcher }
    }
}
