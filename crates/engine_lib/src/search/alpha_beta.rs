use crate::{
    Board, Color, Move, MoveGenerator, MoveList, NEG_INF, POS_INF, SearchResult, Square,
    TransitionManager,
    move_ordering::{OrderingContext, OrderingPolicy},
    search::{
        LeafPolicy, SearchCore,
        control::SearchControl,
        kernel::AlphaBetaKernel,
        metrics::SearchMetrics,
        tt::{SearchContext, TTEntry, TTFlag, from_tt_score, to_tt_score},
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
                        score: from_tt_score(entry.score, 0),
                    };
                }
            }
        }

        let mut moves = MoveList::new();
        self.kernel.generate_moves(board, &mut moves);

        if moves.is_empty() {
            return SearchResult {
                best_move: None,
                score: self.kernel.terminal_score_if_no_moves(board, 0),
            };
        }

        let tt_move = context.tt.as_ref().and_then(|t| t.probe_move(board.hash()));
        self.kernel
            .order_moves(board, &mut moves, OrderingContext::new(tt_move, 0));

        let mut alpha = NEG_INF;
        let beta = POS_INF;
        let mut best_score = i16::MIN;
        let mut best_move: Option<Move> = None;

        for mv in moves.iter() {
            self.kernel.make(board, *mv);
            context.history.push(board.hash());
            let score = self
                .kernel
                .alpha_beta(
                    board,
                    -beta,
                    -alpha,
                    depth.saturating_sub(1),
                    1,
                    control,
                    metrics,
                    context,
                )
                .saturating_neg();
            context.history.pop();
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

            if control.should_stop(metrics) {
                break;
            }
        }

        if !control.should_stop(metrics) {
            if let Some(table) = &mut context.tt {
                if let Some(best_move) = best_move {
                    let entry = TTEntry {
                        key: board.hash(),
                        score: to_tt_score(best_score, 0),
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
        self.kernel.make(board, mv);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        CopyMakeTransition, NaiveMoveGenerator, POS_INF,
        eval::pesto::PestoEvaluator,
        move_gen::attacks::ray_is_attacked,
        move_ordering::MvvLva,
        search::{control::SearchConstraint, metrics::SearchMetrics, static_leaf::StaticLeaf},
    };

    #[test]
    fn test_picks_fastest_mate() {
        // Back rank: Ra8# is mate in 1; slower rook mates also exist.
        let mut board = Board::from_fen("6k1/5ppp/8/8/8/8/5PPP/R5K1 w - - 0 1");

        let mut searcher = AlphaBetaSearcher::new(
            CopyMakeTransition::new(),
            NaiveMoveGenerator::new(CopyMakeTransition::new(), ray_is_attacked),
            StaticLeaf::new(PestoEvaluator::new()),
            MvvLva {},
            ray_is_attacked,
        );
        let mut context = SearchContext::with_tt(16);
        context.history.push(board.hash());
        let control = SearchControl::new(SearchConstraint::fixed_depth(4));
        let mut metrics = SearchMetrics::new();

        let result = searcher.search_at_depth(&mut board, &mut context, 4, &control, &mut metrics);

        // Mate delivered at ply 1 scores exactly POS_INF - 1; anything else
        // means the mate-distance gradient is missing or inverted.
        assert_eq!(result.score, POS_INF - 1);
        let mv = result.best_move.expect("search must return a move");
        assert_eq!(mv.origin(), Square::from_name("a1"));
        assert_eq!(mv.destination(), Square::from_name("a8"));
    }
}
