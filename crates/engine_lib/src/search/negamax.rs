use crate::{
    Board, Move, MoveGenerator, NEG_INF, POS_INF, SearchResult, Searcher, TransitionManager,
    search::{
        LeafPolicy,
        control::{SearchConstraint, SearchControl},
        kernel::AlphaBetaKernel,
    },
};
const DEFAULT_NEGAMAX_DEPTH: u8 = 4;

pub struct PureNegamaxSearcher<TM: TransitionManager, MG: MoveGenerator, LP: LeafPolicy> {
    kernel: AlphaBetaKernel<TM, MG, LP>,
}

impl<TM: TransitionManager, MG: MoveGenerator, LP: LeafPolicy> Searcher
    for PureNegamaxSearcher<TM, MG, LP>
{
    // Called by iterative deepening
    fn start_search(
        &mut self,
        board: &mut Board,
        depth: u8,
        control: &SearchControl,
    ) -> SearchResult {
        self.kernel.search(board, NEG_INF, POS_INF, depth, control)
    }

    fn make(&mut self, board: &mut Board, mv: Move) {
        self.kernel.make(board, mv);
    }

    fn unmake(&mut self, board: &mut Board, mv: Move) {
        self.kernel.unmake(board, mv);
    }
}
