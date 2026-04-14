use crate::{Board, Evaluator, PieceKind};

pub struct MaterialEvaluator;

const PIECE_VALUES: [i16; 5] = [100, 300, 325, 500, 900];

impl MaterialEvaluator {
    pub fn new() -> Self {
        MaterialEvaluator
    }
}

impl Evaluator for MaterialEvaluator {
    fn evaluate(&self, board: &Board) -> i16 {
        let kinds = [
            PieceKind::Pawn,
            PieceKind::Knight,
            PieceKind::Bishop,
            PieceKind::Rook,
            PieceKind::Queen,
        ];

        let mut evaluation: i16 = 0;
        for kind in kinds {
            // Increment for friendly pieces
            evaluation +=
                board.bitboard(board.to_move(), kind).count() as i16 * PIECE_VALUES[kind as usize];

            // Decrement for enemy pieces
            evaluation -= board.bitboard(board.to_move().opponent(), kind).count() as i16
                * PIECE_VALUES[kind as usize];
        }
        evaluation
    }
}
