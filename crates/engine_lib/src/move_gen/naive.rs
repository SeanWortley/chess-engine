use crate::core::board::Board;
use crate::core::board::Color;
use crate::core::board::PieceKind::*;
use crate::core::direction::Direction;
use crate::core::direction::Direction::*;
use crate::core::move_list::MoveList;
use crate::core::moves::Move;
use crate::core::square::Square;
use crate::move_gen::attacks::*;
use crate::{move_gen::MoveGenerator, transition::TransitionManager};

struct GenerationContext<'a> {
    // Doesn't live longer than the values inside :)
    board: &'a Board,
    moves: &'a mut MoveList,
    color: Color,
}

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
        let color = board.to_move();
        let mut context = GenerationContext {
            board,
            moves,
            color,
        };

        for kind in [Pawn, Knight, Bishop, Rook, Queen, King] {
            let bitboard = board.bitboard(color, kind);
            for origin_index in bitboard.iter() {
                let origin = Square::from_index(origin_index);
                match kind {
                    Pawn => self.generate_pawn_moves(&mut context, origin),
                    Knight => self.generate_knight_moves(&mut context, origin),
                    Bishop => self.generate_bishop_moves(&mut context, origin),
                    Rook => self.generate_rook_moves(&mut context, origin),
                    Queen => self.generate_queen_moves(&mut context, origin),
                    King => self.generate_king_moves(&mut context, origin),
                }
            }
        }
    }

    fn generate_pawn_moves(&self, _context: &mut GenerationContext<'_>, _from: Square) {}
    fn generate_knight_moves(&self, _context: &mut GenerationContext<'_>, _from: Square) {}
    fn generate_bishop_moves(&self, _context: &mut GenerationContext<'_>, _from: Square) {}
    fn generate_rook_moves(&self, _context: &mut GenerationContext<'_>, _from: Square) {}
    fn generate_queen_moves(&self, context: &mut GenerationContext<'_>, from: Square) {
        for direction in [
            North, NorthEast, East, SouthEast, South, SouthWest, West, NorthWest,
        ] {
            let mut current = from;
            while let Some(next) = Self::try_step(context, from, current, direction) {
                current = next;
            }
        }
    }
    fn generate_king_moves(&self, _context: &mut GenerationContext<'_>, _from: Square) {}

    fn try_step(
        context: &mut GenerationContext<'_>,
        origin: Square,
        current: Square,
        direction: Direction,
    ) -> Option<Square> {
        let (dx, dy) = direction.offset();
        let from = current;

        let next_file = from.file() as i8 + dx;
        let next_rank = from.rank() as i8 + dy;
        if !(0..8).contains(&next_file) || !(0..8).contains(&next_rank) {
            return None;
        }

        let destination = Square::new(next_file as u8, next_rank as u8);
        if let Some(piece) = context.board.get_piece(destination) {
            if piece.color != context.color {
                context
                    .moves
                    .push(Move::capture(origin.index(), destination.index()));
            }
            return None;
        }

        context
            .moves
            .push(Move::quiet(origin.index(), destination.index()));
        Some(destination)
    }
}
