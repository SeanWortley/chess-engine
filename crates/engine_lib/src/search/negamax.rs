use crate::core::board::{Board, Color};
use crate::core::move_list::MoveList;
use crate::eval::Evaluator;
use crate::move_gen::MoveGenerator;
use crate::search::{SearchResult, Searcher};
use crate::transition::TransitionManager;

pub struct PureNegamaxSearcher<TM: TransitionManager, MG: MoveGenerator, E: Evaluator> {
    tm: TM,
    mg: MG,
    e: E,
    max_depth: u8,
}

impl<TM: TransitionManager, MG: MoveGenerator, E: Evaluator> Searcher
    for PureNegamaxSearcher<TM, MG, E>
{
    fn start_search(&mut self, board: &mut Board) -> SearchResult {
        self.negamax(board, self.max_depth)
    }
}

impl<TM: TransitionManager, MG: MoveGenerator, E: Evaluator> PureNegamaxSearcher<TM, MG, E> {
    pub fn new(tm: TM, mg: MG, e: E, max_depth: u8) -> Self {
        PureNegamaxSearcher {
            tm,
            mg,
            e,
            max_depth,
        }
    }

    fn negamax(&mut self, board: &mut Board, depth: u8) -> Evaluation {
        if depth == 0 {
            return self.e.evaluate(board);
        }
        let mut moves = MoveList::new();
        self.mg.generate_moves(board, &mut moves);

        for mv in moves.iter() {
            toDo!();
        }
    }
}
