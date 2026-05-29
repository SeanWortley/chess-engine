use crate::{
    Board, Color, Move, MoveGenerator, MoveList, NEG_INF, POS_INF, SearchResult, Square,
    TransitionManager,
    search::{
        LeafPolicy, OrderingPolicy, SearchCore,
        control::SearchControl,
        kernel::AlphaBetaKernel,
        metrics::SearchMetrics,
        zobrist::{SearchContext, TTEntry, TTFlag},
    },
};

pub struct AlphaBetaSearcher<
    TM: TransitionManager,
    MG: MoveGenerator,
    LP: LeafPolicy,
    OP: OrderingPolicy,
> {
    kernel: AlphaBetaKernel<TM, MG, LP, OP>,
}

impl<TM: TransitionManager, MG: MoveGenerator, LP: LeafPolicy, OP: OrderingPolicy> SearchCore
    for AlphaBetaSearcher<TM, MG, LP, OP>
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

        if let Some(table) = context.tt.as_ref() {
            if let Some(entry) = table.probe(board.hash(), depth) {
                if matches!(entry.flag, TTFlag::Exact) {
                    return SearchResult {
                        best_move: Some(entry.best_move),
                        score: entry.score,
                    };
                }
            }
        }

        let mut moves = MoveList::new();
        self.kernel.generate_moves(board, &mut moves);

        if moves.is_empty() {
            return SearchResult {
                best_move: None,
                score: self.kernel.terminal_score_if_no_moves(board),
            };
        }

        let mut alpha = NEG_INF;
        let beta = POS_INF;
        let mut best_score = i16::MIN;
        let mut best_move: Option<Move> = None;

        for mv in moves.iter() {
            if control.should_stop(metrics) {
                break;
            }

            self.kernel.make(board, *mv, context);
            let score = self
                .kernel
                .alpha_beta(
                    board,
                    -beta,
                    -alpha,
                    depth.saturating_sub(1),
                    control,
                    metrics,
                    context,
                )
                .saturating_neg();
            self.kernel.unmake(board, *mv);

            if score > best_score {
                best_score = score;
                best_move = Some(*mv);
            }
            if score > alpha {
                alpha = score;
            }
            if score >= beta {
                break;
            }
        }

        if !control.should_stop(metrics) {
            if let Some(table) = &mut context.tt {
                if let Some(best_move) = best_move {
                    let entry = TTEntry {
                        key: board.hash(),
                        score: best_score,
                        best_move,
                        depth,
                        flag: TTFlag::Exact,
                    };
                    table.store(entry);
                }
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
    AlphaBetaSearcher<TM, MG, LP, OP>
{
    pub fn new(
        tm: TM,
        mg: MG,
        lp: LP,
        op: OP,
        attacked_fn: fn(board: &Board, square: Square, attacking_color: Color) -> bool,
    ) -> Self {
        let kernel = AlphaBetaKernel::new(tm, mg, lp, op, attacked_fn);

        Self { kernel }
    }
}
