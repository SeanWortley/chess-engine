use crate::core::bitboard::Bitboard;
use crate::core::square::Square;

pub struct Board {
    bitboards: [[Bitboard; 6]; 2], // 2D array, indexed with enums (color, then piece)
    to_move: Color,
    castling: CastlingRights,
    en_passant: Option<Square>,
}

// For nicer indexing :)
pub enum Color {
    White = 0,
    Black = 1,
}

// For nicer indexing :)
pub enum Piece {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

pub struct Color(bool);

pub struct CastlingRights(u8);

impl CastlingRights {
    pub const WHITE_KINGSIDE: u8 = 0b0001;
    pub const WHITE_QUEENSIDE: u8 = 0b0010;
    pub const BLACK_KINGSIDE: u8 = 0b0100;
    pub const BLACK_QUEENSIDE: u8 = 0b1000;
}
