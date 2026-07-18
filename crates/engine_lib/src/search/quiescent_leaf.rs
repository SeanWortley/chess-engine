use crate::{
    Board, Evaluator, MoveGenerator, MoveList, SearchControl, TransitionManager,
    move_ordering::OrderingPolicy,
    search::{LeafPolicy, metrics::SearchMetrics},
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        CopyMakeTransition, NEG_INF, NaiveMoveGenerator, POS_INF,
        eval::pesto::PestoEvaluator,
        move_gen::attacks::ray_is_attacked,
        move_ordering::MvvLva,
        search::control::SearchConstraint,
    };

    // Returns (static_eval, quiescence_eval) for the side to move.
    fn evaluate(fen: &str) -> (i16, i16) {
        let mut board = Board::from_fen(fen);
        let static_eval = PestoEvaluator::new().evaluate(&board);

        let mut leaf = QuiescentLeaf::new(
            CopyMakeTransition::new(),
            NaiveMoveGenerator::new(CopyMakeTransition::new(), ray_is_attacked),
            PestoEvaluator::new(),
            MvvLva {},
        );
        let control = SearchControl::new(SearchConstraint::fixed_depth(10));
        let mut metrics = SearchMetrics::new();
        let quiesce_eval =
            leaf.evaluate_leaf(&mut board, NEG_INF, POS_INF, &control, &mut metrics);

        (static_eval, quiesce_eval)
    }

    #[test]
    fn test_sees_hanging_queen_beyond_static_eval() {
        // Black to move; white queen on d5 is capturable by the e6 pawn.
        // Static eval thinks black is down a queen; quiescence plays exd5
        // and sees black is actually up a pawn.
        let (static_eval, quiesce_eval) = evaluate("4k3/8/4p3/3Q4/8/8/8/4K3 b - - 0 1");

        assert!(
            static_eval < -400,
            "static eval should see black down a queen, got {static_eval}"
        );
        assert!(
            quiesce_eval > 0,
            "quiescence should see black winning the queen, got {quiesce_eval}"
        );
    }

    #[test]
    fn test_stands_pat_instead_of_losing_capture() {
        // White to move, up a queen. The only capture is Qxe5, which loses
        // the queen to fxe5. Quiescence must decline it and return exactly
        // the stand-pat (static) score.
        let (static_eval, quiesce_eval) = evaluate("7k/8/5p2/4p3/3Q4/8/8/K7 w - - 0 1");

        assert!(
            static_eval > 400,
            "static eval should see white up a queen, got {static_eval}"
        );
        assert_eq!(
            quiesce_eval, static_eval,
            "no capture improves the position; quiescence must stand pat"
        );
    }

    #[test]
    fn test_quiet_position_returns_static_eval() {
        // No captures exist; quiescence must equal the static eval exactly.
        let (static_eval, quiesce_eval) = evaluate("4k3/8/8/8/8/8/8/4K3 w - - 0 1");
        assert_eq!(quiesce_eval, static_eval);
    }
}
