use super::{Bitboard, Move, MoveKind, Square};
use crate::core::zobrist::ZOBRIST;
use std::fmt;

// Clone is for naive copy-make move gen

#[derive(Clone, Debug)]
pub struct Board {
    bitboards: [[Bitboard; 6]; 2],
    squares: [Option<Piece>; 64],
    hash: u64,
    to_move: Color,
    castling: CastlingRights,
    en_passant: Option<Square>,
    halfmove_clock: u8,
    fullmove_counter: u16,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CastlingRights(u8);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: Color,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Color {
    White = 0,
    Black = 1,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PieceKind {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

impl AsRef<Board> for Board {
    fn as_ref(&self) -> &Board {
        &self
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for rank in (0..8).rev() {
            write!(f, "{} |", rank + 1)?;
            for file in 0..8 {
                let square = Square::new(file, rank);
                match self.squares[square.index() as usize] {
                    Some(piece) => write!(f, "{} ", piece.to_fen_char())?,
                    None => write!(f, ". ")?,
                }
            }
            writeln!(f)?;
        }
        writeln!(f, "   ---------------")?;
        write!(f, "   A B C D E F G H")?;
        Ok(())
    }
}

impl Board {
    pub const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    const FEN_START_INDEX: u8 = 56;

    fn empty() -> Self {
        Board {
            bitboards: [[Bitboard::EMPTY; 6]; 2],
            squares: [None; 64],
            hash: 0,
            to_move: Color::White,
            castling: CastlingRights(0),
            en_passant: None,
            halfmove_clock: 0,
            fullmove_counter: 0,
        }
    }

    pub fn starting_position() -> Self {
        Board::from_fen(Board::START_FEN)
    }

    pub fn material_pieces(&self) -> Bitboard {
        self.bitboards[Color::White as usize][PieceKind::Pawn as usize]
            | self.bitboards[Color::White as usize][PieceKind::Bishop as usize]
            | self.bitboards[Color::White as usize][PieceKind::Knight as usize]
            | self.bitboards[Color::White as usize][PieceKind::Rook as usize]
            | self.bitboards[Color::White as usize][PieceKind::Queen as usize]
            | self.bitboards[Color::Black as usize][PieceKind::Pawn as usize]
            | self.bitboards[Color::Black as usize][PieceKind::Bishop as usize]
            | self.bitboards[Color::Black as usize][PieceKind::Knight as usize]
            | self.bitboards[Color::Black as usize][PieceKind::Rook as usize]
            | self.bitboards[Color::Black as usize][PieceKind::Queen as usize]
    }

    pub fn get_piece(&self, square: Square) -> Option<Piece> {
        self.squares[square.index() as usize]
    }

    pub fn bitboard(&self, color: Color, kind: PieceKind) -> Bitboard {
        self.bitboards[color as usize][kind as usize]
    }

    pub fn occupied(&self) -> Bitboard {
        let mut occupied = Bitboard::EMPTY;
        for color in self.bitboards.iter() {
            for bitboard in color.iter() {
                occupied |= *bitboard;
            }
        }
        occupied
    }

    pub fn occupied_by(&self, color: Color) -> Bitboard {
        let mut occupied = Bitboard::EMPTY;
        for bitboard in self.bitboards[color as usize].iter() {
            occupied |= *bitboard
        }
        occupied
    }

    pub fn en_passant(&self) -> Option<Square> {
        self.en_passant
    }

    pub fn is_en_passant(&self, square: Square) -> bool {
        self.en_passant == Some(square)
    }

    pub fn rights(&self) -> CastlingRights {
        self.castling
    }

    pub fn halfmove_clock(&self) -> u8 {
        self.halfmove_clock
    }
    pub fn hash(&self) -> u64 {
        self.hash
    }

    pub fn set_hash(&mut self, new_hash: u64) {
        self.hash = new_hash;
    }

    pub fn xor(&mut self, component: u64) {
        self.hash ^= component;
    }

    // To Do: Add error passing on invalid move, should make perft debugging easier :)
    pub fn apply(&mut self, mv: Move) {
        // Toggle side to move
        self.xor(ZOBRIST.side_to_move);

        let origin = mv.origin();
        let destination = mv.destination();
        let move_kind = mv.kind();
        let is_capture = matches!(
            move_kind,
            MoveKind::Capture | MoveKind::EnPassantCapture | MoveKind::PromotionCapture(_)
        );
        let is_double_pawn_push = move_kind == MoveKind::DoublePawnPush;

        // XOR out old en passant if it exists
        if let Some(ep_square) = self.en_passant {
            self.xor(ZOBRIST.en_passant[ep_square.file() as usize]);
        }

        let piece = self.set_piece(None, origin);
        if piece == None {
            panic!("Moving an empty square are we?")
        };
        let moving_piece = piece.expect("This should never happen");

        // XOR out piece leaving origin
        self.xor(
            ZOBRIST.pieces[moving_piece.color as usize][moving_piece.kind as usize]
                [origin.index() as usize],
        );

        let captured_piece = if is_capture {
            if move_kind == MoveKind::EnPassantCapture {
                Some(Piece {
                    color: self.to_move.opponent(),
                    kind: PieceKind::Pawn,
                })
            } else {
                self.get_piece(destination)
            }
        } else {
            None
        };

        // XOR out captured piece if any
        if let Some(captured) = captured_piece {
            let captured_square = if move_kind == MoveKind::EnPassantCapture {
                // For en passant, piece is on different square
                if self.to_move == Color::White {
                    Square::from_index(destination.index() - 8)
                } else {
                    Square::from_index(destination.index() + 8)
                }
            } else {
                destination
            };
            self.xor(
                ZOBRIST.pieces[captured.color as usize][captured.kind as usize]
                    [captured_square.index() as usize],
            );
        }

        match move_kind {
            MoveKind::Quiet => {
                self.set_piece(piece, destination);
            }
            MoveKind::DoublePawnPush => {
                self.set_piece(piece, destination);

                // Add en passant
                let ep_square = if self.to_move == Color::White {
                    Square::from_index(origin.index() + 8)
                } else {
                    Square::from_index(origin.index() - 8)
                };
                self.en_passant = Some(ep_square);
            }
            MoveKind::KingCastle => {
                self.set_piece(piece, destination);
                let other = Piece {
                    color: self.to_move,
                    kind: PieceKind::Rook,
                };
                if self.to_move == Color::White {
                    // XOR out rook leaving H1
                    self.xor(
                        ZOBRIST.pieces[Color::White as usize][PieceKind::Rook as usize]
                            [Square::H1.index() as usize],
                    );
                    self.set_piece(Some(other), Square::F1);
                    // XOR in rook at F1
                    self.xor(
                        ZOBRIST.pieces[Color::White as usize][PieceKind::Rook as usize]
                            [Square::F1.index() as usize],
                    );
                    self.set_piece(None, Square::H1);
                } else {
                    // XOR out rook leaving H8
                    self.xor(
                        ZOBRIST.pieces[Color::Black as usize][PieceKind::Rook as usize]
                            [Square::H8.index() as usize],
                    );
                    self.set_piece(Some(other), Square::F8);
                    // XOR in rook at F8
                    self.xor(
                        ZOBRIST.pieces[Color::Black as usize][PieceKind::Rook as usize]
                            [Square::F8.index() as usize],
                    );
                    self.set_piece(None, Square::H8);
                }
            }
            MoveKind::QueenCastle => {
                self.set_piece(piece, destination);
                let other = Piece {
                    color: self.to_move,
                    kind: PieceKind::Rook,
                };
                if self.to_move == Color::White {
                    // XOR out rook leaving A1
                    self.xor(
                        ZOBRIST.pieces[Color::White as usize][PieceKind::Rook as usize]
                            [Square::A1.index() as usize],
                    );
                    self.set_piece(Some(other), Square::D1);
                    // XOR in rook at D1
                    self.xor(
                        ZOBRIST.pieces[Color::White as usize][PieceKind::Rook as usize]
                            [Square::D1.index() as usize],
                    );
                    self.set_piece(None, Square::A1);
                } else {
                    // XOR out rook leaving A8
                    self.xor(
                        ZOBRIST.pieces[Color::Black as usize][PieceKind::Rook as usize]
                            [Square::A8.index() as usize],
                    );
                    self.set_piece(Some(other), Square::D8);
                    // XOR in rook at D8
                    self.xor(
                        ZOBRIST.pieces[Color::Black as usize][PieceKind::Rook as usize]
                            [Square::D8.index() as usize],
                    );
                    self.set_piece(None, Square::A8);
                }
            }
            MoveKind::Capture => {
                self.set_piece(piece, destination); // set_piece should take care of bitboard manipulation
            }
            MoveKind::EnPassantCapture => {
                self.set_piece(piece, destination);
                let eliminated_square = if self.to_move == Color::White {
                    Square::from_index(destination.index() - 8)
                } else {
                    Square::from_index(destination.index() + 8)
                };
                self.set_piece(None, eliminated_square);
            }
            MoveKind::Promotion(kind) => {
                let promoted_to = Piece {
                    color: self.to_move,
                    kind,
                };
                // XOR out pawn that was already removed from origin
                // XOR in promoted piece at destination
                self.xor(
                    ZOBRIST.pieces[self.to_move as usize][kind as usize]
                        [destination.index() as usize],
                );
                self.set_piece(Some(promoted_to), destination);
            }
            MoveKind::PromotionCapture(kind) => {
                let promoted_to = Piece {
                    color: self.to_move,
                    kind,
                };
                // XOR in promoted piece at destination
                self.xor(
                    ZOBRIST.pieces[self.to_move as usize][kind as usize]
                        [destination.index() as usize],
                );
                self.set_piece(Some(promoted_to), destination);
            }
        }

        // XOR in piece arriving at destination for non-promotion moves
        match move_kind {
            MoveKind::Promotion(_) | MoveKind::PromotionCapture(_) => {
                // Already handled above
            }
            _ => {
                // All other moves: XOR in piece at destination
                self.xor(
                    ZOBRIST.pieces[moving_piece.color as usize][moving_piece.kind as usize]
                        [destination.index() as usize],
                );
            }
        }

        // Store old castling rights to compute XOR
        let old_castling = self.castling.0;

        match moving_piece.kind {
            PieceKind::King => {
                if moving_piece.color == Color::White {
                    self.castling
                        .remove(CastlingRights::WHITE_KINGSIDE | CastlingRights::WHITE_QUEENSIDE);
                } else {
                    self.castling
                        .remove(CastlingRights::BLACK_KINGSIDE | CastlingRights::BLACK_QUEENSIDE);
                }
            }
            PieceKind::Rook => self.castling.remove(CastlingRights::home_mask(origin)),
            _ => {}
        }

        if let Some(captured) = captured_piece {
            if captured.kind == PieceKind::Rook {
                self.castling.remove(CastlingRights::home_mask(destination));
            }
        }

        // XOR castling rights changes
        let castling_xor = old_castling ^ self.castling.0;
        if (castling_xor & CastlingRights::WHITE_KINGSIDE) != 0 {
            self.xor(ZOBRIST.castling_rights[0]);
        }
        if (castling_xor & CastlingRights::WHITE_QUEENSIDE) != 0 {
            self.xor(ZOBRIST.castling_rights[1]);
        }
        if (castling_xor & CastlingRights::BLACK_KINGSIDE) != 0 {
            self.xor(ZOBRIST.castling_rights[2]);
        }
        if (castling_xor & CastlingRights::BLACK_QUEENSIDE) != 0 {
            self.xor(ZOBRIST.castling_rights[3]);
        }

        // Update halfmove
        if is_capture || moving_piece.kind == PieceKind::Pawn {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }

        // Update en passant
        if !is_double_pawn_push {
            self.en_passant = None
        }

        // Update fulmovve
        if self.to_move == Color::Black {
            self.fullmove_counter += 1;
        }

        // XOR in new en passant if this was a double pawn push
        if is_double_pawn_push {
            if let Some(ep_square) = self.en_passant {
                self.xor(ZOBRIST.en_passant[ep_square.file() as usize]);
            }
        }

        // Update to_move
        self.to_move = self.to_move.opponent();
    }

    pub fn from_fen(fen: &str) -> Self {
        let mut board = Board::empty();
        let mut fields = fen.split_whitespace();

        let arrangement = fields.next().expect("Missing arrangement");
        let mut index = Board::FEN_START_INDEX;
        for char in arrangement.chars() {
            match char {
                '/' => index -= 16,
                '1'..='8' => index += char.to_digit(10).unwrap() as u8,
                'p' | 'P' | 'n' | 'N' | 'b' | 'B' | 'r' | 'R' | 'q' | 'Q' | 'k' | 'K' => {
                    board.add_from_fen(char, index);
                    index += 1;
                }
                _ => panic!("Invalid arrangement character: {}", char),
            }
        }

        let to_move = fields.next().expect("Missing side to move");
        board.to_move = match to_move {
            "w" => Color::White,
            "b" => Color::Black,
            _ => panic!("Invalid side to move: {}", to_move),
        };

        let castling = fields.next().expect("Missing castling rights");
        board.castling = CastlingRights(CastlingRights::NO_RIGHTS);
        for char in castling.chars() {
            match char {
                '-' => break,
                'K' => board.castling.0 |= CastlingRights::WHITE_KINGSIDE,
                'Q' => board.castling.0 |= CastlingRights::WHITE_QUEENSIDE,
                'k' => board.castling.0 |= CastlingRights::BLACK_KINGSIDE,
                'q' => board.castling.0 |= CastlingRights::BLACK_QUEENSIDE,
                _ => panic!("Invalid castling character: {}", char),
            }
        }

        let en_passant = fields.next().expect("Missing en passant");
        board.en_passant = match en_passant {
            "-" => None,
            s => Some(Square::from_name(s)),
        };

        board.halfmove_clock = fields
            .next()
            .expect("Missing halfmove clock")
            .parse()
            .expect("Invalid halfmove clock");

        board.fullmove_counter = fields
            .next()
            .expect("Missing fullmove counter")
            .parse()
            .expect("Invalid fullmove counter");

        board.hash = ZOBRIST.compute_from_scratch(&board);
        board
    }

    fn add_from_fen(&mut self, char: char, index: u8) {
        let color = if char.is_uppercase() {
            Color::White
        } else {
            Color::Black
        };
        let kind = match char.to_ascii_lowercase() {
            'p' => PieceKind::Pawn,
            'n' => PieceKind::Knight,
            'b' => PieceKind::Bishop,
            'r' => PieceKind::Rook,
            'q' => PieceKind::Queen,
            'k' => PieceKind::King,
            _ => panic!("Invalid piece character: {}", char),
        };
        let piece = Piece { color, kind };
        let square = Square::from_index(index);
        self.set_piece(Some(piece), square);
    }

    fn set_piece(&mut self, new: Option<Piece>, square: Square) -> Option<Piece> {
        let old = self.squares[square.index() as usize]; // This should be cleaned up later
        self.squares[square.index() as usize] = new;
        debug_assert_ne!(old, new, "How the fuck did this happen?");
        // Was old something or nothing?
        match old {
            Some(piece) => {
                self.bitboards[piece.color as usize][piece.kind as usize].remove_square(square); // Remove what was
            }
            None => {}
        }
        // Are we setting a piece or a null piece?
        match new {
            Some(piece) => {
                self.bitboards[piece.color as usize][piece.kind as usize].add_square(square); // Add what will be
            }
            None => {}
        }
        old
    }

    pub fn add_piece(&mut self, color: Color, kind: PieceKind, square: Square) {
        let piece = Piece { color, kind };
        self.set_piece(Some(piece), square);
    }

    pub fn remove_piece(&mut self, square: Square) -> Option<Piece> {
        self.set_piece(None, square)
    }

    pub fn to_move(&self) -> Color {
        self.to_move
    }

    pub fn mirror(board: &Board) -> Self {
        let mut mirror = board.clone();
        mirror.to_move = mirror.to_move.opponent();
        mirror
    }
}

impl CastlingRights {
    pub const ALL_RIGHTS: u8 = 0b1111;
    pub const WHITE_KINGSIDE: u8 = 0b0001;
    pub const WHITE_QUEENSIDE: u8 = 0b0010;
    pub const BLACK_KINGSIDE: u8 = 0b0100;
    pub const BLACK_QUEENSIDE: u8 = 0b1000;
    pub const NO_RIGHTS: u8 = 0b0000;

    pub fn add(&mut self, mask: u8) {
        self.0 |= mask;
    }

    pub fn remove(&mut self, mask: u8) {
        self.0 &= !mask;
    }

    pub fn home_mask(square: Square) -> u8 {
        match square.index() {
            0 => Self::WHITE_QUEENSIDE,
            7 => Self::WHITE_KINGSIDE,
            56 => Self::BLACK_QUEENSIDE,
            63 => Self::BLACK_KINGSIDE,
            _ => 0,
        }
    }

    pub fn has_rights(&self, rights: u8) -> bool {
        (self.0 & rights) != 0
    }
}

impl Color {
    pub fn opponent(self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

impl Piece {
    pub fn to_fen_char(self) -> char {
        let c = match self.kind {
            PieceKind::Pawn => 'p',
            PieceKind::Knight => 'n',
            PieceKind::Bishop => 'b',
            PieceKind::Rook => 'r',
            PieceKind::Queen => 'q',
            PieceKind::King => 'k',
        };
        match self.color {
            Color::White => c.to_ascii_uppercase(),
            Color::Black => c,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn idx(name: &str) -> Square {
        Square::from_name(name)
    }

    #[test]
    fn test_starting_position() {
        let board = Board::starting_position();
        // White back rank
        assert_eq!(
            board.squares[0],
            Some(Piece {
                color: Color::White,
                kind: PieceKind::Rook
            })
        );
        assert_eq!(
            board.squares[1],
            Some(Piece {
                color: Color::White,
                kind: PieceKind::Knight
            })
        );
        assert_eq!(
            board.squares[4],
            Some(Piece {
                color: Color::White,
                kind: PieceKind::King
            })
        );
        // Black back rank
        assert_eq!(
            board.squares[56],
            Some(Piece {
                color: Color::Black,
                kind: PieceKind::Rook
            })
        );
        assert_eq!(
            board.squares[60],
            Some(Piece {
                color: Color::Black,
                kind: PieceKind::King
            })
        );
        // Empty middle ranks
        for i in 16..48 {
            assert_eq!(board.squares[i], None);
        }
        // Metadata
        assert_eq!(board.to_move, Color::White);
        assert_eq!(board.castling, CastlingRights(CastlingRights::ALL_RIGHTS));
        assert!(board.en_passant.is_none());
        assert_eq!(board.halfmove_clock, 0);
        assert_eq!(board.fullmove_counter, 1);
    }

    #[test]
    fn test_from_fen() {
        // After e4: black to move, pawn on e4, e2 empty, en passant on e3
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");
        assert_eq!(board.to_move, Color::Black);
        assert_eq!(board.en_passant, Some(Square::from_name("e3")));
        assert_eq!(
            board.squares[28],
            Some(Piece {
                color: Color::White,
                kind: PieceKind::Pawn
            })
        );
        assert_eq!(board.squares[12], None);

        // Partial castling rights
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w Kq - 0 1");
        assert_eq!(
            board.castling,
            CastlingRights(CastlingRights::WHITE_KINGSIDE | CastlingRights::BLACK_QUEENSIDE)
        );

        // No castling rights
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 1");
        assert_eq!(board.castling, CastlingRights(0));

        // Clocks
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 5 42");
        assert_eq!(board.halfmove_clock, 5);
        assert_eq!(board.fullmove_counter, 42);
    }

    #[test]
    fn test_add_piece() {
        let mut board = Board::empty();
        let square = Square::from_index(27);
        board.add_piece(Color::White, PieceKind::Queen, square);
        let piece = Piece {
            color: Color::White,
            kind: PieceKind::Queen,
        };

        assert_eq!(board.get_piece(square), Some(piece));
        assert!(
            board
                .bitboard(Color::White, PieceKind::Queen)
                .has_square(square)
        );
    }

    #[test]
    fn test_set_piece() {
        let mut board = Board::empty();
        let square = Square::from_index(27);
        let white_queen = Piece {
            color: Color::White,
            kind: PieceKind::Queen,
        };
        let black_knight = Piece {
            color: Color::Black,
            kind: PieceKind::Knight,
        };

        assert_eq!(board.set_piece(Some(white_queen), square), None);
        assert_eq!(board.get_piece(square), Some(white_queen));
        assert!(
            board
                .bitboard(Color::White, PieceKind::Queen)
                .has_square(square)
        );

        assert_eq!(
            board.set_piece(Some(black_knight), square),
            Some(white_queen)
        );
        assert_eq!(board.get_piece(square), Some(black_knight));
        assert!(
            !board
                .bitboard(Color::White, PieceKind::Queen)
                .has_square(square)
        );
        assert!(
            board
                .bitboard(Color::Black, PieceKind::Knight)
                .has_square(square)
        );
    }

    #[test]
    fn test_remove_piece() {
        let mut board = Board::empty();
        let square = Square::from_index(27);
        board.add_piece(Color::White, PieceKind::Queen, square);

        let removed = board.remove_piece(square);
        assert_eq!(
            removed,
            Some(Piece {
                color: Color::White,
                kind: PieceKind::Queen
            })
        );
        assert_eq!(board.get_piece(square), None);
        assert!(
            !board
                .bitboard(Color::White, PieceKind::Queen)
                .has_square(square)
        );
    }

    #[test]
    fn test_bitboard() {
        let board = Board::starting_position();
        // a8 rook should be the least significant set bit in the black rook bitboard
        assert_eq!(
            board.bitboard(Color::Black, PieceKind::Rook).pop_lsb(),
            Some(Square::from_name("a8").index())
        );
    }

    #[test]
    fn test_occupied() {
        let board = Board::starting_position();
        let occupied = board.occupied();

        assert_eq!(occupied.count(), 32);
        assert!(occupied.has_square(Square::from_name("a1")));
        assert!(occupied.has_square(Square::from_name("e8")));
        assert!(!occupied.has_square(Square::from_name("e4")));
    }

    #[test]
    fn test_occupied_by() {
        let board = Board::starting_position();
        let white_occupied = board.occupied_by(Color::White);
        let black_occupied = board.occupied_by(Color::Black);

        assert_eq!(white_occupied.count(), 16);
        assert_eq!(black_occupied.count(), 16);

        assert!(white_occupied.has_square(Square::from_name("a1")));
        assert!(!white_occupied.has_square(Square::from_name("a8")));

        assert!(black_occupied.has_square(Square::from_name("a8")));
        assert!(!black_occupied.has_square(Square::from_name("a1")));
    }

    #[test]
    fn test_apply() {
        let mut board = Board::starting_position();

        board.apply(Move::quiet(idx("g1"), idx("f3")));

        assert_eq!(
            board.get_piece(Square::from_name("f3")),
            Some(Piece {
                color: Color::White,
                kind: PieceKind::Knight,
            })
        );
        assert_eq!(board.get_piece(Square::from_name("g1")), None);
        assert_eq!(board.to_move, Color::Black);
        assert_eq!(board.halfmove_clock, 1);
        assert_eq!(board.fullmove_counter, 1);
    }

    #[test]
    fn test_opponent() {
        assert_eq!(Color::White.opponent(), Color::Black);
        assert_eq!(Color::Black.opponent(), Color::White);
    }

    #[test]
    fn test_to_fen_char() {
        assert_eq!(
            Piece {
                color: Color::White,
                kind: PieceKind::Pawn
            }
            .to_fen_char(),
            'P'
        );
        assert_eq!(
            Piece {
                color: Color::Black,
                kind: PieceKind::Pawn
            }
            .to_fen_char(),
            'p'
        );
        assert_eq!(
            Piece {
                color: Color::White,
                kind: PieceKind::King
            }
            .to_fen_char(),
            'K'
        );
        assert_eq!(
            Piece {
                color: Color::Black,
                kind: PieceKind::King
            }
            .to_fen_char(),
            'k'
        );
    }

    // ---- Zobrist hash invariance ----
    // Each test asserts that the incrementally maintained hash matches a
    // from-scratch recompute after every ply. A failure means apply() is
    // missing (or double-applying) a XOR for that move kind.

    fn assert_hash_synced(board: &Board) {
        assert_eq!(
            board.hash(),
            ZOBRIST.compute_from_scratch(board),
            "incremental hash drifted out of sync with compute_from_scratch"
        );
    }

    fn apply_synced(board: &mut Board, mv: Move) {
        board.apply(mv);
        assert_hash_synced(board);
    }

    #[test]
    fn test_hash_initialized_by_from_fen() {
        let board = Board::starting_position();
        assert_ne!(board.hash(), 0);
        assert_hash_synced(&board);
    }

    #[test]
    fn test_hash_sync_quiet_and_double_push() {
        let mut board = Board::starting_position();
        apply_synced(&mut board, Move::double_pawn_push(idx("e2"), idx("e4"))); // sets EP file
        apply_synced(&mut board, Move::double_pawn_push(idx("e7"), idx("e5"))); // swaps EP file
        apply_synced(&mut board, Move::quiet(idx("g1"), idx("f3"))); // clears EP
        apply_synced(&mut board, Move::quiet(idx("b8"), idx("c6")));
    }

    #[test]
    fn test_hash_sync_en_passant() {
        let mut board = Board::starting_position();
        apply_synced(&mut board, Move::double_pawn_push(idx("e2"), idx("e4")));
        apply_synced(&mut board, Move::quiet(idx("a7"), idx("a6")));
        apply_synced(&mut board, Move::quiet(idx("e4"), idx("e5")));
        apply_synced(&mut board, Move::double_pawn_push(idx("d7"), idx("d5")));
        apply_synced(&mut board, Move::en_passant(idx("e5"), idx("d6")));
    }

    #[test]
    fn test_hash_sync_kingside_castles() {
        let mut board =
            Board::from_fen("rnbqk2r/pppp1ppp/5n2/2b1p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4");
        assert_hash_synced(&board);
        apply_synced(&mut board, Move::king_castle(idx("e1"), idx("g1")));
        apply_synced(&mut board, Move::king_castle(idx("e8"), idx("g8")));
    }

    #[test]
    fn test_hash_sync_queenside_castles() {
        let mut board =
            Board::from_fen("r3kbnr/pppqpppp/2npb3/8/8/2NPB3/PPPQPPPP/R3KBNR w KQkq - 6 6");
        assert_hash_synced(&board);
        apply_synced(&mut board, Move::queen_castle(idx("e1"), idx("c1")));
        apply_synced(&mut board, Move::queen_castle(idx("e8"), idx("c8")));
    }

    #[test]
    fn test_hash_sync_promotions() {
        let mut board = Board::from_fen("8/P3k3/8/8/8/8/4K1p1/8 w - - 0 1");
        assert_hash_synced(&board);
        apply_synced(
            &mut board,
            Move::promotion(idx("a7"), idx("a8"), PieceKind::Queen),
        );
        apply_synced(
            &mut board,
            Move::promotion(idx("g2"), idx("g1"), PieceKind::Knight),
        );
    }

    #[test]
    fn test_hash_sync_promotion_captures() {
        let mut board = Board::from_fen("1n2k3/P7/8/8/8/8/6p1/4K1NR w - - 0 1");
        assert_hash_synced(&board);
        apply_synced(
            &mut board,
            Move::promotion_capture(idx("a7"), idx("b8"), PieceKind::Queen),
        );
        apply_synced(
            &mut board,
            Move::promotion_capture(idx("g2"), idx("h1"), PieceKind::Queen),
        );
    }

    #[test]
    fn test_hash_sync_rights_revoking_rook_captures() {
        let mut board = Board::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1");
        assert_hash_synced(&board);
        apply_synced(&mut board, Move::capture(idx("a1"), idx("a8"))); // kills both queenside rights
        apply_synced(&mut board, Move::capture(idx("h8"), idx("h1"))); // kills both kingside rights
    }
}
