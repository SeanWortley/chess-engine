use crate::{
    Board, Color, DRAW, IsAttackedFn, Move, MoveGenerator, MoveList, NEG_INF, PieceKind,
    SearchControl, Square, TransitionManager,
    search::{
        LeafPolicy, OrderingPolicy,
        metrics::SearchMetrics,
        tt::{SearchContext, TTEntry, TTFlag},
    },
};

pub struct AlphaBetaKernel<
    TM: TransitionManager,
    MG: MoveGenerator,
    LP: LeafPolicy,
    OP: OrderingPolicy,
> {
    tm: TM,
    mg: MG,
    lp: LP,
    op: OP,
    attacked_fn: IsAttackedFn,
}

pub struct PureNegamaxKernel<
    TM: TransitionManager,
    MG: MoveGenerator,
    LP: LeafPolicy,
    OP: OrderingPolicy,
> {
    tm: TM,
    mg: MG,
    lp: LP,
    op: OP,
    attacked_fn: IsAttackedFn,
}

fn terminal_score_if_no_moves(board: &Board, attacked_fn: IsAttackedFn) -> i16 {
    let king_square = Square::from_index(
        board
            .bitboard(board.to_move(), PieceKind::King)
            .lsb()
            .unwrap(),
    );
    let king_check = attacked_fn(board, king_square, board.to_move().opponent());
    if king_check { NEG_INF } else { DRAW }
}

impl<TM: TransitionManager, MG: MoveGenerator, LP: LeafPolicy, OP: OrderingPolicy>
    AlphaBetaKernel<TM, MG, LP, OP>
{
    pub fn new(
        tm: TM,
        mg: MG,
        lp: LP,
        op: OP,
        attacked_fn: fn(board: &Board, square: Square, attacking_color: Color) -> bool,
    ) -> Self {
        AlphaBetaKernel {
            tm,
            mg,
            lp,
            op,
            attacked_fn,
        }
    }

    pub fn generate_moves(&mut self, board: &mut Board, moves: &mut MoveList) {
        self.mg.generate_moves(board, moves);
    }

    pub fn terminal_score_if_no_moves(&self, board: &Board) -> i16 {
        terminal_score_if_no_moves(board, self.attacked_fn)
    }

    pub fn alpha_beta(
        &mut self,
        board: &mut Board,
        alpha: i16,
        beta: i16,
        depth: u8,
        control: &SearchControl,
        metrics: &mut SearchMetrics,
        context: &mut SearchContext,
    ) -> i16 {
        metrics.increment();

        if depth == 0 {
            return self.lp.evaluate_leaf(board, alpha, beta);
        }

        if let Some(table) = &context.tt {
            if let Some(entry) = table.probe(board.hash(), depth) {
                match entry.flag {
                    TTFlag::Exact => return entry.score,
                    TTFlag::LowerBound if entry.score >= beta => return entry.score,
                    TTFlag::UpperBound if entry.score <= alpha => return entry.score,
                    _ => {}
                }
            }
        }

        let mut moves = MoveList::new();
        self.generate_moves(board, &mut moves);
        self.op.order_moves(board, &mut moves);

        let original_alpha = alpha;
        let mut alpha = alpha;
        let mut best_score = i16::MIN;
        let mut best_move: Option<Move> = None;

        // Check for checkmate or stalemate
        if moves.is_empty() {
            best_score = self.terminal_score_if_no_moves(board);
        }

        for mv in moves.iter() {
            if control.should_stop(metrics) {
                break;
            }

            self.tm.make(board, *mv);
            let score = self
                .alpha_beta(board, -beta, -alpha, depth - 1, control, metrics, context)
                .saturating_neg();

            self.tm.unmake(board, *mv);

            if score > best_score {
                best_score = score;
                best_move = Some(*mv);
            }
            if score > alpha {
                alpha = score;
            }
            if alpha >= beta {
                break;
            }
        }

        if !control.should_stop(metrics) {
            if let Some(table) = &mut context.tt {
                if let Some(best_move) = best_move {
                    let flag = if best_score <= original_alpha {
                        // No move improved alpha; this is an upper bound
                        TTFlag::UpperBound
                    } else if best_score >= beta {
                        // Beta cutoff; this is a lower bound
                        TTFlag::LowerBound
                    } else {
                        // Move improved alpha but didn't cause cutoff; exact value
                        TTFlag::Exact
                    };

                    let entry = TTEntry {
                        key: board.hash(),
                        score: best_score,
                        best_move,
                        depth,
                        flag,
                    };
                    table.store(entry);
                }
            }
        }
        best_score
    }

    pub fn make(&mut self, board: &mut Board, mv: Move) {
        self.tm.make(board, mv);
    }

    pub fn unmake(&mut self, board: &mut Board, mv: Move) {
        self.tm.unmake(board, mv);
    }
}

impl<TM: TransitionManager, MG: MoveGenerator, LP: LeafPolicy, OP: OrderingPolicy>
    PureNegamaxKernel<TM, MG, LP, OP>
{
    pub fn new(
        tm: TM,
        mg: MG,
        lp: LP,
        op: OP,
        attacked_fn: fn(board: &Board, square: Square, attacking_color: Color) -> bool,
    ) -> Self {
        PureNegamaxKernel {
            tm,
            mg,
            lp,
            op,
            attacked_fn,
        }
    }

    pub fn generate_moves(&mut self, board: &mut Board, moves: &mut MoveList) {
        self.mg.generate_moves(board, moves);
    }

    pub fn terminal_score_if_no_moves(&self, board: &Board) -> i16 {
        terminal_score_if_no_moves(board, self.attacked_fn)
    }

    pub fn negamax(
        &mut self,
        board: &mut Board,
        depth: u8,
        control: &SearchControl,
        metrics: &mut SearchMetrics,
        context: &mut SearchContext,
    ) -> i16 {
        metrics.increment();

        if depth == 0 {
            return self.lp.evaluate_leaf(board, NEG_INF, i16::MAX);
        }

        if let Some(table) = &context.tt {
            if let Some(entry) = table.probe(board.hash(), depth) {
                return entry.score;
            }
        }

        let mut moves = MoveList::new();
        self.generate_moves(board, &mut moves);
        self.op.order_moves(board, &mut moves);

        let mut best_score = i16::MIN;
        let mut best_move: Option<Move> = None;

        // Check for checkmate or stalemate.
        if moves.is_empty() {
            best_score = self.terminal_score_if_no_moves(board);
        }

        for mv in moves.iter() {
            if control.should_stop(metrics) {
                break;
            }

            self.tm.make(board, *mv);
            let score = self
                .negamax(board, depth - 1, control, metrics, context)
                .saturating_neg();
            self.tm.unmake(board, *mv);

            if score > best_score {
                best_score = score;
                best_move = Some(*mv);
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

        best_score
    }

    pub fn make(&mut self, board: &mut Board, mv: Move) {
        self.tm.make(board, mv);
    }

    pub fn unmake(&mut self, board: &mut Board, mv: Move) {
        self.tm.unmake(board, mv);
    }
}
