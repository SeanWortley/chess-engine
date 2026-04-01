use crate::{
    core::{board::*, move_list::*, square::*},
    move_gen::naive::NaiveMoveGenerator,
    transition::copy_make::CopyMakeTransition,
};

pub struct Attacks {}

pub enum Algorithm {
    Naive,
    Bitboards,
}

impl Attacks {
    pub fn side_to_move_gives_check(board: &Board, _algorithm: Algorithm) -> bool {
        Attacks::is_naive_gives_check(board)
    }

    // Generates pseudo-legal moves for board.to_move() and checks whether any capture the enemy king.
    fn is_naive_gives_check(board: &Board) -> bool {
        let yucky_generator = NaiveMoveGenerator::new(CopyMakeTransition);
        let mut moves = MoveList::new();

        yucky_generator.generate_pseudo_legal(board, &mut moves);

        for mv in moves.iter() {
            let target_square = Square::from_index(mv.destination());
            let target_piece = board.get_piece(target_square);

            if let Some(target_piece) = target_piece {
                if mv.is_capture() && target_piece.kind == PieceKind::King {
                    return true;
                }
            }
        }
        false
    }
}
