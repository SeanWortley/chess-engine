use crate::core::board::Board;
use crate::core::move_list::MoveList;
use crate::move_gen::attacks::*;
use crate::{move_gen::MoveGenerator, transition::TransitionManager};

pub struct NaiveMoveGenerator<TM: TransitionManager> {
    tm: TM,
}

impl<TM: TransitionManager> MoveGenerator for NaiveMoveGenerator<TM> {
    fn generate_moves(&self, board: &mut Board, moves: &mut MoveList) {
        let mut pseudo = MoveList::new();
        self.generate_pseudo_legal(board, &mut pseudo);

        for mv in pseudo.iter() {
            let copy = self.tm.make(board, *mv);
            if !Attacks::side_to_move_gives_check(copy.as_ref(), Algorithm::Naive) {
                moves.push(*mv);
            }
        }
    }
}

impl<TM: TransitionManager> NaiveMoveGenerator<TM> {
    pub fn new(tm: TM) -> Self {
        NaiveMoveGenerator { tm }
    }
    pub fn generate_pseudo_legal(&self, board: &Board, moves: &mut MoveList) {
        self.generate_pawn_moves(board, moves);
        self.generate_knight_moves(board, moves);
        self.generate_bishop_moves(board, moves);
        self.generate_rook_moves(board, moves);
        self.generate_queen_moves(board, moves);
        self.generate_king_moves(board, moves);
    }

    fn generate_pawn_moves(&self, board: &Board, moves: &mut MoveList) {}
    fn generate_knight_moves(&self, board: &Board, moves: &mut MoveList) {}
    fn generate_bishop_moves(&self, board: &Board, moves: &mut MoveList) {}
    fn generate_rook_moves(&self, board: &Board, moves: &mut MoveList) {}
    fn generate_queen_moves(&self, board: &Board, moves: &mut MoveList) {}
    fn generate_king_moves(&self, board: &Board, moves: &mut MoveList) {}
}
