use crate::{
    Board, Color, Move, MoveGenerator, MoveList, SearchResult, Square, TransitionManager,
    search::{
        LeafPolicy, OrderingPolicy, SearchCore, control::SearchControl, kernel::PureNegamaxKernel,
        metrics::SearchMetrics, zobrist::SearchContext,
    },
};

pub struct PureNegamaxSearcher<
    TM: TransitionManager,
    MG: MoveGenerator,
    LP: LeafPolicy,
    OP: OrderingPolicy,
> {
    kernel: PureNegamaxKernel<TM, MG, LP, OP>,
}

impl<TM: TransitionManager, MG: MoveGenerator, LP: LeafPolicy, OP: OrderingPolicy> SearchCore
    for PureNegamaxSearcher<TM, MG, LP, OP>
{
    fn search_at_depth(
        &mut self,
        board: &mut Board,
        context: &mut SearchContext,
        depth: u8,
        control: &SearchControl,
        metrics: &mut SearchMetrics,
    ) -> SearchResult {
        metrics.increment();

        let mut moves = MoveList::new();
        self.kernel.generate_moves(board, &mut moves);

        if moves.is_empty() {
            return SearchResult {
                best_move: None,
                score: self.kernel.terminal_score_if_no_moves(board),
            };
        }

        let mut best_score = i16::MIN;
        let mut best_move: Option<Move> = None;

        for mv in moves.iter() {
            if control.should_stop(metrics) {
                break;
            }

                self.kernel.make(board, *mv, context);
            let score = self
                .kernel
                .negamax(board, depth.saturating_sub(1), control, metrics, context)
                .saturating_neg();
            self.kernel.unmake(board, *mv);

            if score > best_score {
                best_score = score;
                best_move = Some(*mv);
            }
        }

        SearchResult {
            best_move,
            score: best_score,
        }
    }

    fn make(&mut self, board: &mut Board, mv: Move) {
        let mut context = SearchContext::without_tt();
        self.kernel.make(board, mv, &mut context);
    }

    fn unmake(&mut self, board: &mut Board, mv: Move) {
        self.kernel.unmake(board, mv);
    }
}

impl<TM: TransitionManager, MG: MoveGenerator, LP: LeafPolicy, OP: OrderingPolicy>
    PureNegamaxSearcher<TM, MG, LP, OP>
{
    pub fn new(
        tm: TM,
        mg: MG,
        lp: LP,
        op: OP,
        attacked_fn: fn(board: &Board, square: Square, attacking_color: Color) -> bool,
    ) -> Self {
        let kernel = PureNegamaxKernel::new(tm, mg, lp, op, attacked_fn);

        Self { kernel }
    }
}
