use crate::core::bitboard::Bitboard;
use crate::core::square::Square;

pub struct Board {
    bitboards: [[Bitboard; 6]; 2], // 2D array, indexed with enums (color, then piece)
    squares: [Option<(Color, Piece)>; 64], // Ensures O(1) Lookup hwne scanning all bitboards was nessecary :)
    to_move: Color,
    castling: CastlingRights,
    en_passant: Option<Square>,
    halfmove_clock: u8,
    fullmove_counter: u8,
}

pub struct CastlingRights(u8);

// For nicer indexing :)
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Color {
    White = 0,
    Black = 1,
}

// For nicer indexing :)
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Piece {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

impl Board {
    const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    const FEN_START_INDEX: u8 = 56;

    fn new() -> Self {
        Board {
            bitboards: [[Bitboard::EMPTY; 6]; 2],
            squares: [Option::None; 64],
            to_move: Color::White,
            castling: CastlingRights(CastlingRights::ALL_RIGHTS),
            en_passant: Option::None,
            halfmove_clock: 0,
            fullmove_counter: 0,
        }
    }

    pub fn new_game() -> Self {
        Board::from_fen(Board::START_FEN)
    }

    pub fn from_fen(fen: &str) -> Self {
        let mut board = Board::new();
        let mut fields = fen.split(' ');

        // Piece arrangement
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

        // Side to move
        let to_move = fields.next().expect("Missing side to move");
        board.to_move = match to_move {
            "w" => Color::White,
            "b" => Color::Black,
            _ => panic!("Invalid side to move: {}", to_move),
        };

        // Castling rights
        let castling = fields.next().expect("Missing castling rights");
        board.castling = CastlingRights(0);
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

        // En passant
        let en_passant = fields.next().expect("Missing en passant");
        board.en_passant = match en_passant {
            "-" => None,
            s => Some(Square::from_name(s)),
        };

        // Halfmove clock
        let halfmove = fields.next().expect("Missing halfmove clock");
        board.halfmove_clock = halfmove.parse().expect("Invalid halfmove clock");

        // Fullmove counter
        let fullmove = fields.next().expect("Missing fullmove counter");
        board.fullmove_counter = fullmove.parse().expect("Invalid fullmove counter");

        board
    }

    fn add_from_fen(&mut self, char: char, index: u8) {
        match char {
            'p' => self.add_piece(Color::Black, Piece::Pawn, index),
            'n' => self.add_piece(Color::Black, Piece::Knight, index),
            'b' => self.add_piece(Color::Black, Piece::Bishop, index),
            'r' => self.add_piece(Color::Black, Piece::Rook, index),
            'q' => self.add_piece(Color::Black, Piece::Queen, index),
            'k' => self.add_piece(Color::Black, Piece::King, index),
            'P' => self.add_piece(Color::White, Piece::Pawn, index),
            'N' => self.add_piece(Color::White, Piece::Knight, index),
            'B' => self.add_piece(Color::White, Piece::Bishop, index),
            'R' => self.add_piece(Color::White, Piece::Rook, index),
            'Q' => self.add_piece(Color::White, Piece::Queen, index),
            'K' => self.add_piece(Color::White, Piece::King, index),
            _ => panic!("Invalid piece character: {}", char),
        }
    }
    fn add_piece(&mut self, color: Color, piece: Piece, index: u8) {
        let square = Square::from_index(index);
        self.bitboards[color as usize][piece as usize].add_square(square);
        self.squares[index as usize] = Some((color, piece));
    }
}

impl CastlingRights {
    pub const ALL_RIGHTS: u8 = 0b1111;
    pub const WHITE_KINGSIDE: u8 = 0b0001;
    pub const WHITE_QUEENSIDE: u8 = 0b0010;
    pub const BLACK_KINGSIDE: u8 = 0b0100;
    pub const BLACK_QUEENSIDE: u8 = 0b1000;
}

impl Color {
    pub fn opponent(self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}
