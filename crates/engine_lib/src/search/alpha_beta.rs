use crate::{
    Board, Color, Move, MoveGenerator, NEG_INF, POS_INF, SearchResult, Square, TransitionManager,
    search::{LeafPolicy, SearchCore, control::SearchControl, kernel::AlphaBetaKernel},
};

pub struct AlphaBetaSearcher<TM: TransitionManager, MG: MoveGenerator, LP: LeafPolicy> {
    kernel: AlphaBetaKernel<TM, MG, LP>,
}

impl<TM: TransitionManager, MG: MoveGenerator, LP: LeafPolicy> SearchCore
    for AlphaBetaSearcher<TM, MG, LP>
{
    fn search_at_depth(
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

impl<TM: TransitionManager, MG: MoveGenerator, LP: LeafPolicy> AlphaBetaSearcher<TM, MG, LP> {
    pub fn new(
        tm: TM,
        mg: MG,
        lp: LP,
        attacked_fn: fn(board: &Board, square: Square, attacking_color: Color) -> bool,
    ) -> Self {
        let kernel = AlphaBetaKernel::new(tm, mg, lp, attacked_fn);

        Self { kernel }
    }
}
