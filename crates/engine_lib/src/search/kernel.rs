use crate::{
    Board, Color, DRAW, IsAttackedFn, Move, MoveGenerator, MoveList, NEG_INF, PieceKind,
    SearchControl, Square, TransitionManager,
    search::{LeafPolicy, OrderingPolicy, metrics::SearchMetrics},
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
    ) -> i16 {
        metrics.increment();

        if depth == 0 {
            return self.lp.evaluate_leaf(board, alpha, beta);
        }

        let mut moves = MoveList::new();
        self.generate_moves(board, &mut moves);

        let mut alpha = alpha;
        let mut best_score = i16::MIN;

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
                .alpha_beta(board, -beta, -alpha, depth - 1, control, metrics)
                .saturating_neg();
            self.tm.unmake(board, *mv);

            if score > best_score {
                best_score = score;
            }
            if score > alpha {
                alpha = score;
            }
            if alpha >= beta {
                break;
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
    ) -> i16 {
        metrics.increment();

        if depth == 0 {
            return self.lp.evaluate_leaf(board, NEG_INF, i16::MAX);
        }

        let mut moves = MoveList::new();
        self.generate_moves(board, &mut moves);

        let mut best_score = i16::MIN;

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
                .negamax(board, depth - 1, control, metrics)
                .saturating_neg();
            self.tm.unmake(board, *mv);

            if score > best_score {
                best_score = score;
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
