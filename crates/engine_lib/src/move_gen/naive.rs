use crate::core::board::Board;
use crate::core::board::CastlingRights;
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
    is_attacked: IsAttackedFn,
}

impl<TM: TransitionManager> MoveGenerator for NaiveMoveGenerator<TM> {
    #[inline]
    fn generate_moves(&mut self, board: &mut Board, moves: &mut MoveList) {
        let mut pseudo = MoveList::new();
        self.generate_pseudo_legal(board, &mut pseudo, true);

        for mv in pseudo.iter() {
            self.tm.make(board, *mv);

            let king_board = board.bitboard(board.to_move().opponent(), King);
            let king_square = Square::from_index((king_board.lsb()).unwrap());
            if !(self.is_attacked)(board, king_square, board.to_move()) {
                moves.push(*mv);
            }
            self.tm.unmake(board, *mv);
        }
    }
}

impl<TM: TransitionManager> NaiveMoveGenerator<TM> {
    pub fn new(tm: TM, is_attacked: IsAttackedFn) -> Self {
        NaiveMoveGenerator { tm, is_attacked }
    }
    #[inline]
    pub fn generate_pseudo_legal(&self, board: &Board, moves: &mut MoveList, with_castling: bool) {
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
                    King => self.generate_king_moves(&mut context, origin, with_castling),
                }
            }
        }
    }

    #[inline]
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
    #[inline]
    fn generate_knight_moves(&self, context: &mut GenerationContext<'_>, origin: Square) {
        for direction in [
            NorthNorthEast,
            EastNorthEast,
            EastSouthEast,
            SouthSouthEast,
            SouthSouthWest,
            WestSouthWest,
            WestNorthWest,
            NorthNorthWest,
        ] {
            Self::try_sliding_step(context, origin, origin, direction);
        }
    }
    #[inline]
    fn generate_bishop_moves(&self, context: &mut GenerationContext<'_>, origin: Square) {
        for direction in [NorthEast, SouthEast, SouthWest, NorthWest] {
            let mut current = origin;
            while let Some(next) = Self::try_sliding_step(context, current, origin, direction) {
                current = next;
            }
        }
    }
    #[inline]
    fn generate_rook_moves(&self, context: &mut GenerationContext<'_>, origin: Square) {
        for direction in [North, East, South, West] {
            let mut current = origin;
            while let Some(next) = Self::try_sliding_step(context, current, origin, direction) {
                current = next;
            }
        }
    }
    #[inline]
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
    #[inline]
    fn generate_king_moves(
        &self,
        context: &mut GenerationContext<'_>,
        origin: Square,
        with_castling: bool,
    ) {
        for direction in [
            North, NorthEast, East, SouthEast, South, SouthWest, West, NorthWest,
        ] {
            Self::try_sliding_step(context, origin, origin, direction); // Just once in each direction
        }
        if with_castling {
            Self::try_king_castle(context, origin);
            Self::try_queen_castle(context, origin);
        }
    }

    #[inline]
    fn try_king_castle(&self, context: &mut GenerationContext<'_>, origin: Square) {
        let board = context.board;
        let rights = board.rights();
        match context.color {
            Color::White => {
                // Pieces in the way?
                if !((board.get_piece(Square::F1) == None) && (board.get_piece(Square::G1) == None))
                {
                    return;
                }
                // Has rights?
                if !rights.has_rights(CastlingRights::WHITE_KINGSIDE) {
                    return;
                }
                // In check, or checks in transit?
                if (self.is_attacked)(board, Square::F1, context.color.opponent())
                    || (self.is_attacked)(board, Square::E1, context.color.opponent())
                {
                    return;
                }

                context.moves.push(Move::king_castle(origin, Square::G1));
            }
            Color::Black => {
                // Pieces in the way?
                if !((board.get_piece(Square::F8) == None) && (board.get_piece(Square::G8) == None))
                {
                    return;
                }
                // Has rights?
                if !rights.has_rights(CastlingRights::BLACK_KINGSIDE) {
                    return;
                }
                // In check, or checks in transit?
                if (self.is_attacked)(board, Square::F8, context.color.opponent())
                    || (self.is_attacked)(board, Square::E8, context.color.opponent())
                {
                    return;
                }

                context.moves.push(Move::king_castle(origin, Square::G8));
            }
        }
    }

    #[inline]
    fn try_queen_castle(&self, context: &mut GenerationContext<'_>, origin: Square) {
        let board = context.board;
        let rights = board.rights();
        match context.color {
            Color::White => {
                // Pieces in the way?
                if !((board.get_piece(Square::B1) == None)
                    && (board.get_piece(Square::C1) == None)
                    && (board.get_piece(Square::D1) == None))
                {
                    return;
                }
                // Has rights?
                if !rights.has_rights(CastlingRights::WHITE_QUEENSIDE) {
                    return;
                }
                // In check, or checks in transit?
                if (self.is_attacked)(board, Square::D1, context.color.opponent())
                    || (self.is_attacked)(board, Square::E1, context.color.opponent())
                {
                    return;
                }

                context.moves.push(Move::queen_castle(origin, Square::C1));
            }
            Color::Black => {
                // Pieces in the way?
                if !((board.get_piece(Square::B8) == None)
                    && (board.get_piece(Square::C8) == None)
                    && (board.get_piece(Square::D8) == None))
                {
                    return;
                }
                // Has rights?
                if !rights.has_rights(CastlingRights::BLACK_QUEENSIDE) {
                    return;
                }
                // In check, or checks in transit?
                if (self.is_attacked)(board, Square::D8, context.color.opponent())
                    || (self.is_attacked)(board, Square::E8, context.color.opponent())
                {
                    return;
                }

                context.moves.push(Move::queen_castle(origin, Square::C8));
            }
        }
    }

    #[inline]
    fn try_pawn_capture(context: &mut GenerationContext<'_>, origin: Square, direction: Direction) {
        let board = context.board;

        let (x, y) = direction.offset();
        let file = (origin.file() as i8 + x) as u8;
        let rank = (origin.rank() as i8 + y) as u8;

        // Cant capture outside the board!
        if !(0..8).contains(&file) {
            return;
        }

        let destination = Square::new(file, rank);

        match board.get_piece(destination) {
            Some(target_piece) => {
                if target_piece.color == context.color {
                    return;
                }
                if (rank == 7) || (rank == 0) {
                    context
                        .moves
                        .push(Move::promotion_capture(origin, destination, Knight));
                    context
                        .moves
                        .push(Move::promotion_capture(origin, destination, Bishop));
                    context
                        .moves
                        .push(Move::promotion_capture(origin, destination, Rook));
                    context
                        .moves
                        .push(Move::promotion_capture(origin, destination, Queen));
                    return;
                }
                context.moves.push(Move::capture(origin, destination));
            }
            None => {
                if board.is_en_passant(destination) {
                    context.moves.push(Move::en_passant(origin, destination));
                }
            }
        }
    }

    #[inline]
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
                    return false; // Don't need to check for double push on promotion
                } else {
                    context.moves.push(Move::quiet(origin, destination));
                    return true;
                }
            }
        }
    }

    #[inline]
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

    #[inline]
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
