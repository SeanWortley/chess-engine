use crate::{
    Board, Color, DRAW, IsAttackedFn, Move, MoveGenerator, MoveList, NEG_INF, POS_INF, PieceKind,
    SearchResult, Searcher, Square, TransitionManager,
    search::{
        LeafPolicy,
        control::{SearchConstraint, SearchControl},
        kernel::AlphaBetaKernel,
    },
};
const DEFAULT_NEGAMAX_DEPTH: u8 = 4;

pub struct PureNegamaxSearcher<TM: TransitionManager, MG: MoveGenerator, LP: LeafPolicy> {
    kernel: AlphaBetaKernel<TM, MG, LP>,
}

impl<TM: TransitionManager, MG: MoveGenerator, LP: LeafPolicy> Searcher
    for PureNegamaxSearcher<TM, MG, LP>
{
    // Root NegaMax function
    fn start_search(
        &mut self,
        board: &mut Board,
        constraint: SearchConstraint,
        control: &SearchControl,
    ) -> SearchResult {
        let depth_limit = constraint.max_depth.unwrap_or(DEFAULT_NEGAMAX_DEPTH);
        self.kernel
            .search(board, NEG_INF, POS_INF, depth_limit, control)
    }

    fn make(&mut self, board: &mut Board, mv: Move) {
        self.kernel.make(board, mv);
    }

    fn unmake(&mut self, board: &mut Board, mv: Move) {
        self.kernel.unmake(board, mv);
    }
}

impl<TM: TransitionManager, MG: MoveGenerator, LP: LeafPolicy> PureNegamaxSearcher<TM, MG, E> {
    pub fn new(
        tm: TM,
        mg: MG,
        e: E,
        attacked_fn: fn(board: &Board, square: Square, attacking_color: Color) -> bool,
    ) -> Self {
        PureNegamaxSearcher {
            tm,
            mg,
            e,
            attacked_fn,
        }
    }

    pub fn search_at_depth(
        &mut self,
        board: &mut Board,
        depth_limit: u8,
        control: &SearchControl,
    ) -> SearchResult {
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
            if control.should_stop() {
                break;
            }

            self.tm.make(board, *mv);
            leaf_eval = self
                .negamax_proper(board, depth_limit.saturating_sub(1), control)
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

    // Called by root search, and other root search algs
    pub fn negamax_proper(&mut self, board: &mut Board, depth: u8, control: &SearchControl) -> i16 {
        if control.should_stop() {
            return self.e.evaluate(board);
        }

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
            if control.should_stop() {
                break;
            }

            self.tm.make(board, *mv);
            let leaf_eval = self
                .negamax_proper(board, depth - 1, control)
                .saturating_neg();
            if leaf_eval > max {
                max = leaf_eval;
            }
            self.tm.unmake(board, *mv);
        }

        max
    }
}
