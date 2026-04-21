use crate::{
    Board, Color, DRAW, IsAttackedFn, Move, MoveGenerator, MoveList, NEG_INF, PieceKind,
    SearchControl, SearchResult, Square, TransitionManager, search::LeafPolicy,
};

pub struct AlphaBetaKernel<TM: TransitionManager, MG: MoveGenerator, LP: LeafPolicy> {
    tm: TM,
    mg: MG,
    lp: LP,
    attacked_fn: IsAttackedFn,
}

pub struct PureNegamaxKernel<TM: TransitionManager, MG: MoveGenerator, LP: LeafPolicy> {
    tm: TM,
    mg: MG,
    lp: LP,
    attacked_fn: IsAttackedFn,
}

impl<TM: TransitionManager, MG: MoveGenerator, LP: LeafPolicy> AlphaBetaKernel<TM, MG, LP> {
    pub fn new(
        tm: TM,
        mg: MG,
        lp: LP,
        attacked_fn: fn(board: &Board, square: Square, attacking_color: Color) -> bool,
    ) -> Self {
        AlphaBetaKernel {
            tm,
            mg,
            lp,
            attacked_fn,
        }
    }

    pub fn search(
        &mut self,
        board: &mut Board,
        alpha: i16,
        beta: i16,
        depth: u8,
        control: &SearchControl,
    ) -> SearchResult {
        let mut moves = MoveList::new();
        self.mg.generate_moves(board, &mut moves);

        let mut local_alpha = alpha;
        let mut best_score = i16::MIN;
        let mut best_move: Option<Move> = None;

        // Check for checkmate or stalemate
        if moves.is_empty() {
            let king_square = Square::from_index(
                board
                    .bitboard(board.to_move(), PieceKind::King)
                    .lsb()
                    .unwrap(),
            );
            let king_check = (self.attacked_fn)(board, king_square, board.to_move().opponent());
            if king_check {
                best_score = NEG_INF;
            } else {
                best_score = DRAW;
            }
        }

        for mv in moves.iter() {
            if control.should_stop() {
                break;
            }

            self.tm.make(board, *mv);
            let score = self
                .alpha_beta(board, -beta, -local_alpha, depth - 1, control)
                .saturating_neg();
            self.tm.unmake(board, *mv);

            if score > best_score {
                best_score = score;
                best_move = Some(*mv);
            }
            if score > local_alpha {
                local_alpha = score;
            }
            if score >= beta {
                break;
            }
        }

        SearchResult {
            best_move,
            score: best_score,
        }
    }

    fn alpha_beta(
        &mut self,
        board: &mut Board,
        alpha: i16,
        beta: i16,
        depth: u8,
        control: &SearchControl,
    ) -> i16 {
        if depth == 0 {
            return self.lp.evaluate_leaf(board, alpha, beta);
        }

        let mut moves = MoveList::new();
        self.mg.generate_moves(board, &mut moves);

        let mut alpha = alpha;
        let mut best_score = i16::MIN;

        // Check for checkmate or stalemate
        if moves.is_empty() {
            let king_square = Square::from_index(
                board
                    .bitboard(board.to_move(), PieceKind::King)
                    .lsb()
                    .unwrap(),
            );
            let king_check = (self.attacked_fn)(board, king_square, board.to_move().opponent());
            if king_check {
                best_score = NEG_INF;
            } else {
                best_score = DRAW;
            }
        }

        for mv in moves.iter() {
            if control.should_stop() {
                break;
            }

            self.tm.make(board, *mv);
            let score = self
                .alpha_beta(board, -beta, -alpha, depth - 1, control)
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

impl<TM: TransitionManager, MG: MoveGenerator, LP: LeafPolicy> PureNegamaxKernel<TM, MG, LP> {
    pub fn new(
        tm: TM,
        mg: MG,
        lp: LP,
        attacked_fn: fn(board: &Board, square: Square, attacking_color: Color) -> bool,
    ) -> Self {
        PureNegamaxKernel {
            tm,
            mg,
            lp,
            attacked_fn,
        }
    }

    pub fn search(
        &mut self,
        board: &mut Board,
        depth: u8,
        control: &SearchControl,
    ) -> SearchResult {
        let mut moves = MoveList::new();
        self.mg.generate_moves(board, &mut moves);

        let mut best_score = i16::MIN;
        let mut best_move: Option<Move> = None;

        // Check for checkmate or stalemate.
        if moves.is_empty() {
            let king_square = Square::from_index(
                board
                    .bitboard(board.to_move(), PieceKind::King)
                    .lsb()
                    .unwrap(),
            );
            let king_check = (self.attacked_fn)(board, king_square, board.to_move().opponent());
            best_score = if king_check { NEG_INF } else { DRAW };
        }

        for mv in moves.iter() {
            if control.should_stop() {
                break;
            }

            self.tm.make(board, *mv);
            let score = self
                .negamax(board, depth.saturating_sub(1), control)
                .saturating_neg();
            self.tm.unmake(board, *mv);

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

    fn negamax(&mut self, board: &mut Board, depth: u8, control: &SearchControl) -> i16 {
        if depth == 0 {
            return self.lp.evaluate_leaf(board, NEG_INF, i16::MAX);
        }

        let mut moves = MoveList::new();
        self.mg.generate_moves(board, &mut moves);

        let mut best_score = i16::MIN;

        // Check for checkmate or stalemate.
        if moves.is_empty() {
            let king_square = Square::from_index(
                board
                    .bitboard(board.to_move(), PieceKind::King)
                    .lsb()
                    .unwrap(),
            );
            let king_check = (self.attacked_fn)(board, king_square, board.to_move().opponent());
            best_score = if king_check { NEG_INF } else { DRAW };
        }

        for mv in moves.iter() {
            if control.should_stop() {
                break;
            }

            self.tm.make(board, *mv);
            let score = self.negamax(board, depth - 1, control).saturating_neg();
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
