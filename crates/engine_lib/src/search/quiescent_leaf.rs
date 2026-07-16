use crate::{
    Board, Evaluator, MoveGenerator, MoveList, SearchControl, TransitionManager,
    search::{LeafPolicy, OrderingPolicy, metrics::SearchMetrics},
};

pub struct QuiescentLeaf<TM: TransitionManager, MG: MoveGenerator, E: Evaluator, OP: OrderingPolicy>
{
    tm: TM,
    mg: MG,
    e: E,
    op: OP,
}

impl<TM: TransitionManager, MG: MoveGenerator, E: Evaluator, OP: OrderingPolicy>
    QuiescentLeaf<TM, MG, E, OP>
{
    pub fn new(tm: TM, mg: MG, e: E, op: OP) -> Self {
        Self { tm, mg, e, op }
    }

    pub fn quiesce(
        &mut self,
        board: &mut Board,
        alpha: i16,
        beta: i16,
        control: &SearchControl,
        metrics: &mut SearchMetrics,
    ) -> i16 {
        metrics.increment();

        let static_eval = self.e.evaluate(board);
        let mut alpha = alpha;

        // Stand pat
        let mut best_score = static_eval;
        if best_score >= beta {
            return best_score;
        }
        if best_score > alpha {
            alpha = best_score
        }

        let mut captures = MoveList::new();
        self.mg.generate_captures_only(board, &mut captures);
        self.op.order_moves(board, &mut captures);

        for capture in captures.iter() {
            if control.should_stop(metrics) {
                break;
            }
            self.tm.make(board, *capture);
            let score = self
                .quiesce(board, -beta, -alpha, control, metrics)
                .saturating_neg();
            self.tm.unmake(board, *capture);

            if score >= beta {
                return score;
            }
            if score > best_score {
                best_score = score;
            }
            if score > alpha {
                alpha = score;
            }
        }

        best_score
    }
}

impl<TM: TransitionManager, MG: MoveGenerator, E: Evaluator, OP: OrderingPolicy> LeafPolicy
    for QuiescentLeaf<TM, MG, E, OP>
{
    fn evaluate_leaf(
        &mut self,
        board: &mut Board,
        alpha: i16,
        beta: i16,
        control: &SearchControl,
        metrics: &mut SearchMetrics,
    ) -> i16 {
        self.quiesce(board, alpha, beta, control, metrics)
    }
}
