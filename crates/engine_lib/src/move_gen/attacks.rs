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
    #[inline]
    pub fn side_to_move_gives_check(board: &Board, _algorithm: Algorithm) -> bool {
        Attacks::is_naive_gives_check(board)
    }

    // Generates pseudo-legal moves for board.to_move() and checks whether any capture the enemy king.
    #[inline]
    fn is_naive_gives_check(board: &Board) -> bool {
        let yucky_generator = NaiveMoveGenerator::new(CopyMakeTransition::new());
        let mut moves = MoveList::new();

        yucky_generator.generate_pseudo_legal(board, &mut moves, false);

        for mv in moves.iter() {
            let target_square = mv.destination();
            let target_piece = board.get_piece(target_square);

            if let Some(target_piece) = target_piece {
                if mv.is_capture() && target_piece.kind == PieceKind::King {
                    return true;
                }
            }
        }
        false
    }

    #[inline]
    pub fn is_under_attack(board: &Board, square: Square) -> bool {
        let yucky_generator = NaiveMoveGenerator::new(CopyMakeTransition::new());
        let mut moves = MoveList::new();

        let opponent_board = Board::mirror(board);

        yucky_generator.generate_pseudo_legal(&opponent_board, &mut moves, false);

        for mv in moves.iter() {
            if square != mv.destination() {
                continue;
            }

            // Pseudo move-gen includes pawn forward pushes, which are not attacks.
            // Keep only diagonal pawn moves as attacking moves.
            if let Some(piece) = opponent_board.get_piece(mv.origin()) {
                if piece.kind == PieceKind::Pawn {
                    let file_delta = mv.destination().file() as i8 - mv.origin().file() as i8;
                    if file_delta.abs() != 1 {
                        continue;
                    }
                }
            }

            return true;
        }
        return false;
    }
}
