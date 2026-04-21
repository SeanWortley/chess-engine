use crate::{
    Board, Color, Move, MoveGenerator, SearchResult, Searcher, Square, TransitionManager,
    search::{
        LeafPolicy, SearchCore,
        control::{SearchConstraint, SearchControl},
        kernel::PureNegamaxKernel,
    },
};
const DEFAULT_NEGAMAX_DEPTH: u8 = 4;

pub struct PureNegamaxSearcher<TM: TransitionManager, MG: MoveGenerator, LP: LeafPolicy> {
    kernel: PureNegamaxKernel<TM, MG, LP>,
}

impl<TM: TransitionManager, MG: MoveGenerator, LP: LeafPolicy> SearchCore
    for PureNegamaxSearcher<TM, MG, LP>
{
    fn search_at_depth(
        &mut self,
        board: &mut Board,
        depth: u8,
        control: &SearchControl,
    ) -> SearchResult {
        self.kernel.search(board, depth, control)
    }

    fn make(&mut self, board: &mut Board, mv: Move) {
        self.kernel.make(board, mv);
    }

    fn unmake(&mut self, board: &mut Board, mv: Move) {
        self.kernel.unmake(board, mv);
    }
}

impl<TM: TransitionManager, MG: MoveGenerator, LP: LeafPolicy> Searcher
    for PureNegamaxSearcher<TM, MG, LP>
{
    fn start_search(
        &mut self,
        board: &mut Board,
        constraint: SearchConstraint,
        control: &SearchControl,
    ) -> SearchResult {
        let depth = constraint.max_depth.unwrap_or(DEFAULT_NEGAMAX_DEPTH);
        self.search_at_depth(board, depth, control)
    }

    fn make(&mut self, board: &mut Board, mv: Move) {
        self.kernel.make(board, mv);
    }

    fn unmake(&mut self, board: &mut Board, mv: Move) {
        self.kernel.unmake(board, mv);
    }
}

impl<TM: TransitionManager, MG: MoveGenerator, LP: LeafPolicy> PureNegamaxSearcher<TM, MG, LP> {
    pub fn new(
        tm: TM,
        mg: MG,
        lp: LP,
        attacked_fn: fn(board: &Board, square: Square, attacking_color: Color) -> bool,
    ) -> Self {
        let kernel = PureNegamaxKernel::new(tm, mg, lp, attacked_fn);

        Self { kernel }
    }
}
