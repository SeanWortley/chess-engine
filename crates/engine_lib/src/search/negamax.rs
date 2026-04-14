use crate::{
    Board, Color, DRAW, Evaluator, IsAttackedFn, Move, MoveGenerator, MoveList, NEG_INF, PieceKind,
    SearchResult, Searcher, Square, TransitionManager,
};

pub struct PureNegamaxSearcher<TM: TransitionManager, MG: MoveGenerator, E: Evaluator> {
    tm: TM,
    mg: MG,
    e: E,
    max_depth: u8,
    attacked_fn: IsAttackedFn,
}

impl<TM: TransitionManager, MG: MoveGenerator, E: Evaluator> Searcher
    for PureNegamaxSearcher<TM, MG, E>
{
    // Root NegaMax function
    fn start_search(&mut self, board: &mut Board) -> SearchResult {
        let mut moves = MoveList::new();
        self.mg.generate_moves(board, &mut moves);

        let mut best_move: Option<Move> = None;
        let mut max = i16::MIN;
        let mut leaf_eval: i16;

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
                max = NEG_INF;
            } else {
                max = DRAW;
            };
        }
        for mv in moves.iter() {
            self.tm.make(board, *mv);
            leaf_eval = self
                .negamax_proper(board, self.max_depth.saturating_sub(1))
                .saturating_neg();
            if best_move.is_none() || leaf_eval > max {
                max = leaf_eval;
                best_move = Some(*mv);
            }

            self.tm.unmake(board, *mv);
        }

        SearchResult {
            best_move,
            score: max,
        }
    }

    fn make(&mut self, board: &mut Board, mv: Move) {
        self.tm.make(board, mv);
    }

    fn unmake(&mut self, board: &mut Board, mv: Move) {
        self.tm.unmake(board, mv);
    }
}

impl<TM: TransitionManager, MG: MoveGenerator, E: Evaluator> PureNegamaxSearcher<TM, MG, E> {
    pub fn new(
        tm: TM,
        mg: MG,
        e: E,
        max_depth: u8,
        attacked_fn: fn(board: &Board, square: Square, attacking_color: Color) -> bool,
    ) -> Self {
        PureNegamaxSearcher {
            tm,
            mg,
            e,
            max_depth,
            attacked_fn,
        }
    }

    fn negamax_proper(&mut self, board: &mut Board, depth: u8) -> i16 {
        if depth <= 0 {
            return self.e.evaluate(board);
        }
        let mut moves = MoveList::new();
        self.mg.generate_moves(board, &mut moves);

        let mut max = i16::MIN;

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
                max = NEG_INF;
            } else {
                max = DRAW;
            };
        }
        if board.material_pieces().inner() == 0 {
            return DRAW;
        }

        for mv in moves.iter() {
            self.tm.make(board, *mv);
            let leaf_eval = self.negamax_proper(board, depth - 1).saturating_neg();
            if leaf_eval > max {
                max = leaf_eval;
            }
            self.tm.unmake(board, *mv);
        }

        max
    }
}
