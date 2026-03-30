use crate::core::board::*;
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

    fn new(origin_raw: u8, destination_raw: u8, flags_raw: u16) -> Self {
        let origin = (origin_raw as u16) << Move::ORIGIN_SHIFT;
        let destination = (destination_raw as u16) << Move::DESTINATION_SHIFT;
        let flags = flags_raw << Move::FLAGS_SHIFT;
        Move(origin | destination | flags)
    }
    pub fn quiet(origin: u8, destination: u8) -> Self {
        Move::new(origin, destination, flags::QUIET)
    }
    pub fn double_pawn_push(origin: u8, destination: u8) -> Self {
        Move::new(origin, destination, flags::DOUBLE_PAWN_PUSH)
    }
    pub fn capture(origin: u8, destination: u8) -> Self {
        Move::new(origin, destination, flags::CAPTURE)
    }
    pub fn en_passant(origin: u8, destination: u8) -> Self {
        Move::new(origin, destination, flags::EP_CAPTURE)
    }
    pub fn king_castle(origin: u8, destination: u8) -> Self {
        Move::new(origin, destination, flags::KING_CASTLE)
    }
    pub fn queen_castle(origin: u8, destination: u8) -> Self {
        Move::new(origin, destination, flags::QUEEN_CASTLE)
    }
    pub fn promotion(origin: u8, destination: u8, piece: PieceKind) -> Self {
        let flag = match piece {
            PieceKind::Knight => flags::KNIGHT_PROMOTION,
            PieceKind::Bishop => flags::BISHOP_PROMOTION,
            PieceKind::Rook => flags::ROOK_PROMOTION,
            PieceKind::Queen => flags::QUEEN_PROMOTION,
            _ => panic!("Invalid promotion piece"),
        };
        Move::new(origin, destination, flag)
    }
    pub fn promotion_capture(origin: u8, destination: u8, piece: PieceKind) -> Self {
        let flag = match piece {
            PieceKind::Knight => flags::KNIGHT_PROMO_CAPTURE,
            PieceKind::Bishop => flags::BISHOP_PROMO_CAPTURE,
            PieceKind::Rook => flags::ROOK_PROMO_CAPTURE,
            PieceKind::Queen => flags::QUEEN_PROMO_CAPTURE,
            _ => panic!("Invalid promotion piece"),
        };
        Move::new(origin, destination, flag)
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quiet() {
        let origin: u8 = 0;
        let destination: u8 = 12;
        assert!(!Move::quiet(origin, destination).is_capture());
        assert!(!Move::quiet(origin, destination).is_promotion());
        assert!(!Move::quiet(origin, destination).is_castling());
        assert!(!Move::quiet(origin, destination).is_double_pawn_push());
    }

    #[test]
    fn test_double_pawn_push() {
        assert!(Move::double_pawn_push(0, 12).is_double_pawn_push());
    }

    #[test]
    fn test_capture() {
        assert!(Move::capture(0, 12).is_capture());
        assert!(Move::en_passant(0, 12).is_capture());
    }

    #[test]
    fn test_castling() {
        assert!(Move::king_castle(0, 12).is_castling());
        assert!(Move::queen_castle(0, 12).is_castling());
    }

    #[test]
    fn test_promotion() {
        let piece = PieceKind::Queen;
        assert!(Move::promotion(0, 12, piece).is_promotion());
        assert!(!Move::promotion(0, 12, piece).is_capture());
    }

    #[test]
    fn test_promotion_capture() {
        let piece = PieceKind::Queen;
        assert!(Move::promotion_capture(0, 12, piece).is_promotion());
        assert!(Move::promotion_capture(0, 12, piece).is_capture());
    }
}
