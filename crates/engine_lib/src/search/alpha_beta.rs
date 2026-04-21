use crate::{
    Board, Move, MoveGenerator, NEG_INF, POS_INF, SearchResult, Searcher, TransitionManager,
    search::{LeafPolicy, control::SearchControl, kernel::AlphaBetaKernel},
};

pub struct AlphaBetaSearcher<TM: TransitionManager, MG: MoveGenerator, LP: LeafPolicy> {
    kernel: AlphaBetaKernel<TM, MG, LP>,
}

impl<TM: TransitionManager, MG: MoveGenerator, LP: LeafPolicy> Searcher
    for AlphaBetaSearcher<TM, MG, LP>
{
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
