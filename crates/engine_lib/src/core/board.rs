use super::{Bitboard, Move, MoveKind, Square};
use std::fmt;

// Clone is for naive copy-make move gen

#[derive(Clone, Debug)]
pub struct Board {
    bitboards: [[Bitboard; 6]; 2],
    squares: [Option<Piece>; 64],
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
    pub fn is_en_passant(&self, square: Square) -> bool {
        self.en_passant == Some(square)
    }

    pub fn rights(&self) -> CastlingRights {
        self.castling
    }

    // To Do: Add error passing on invalid move, should make perft debugging easier :)
    pub fn apply(&mut self, mv: Move) {
        let origin = mv.origin();
        let destination = mv.destination();
        let move_kind = mv.kind();
        let is_capture = matches!(
            move_kind,
            MoveKind::Capture | MoveKind::EnPassantCapture | MoveKind::PromotionCapture(_)
        );
        let is_double_pawn_push = move_kind == MoveKind::DoublePawnPush;

        let piece = self.set_piece(None, origin);
        if piece == None {
            panic!("Moving an empty square are we?")
        };
        let moving_piece = piece.expect("This should never happen");

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
                    self.set_piece(Some(other), Square::F1);
                    self.set_piece(None, Square::H1);
                } else {
                    self.set_piece(Some(other), Square::F8);
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
                    self.set_piece(Some(other), Square::D1);
                    self.set_piece(None, Square::A1);
                } else {
                    self.set_piece(Some(other), Square::D8);
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
                self.set_piece(Some(promoted_to), destination);
            }
            MoveKind::PromotionCapture(kind) => {
                let promoted_to = Piece {
                    color: self.to_move,
                    kind,
                };
                self.set_piece(Some(promoted_to), destination);
            }
        }

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
}
