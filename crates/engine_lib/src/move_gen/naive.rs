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

    fn generate_pawn_moves(&self, context: &mut GenerationContext<'_>, origin: Square) {
        let (directions, starting_rank): ([Direction; 3], u8) = if context.color == Color::White {
            ([North, NorthWest, NorthEast], 1)
        } else {
            ([South, SouthWest, SouthEast], 6)
        };

        if Self::try_pawn_push(context, origin, directions[0]) && (origin.rank() == starting_rank) {
            Self::try_double_pawn_push(context, origin, directions[0]);
        }
        Self::try_pawn_capture(context, origin, directions[1]);
        Self::try_pawn_capture(context, origin, directions[2]);
    }
    fn generate_knight_moves(&self, _context: &mut GenerationContext<'_>, _origin: Square) {}
    fn generate_bishop_moves(&self, context: &mut GenerationContext<'_>, origin: Square) {
        for direction in [NorthEast, SouthEast, SouthWest, NorthWest] {
            let mut current = origin;
            while let Some(next) = Self::try_sliding_step(context, current, origin, direction) {
                current = next;
            }
        }
    }
    fn generate_rook_moves(&self, context: &mut GenerationContext<'_>, origin: Square) {
        for direction in [North, East, South, West] {
            let mut current = origin;
            while let Some(next) = Self::try_sliding_step(context, current, origin, direction) {
                current = next;
            }
        }
    }
    fn generate_queen_moves(&self, context: &mut GenerationContext<'_>, origin: Square) {
        for direction in [
            North, NorthEast, East, SouthEast, South, SouthWest, West, NorthWest,
        ] {
            let mut current = origin;
            while let Some(next) = Self::try_sliding_step(context, current, origin, direction) {
                current = next;
            }
        }
    }
    fn generate_king_moves(&self, _context: &mut GenerationContext<'_>, _from: Square) {}

    fn try_pawn_capture(context: &mut GenerationContext<'_>, origin: Square, direction: Direction) {
    }

    fn try_pawn_push(
        context: &mut GenerationContext<'_>,
        origin: Square,
        direction: Direction,
    ) -> bool {
        let board = context.board;

        let (x, y) = direction.offset();
        let file = (origin.file() as i8 + x) as u8;
        let rank = (origin.rank() as i8 + y) as u8;

        let destination = Square::new(file, rank);

        match board.get_piece(destination) {
            Some(_target_piece) => {
                return false;
            }
            None => {
                if (rank == 7) || (rank == 0) {
                    context
                        .moves
                        .push(Move::promotion(origin, destination, Knight));
                    context
                        .moves
                        .push(Move::promotion(origin, destination, Bishop));
                    context
                        .moves
                        .push(Move::promotion(origin, destination, Rook));
                    context
                        .moves
                        .push(Move::promotion(origin, destination, Queen));
                }
                context.moves.push(Move::quiet(origin, destination));
                if (destination.rank() == 1) || (destination.rank() == 6) {}
            }
        }
        return true;
    }

    fn try_double_pawn_push(
        context: &mut GenerationContext<'_>,
        origin: Square,
        direction: Direction,
    ) {
        let board = context.board;

        let (x, y) = direction.offset();
        let file = (origin.file() as i8 + 2 * x) as u8;
        let rank = (origin.rank() as i8 + 2 * y) as u8;

        let destination = Square::new(file, rank);

        match board.get_piece(destination) {
            Some(_target_piece) => {}
            None => {
                context
                    .moves
                    .push(Move::double_pawn_push(origin, destination));
            }
        }
    }

    fn try_sliding_step(
        context: &mut GenerationContext<'_>,
        current: Square,
        origin: Square,
        direction: Direction,
    ) -> Option<Square> {
        let board = context.board;
        let color = context.color;

        let (x, y) = direction.offset();
        let file = current.file() as i8 + x;
        let rank = current.rank() as i8 + y;

        if !(0..8).contains(&file) || !(0..8).contains(&rank) {
            return None;
        }

        let destination = Square::new(file as u8, rank as u8);

        // Check for piece
        match board.get_piece(destination) {
            Some(target_piece) => {
                if target_piece.color == color {
                    return None;
                }
                context.moves.push(Move::capture(origin, destination));
                return None;
            }
            None => {
                context.moves.push(Move::quiet(origin, destination));
            }
        }
        Some(destination)
    }
}
