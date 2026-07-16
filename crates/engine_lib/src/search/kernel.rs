use crate::{
    Board, Color, DRAW, IsAttackedFn, Move, MoveGenerator, MoveList, NEG_INF, PieceKind,
    SearchControl, Square, TransitionManager,
    search::{
        LeafPolicy, OrderingPolicy,
        metrics::SearchMetrics,
        tt::{SearchContext, TTEntry, TTFlag, from_tt_score, to_tt_score},
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

fn terminal_score_if_no_moves(board: &Board, root_distance: u8, attacked_fn: IsAttackedFn) -> i16 {
    let king_square = Square::from_index(
        board
            .bitboard(board.to_move(), PieceKind::King)
            .lsb()
            .unwrap(),
    );
    let king_check = attacked_fn(board, king_square, board.to_move().opponent());
    if king_check {
        // Being mated further from the root is less bad: -29_999 is mate at
        // ply 1, -29_997 mate at ply 3. Negation up the tree makes the root
        // prefer the fastest mate and the defender the slowest.
        NEG_INF + root_distance as i16
    } else {
        DRAW
    }
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
        self.mg.generate_moves(board, moves, true);
    }

    pub fn terminal_score_if_no_moves(&self, board: &Board, root_distance: u8) -> i16 {
        terminal_score_if_no_moves(board, root_distance, self.attacked_fn)
    }

    pub fn alpha_beta(
        &mut self,
        board: &mut Board,
        alpha: i16,
        beta: i16,
        depth: u8,
        root_distance: u8,
        control: &SearchControl,
        metrics: &mut SearchMetrics,
        context: &mut SearchContext,
    ) -> i16 {
        metrics.increment();

        if context.is_draw_by_rule(board) {
            return DRAW;
        }

        if depth == 0 {
            return self.lp.evaluate_leaf(board, alpha, beta, control, metrics);
        }

        if let Some(table) = &context.tt {
            if let Some(entry) = table.probe(board.hash(), depth) {
                match entry.flag {
                    TTFlag::Exact => return from_tt_score(entry.score, root_distance),
                    TTFlag::LowerBound if entry.score >= beta => {
                        return from_tt_score(entry.score, root_distance);
                    }
                    TTFlag::UpperBound if entry.score <= alpha => {
                        return from_tt_score(entry.score, root_distance);
                    }
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
            best_score = self.terminal_score_if_no_moves(board, root_distance);
        }

        for mv in moves.iter() {
            if control.should_stop(metrics) {
                break;
            }

            self.tm.make(board, *mv);
            context.history.push(board.hash());
            let score = self
                .alpha_beta(
                    board,
                    -beta,
                    -alpha,
                    depth - 1,
                    root_distance + 1,
                    control,
                    metrics,
                    context,
                )
                .saturating_neg();

            context.history.pop();
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
                        score: to_tt_score(best_score, root_distance),
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
        self.mg.generate_moves(board, moves, true);
    }

    pub fn terminal_score_if_no_moves(&self, board: &Board, root_distance: u8) -> i16 {
        terminal_score_if_no_moves(board, root_distance, self.attacked_fn)
    }

    pub fn negamax(
        &mut self,
        board: &mut Board,
        depth: u8,
        root_distance: u8,
        control: &SearchControl,
        metrics: &mut SearchMetrics,
        context: &mut SearchContext,
    ) -> i16 {
        metrics.increment();

        if context.is_draw_by_rule(board) {
            return DRAW;
        }

        if depth == 0 {
            return self
                .lp
                .evaluate_leaf(board, NEG_INF, i16::MAX, control, metrics);
        }

        if let Some(table) = &context.tt {
            if let Some(entry) = table.probe(board.hash(), depth) {
                return from_tt_score(entry.score, root_distance);
            }
        }

        let mut moves = MoveList::new();
        self.generate_moves(board, &mut moves);
        self.op.order_moves(board, &mut moves);

        let mut best_score = i16::MIN;
        let mut best_move: Option<Move> = None;

        // Check for checkmate or stalemate.
        if moves.is_empty() {
            best_score = self.terminal_score_if_no_moves(board, root_distance);
        }

        for mv in moves.iter() {
            if control.should_stop(metrics) {
                break;
            }

            self.tm.make(board, *mv);
            context.history.push(board.hash());
            let score = self
                .negamax(
                    board,
                    depth - 1,
                    root_distance + 1,
                    control,
                    metrics,
                    context,
                )
                .saturating_neg();

            context.history.pop();
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
                        score: to_tt_score(best_score, root_distance),
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
