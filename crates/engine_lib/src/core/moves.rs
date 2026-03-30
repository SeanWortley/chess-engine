// 16 bits for move encoding, per the wiki :)
// Very solid case to be made for 32 bit, but I lack the expertise to reason beyond the established starting path of 16 bits

/// Bit layout (low to high):
/// [0..5]   from/origin square (6 bits)
/// [6..11]  to/destination square (6 bits)
/// [12..15] flags (4 bits)
#[derive(Clone, Copy)]
pub struct Move(u16);

/// Ripped straight from the wiki babyyyyyy
pub mod flags {
    pub const QUIET: u16 = 0b0000; // Not special
    pub const DOUBLE_PAWN_PUSH: u16 = 0b0001;
    pub const KING_CASTLE: u16 = 0b0010; // Don't need this for standard chess, but ntil I know better, I'll just follow the wiki :)
    pub const QUEEN_CASTLE: u16 = 0b0011;
    pub const CAPTURE: u16 = 0b0100; // Can look this up, as we'll need to for PieceKind, but oh well for now
    pub const EP_CAPTURE: u16 = 0b0101;
    pub const KNIGHT_PROMOTION: u16 = 0b1000;
    pub const BISHOP_PROMOTION: u16 = 0b1001;
    pub const ROOK_PROMOTION: u16 = 0b1010;
    pub const QUEEN_PROMOTION: u16 = 0b1011;
    pub const KNIGHT_PROMO_CAPTURE: u16 = 0b1100;
    pub const BISHOP_PROMO_CAPTURE: u16 = 0b1101;
    pub const ROOK_PROMO_CAPTURE: u16 = 0b1110;
    pub const QUEEN_PROMO_CAPTURE: u16 = 0b1111;
}

impl Move {
    const ORIGIN_SHIFT: u16 = 0;
    const DESTINATION_SHIFT: u16 = 6; // Offset of destination bits
    const FLAGS_SHIFT: u16 = 12; // Offset of flag bits
    const SQUARE_MASK: u16 = 0b111111; // For origin and destination squares
    const FLAGS_MASK: u16 = 0b1111;

    pub fn new(origin_raw: u8, destination_raw: u8, flags_raw: u16) -> Self {
        let origin = (origin_raw as u16) << Move::ORIGIN_SHIFT;
        let destination = (destination_raw as u16) << Move::DESTINATION_SHIFT;
        let flags = flags_raw << Move::FLAGS_SHIFT;
        Move(origin | destination | flags)
    }

    pub fn origin(self) -> u8 {
        ((self.0 >> Move::ORIGIN_SHIFT) & Move::SQUARE_MASK) as u8
    }
    pub fn destination(self) -> u8 {
        ((self.0 >> Move::DESTINATION_SHIFT) & Move::SQUARE_MASK) as u8
    }

    fn flags(self) -> u16 {
        (self.0 >> Move::FLAGS_SHIFT) & Move::FLAGS_MASK
    }
    pub fn is_double_pawn_push(self) -> bool {
        self.flags() == flags::DOUBLE_PAWN_PUSH
    }
    pub fn is_queen_castling(self) -> bool {
        self.flags() == flags::QUEEN_CASTLE
    }
    pub fn is_king_castling(self) -> bool {
        self.flags() == flags::KING_CASTLE
    }
    pub fn is_castling(self) -> bool {
        self.is_queen_castling() || self.is_king_castling()
    }
    pub fn is_capture(self) -> bool {
        self.flags() & 0b0100 != 0
    }
    pub fn is_promotion(self) -> bool {
        self.flags() & 0b1000 != 0
    }
}
