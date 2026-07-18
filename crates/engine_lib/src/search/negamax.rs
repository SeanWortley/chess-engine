use crate::{
    Board, Color, Move, MoveGenerator, MoveList, SearchResult, Square, TransitionManager,
    move_ordering::OrderingPolicy,
    search::{
        LeafPolicy, SearchCore,
        control::SearchControl,
        kernel::PureNegamaxKernel,
        metrics::SearchMetrics,
        tt::{SearchContext, TTEntry, TTFlag, from_tt_score, to_tt_score},
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

        if let Some(table) = &context.tt {
            if let Some(entry) = table.probe(board.hash(), depth) {
                return SearchResult {
                    best_move: Some(entry.best_move),
                    score: from_tt_score(entry.score, 0),
                };
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

        let mut best_score = i16::MIN;
        let mut best_move: Option<Move> = None;

        for mv in moves.iter() {
            self.kernel.make(board, *mv);
            context.history.push(board.hash());
            let score = self
                .kernel
                .negamax(board, depth.saturating_sub(1), 1, control, metrics, context)
                .saturating_neg();
            context.history.pop();
            self.kernel.unmake(board, *mv);

            if score > best_score {
                best_score = score;
                best_move = Some(*mv);
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
