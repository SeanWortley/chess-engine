use super::{PieceKind, Square};
// 16 bits for move encoding, per the wiki :)
// Very solid case to be made for 32 bit, but I lack the expertise to reason beyond the established starting path of 16 bits

/// Bit layout (low to high):
/// [0..5]   from/origin square (6 bits)
/// [6..11]  to/destination square (6 bits)
/// [12..15] flags (4 bits)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Move(u16);

/// Ripped straight from the wiki babyyyyyy
pub mod flags {
    pub const QUIET: u16 = 0b0000; // Not special
    pub const DOUBLE_PAWN_PUSH: u16 = 0b0001;
    pub const KING_CASTLE: u16 = 0b0010; // Don't need this for standard chess, but until I know better, I'll just follow the wiki :)
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

// For use in Board.apply()
#[derive(Debug, PartialEq)]
pub enum MoveKind {
    Quiet,
    DoublePawnPush,
    KingCastle,
    QueenCastle,
    Capture,
    EnPassantCapture,
    Promotion(PieceKind),
    PromotionCapture(PieceKind),
}

impl Default for Move {
    fn default() -> Self {
        Move::none()
    }
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
    fn none() -> Self {
        Move(0)
    }
    pub fn quiet(origin: Square, destination: Square) -> Self {
        Move::new(origin.index(), destination.index(), flags::QUIET)
    }
    pub fn double_pawn_push(origin: Square, destination: Square) -> Self {
        Move::new(origin.index(), destination.index(), flags::DOUBLE_PAWN_PUSH)
    }
    pub fn capture(origin: Square, destination: Square) -> Self {
        Move::new(origin.index(), destination.index(), flags::CAPTURE)
    }
    pub fn en_passant(origin: Square, destination: Square) -> Self {
        Move::new(origin.index(), destination.index(), flags::EP_CAPTURE)
    }
    pub fn king_castle(origin: Square, destination: Square) -> Self {
        Move::new(origin.index(), destination.index(), flags::KING_CASTLE)
    }
    pub fn queen_castle(origin: Square, destination: Square) -> Self {
        Move::new(origin.index(), destination.index(), flags::QUEEN_CASTLE)
    }
    pub fn promotion(origin: Square, destination: Square, piece: PieceKind) -> Self {
        let flag = match piece {
            PieceKind::Knight => flags::KNIGHT_PROMOTION,
            PieceKind::Bishop => flags::BISHOP_PROMOTION,
            PieceKind::Rook => flags::ROOK_PROMOTION,
            PieceKind::Queen => flags::QUEEN_PROMOTION,
            _ => panic!("Invalid promotion piece"),
        };
        Move::new(origin.index(), destination.index(), flag)
    }
    pub fn promotion_capture(origin: Square, destination: Square, piece: PieceKind) -> Self {
        let flag = match piece {
            PieceKind::Knight => flags::KNIGHT_PROMO_CAPTURE,
            PieceKind::Bishop => flags::BISHOP_PROMO_CAPTURE,
            PieceKind::Rook => flags::ROOK_PROMO_CAPTURE,
            PieceKind::Queen => flags::QUEEN_PROMO_CAPTURE,
            _ => panic!("Invalid promotion piece"),
        };
        Move::new(origin.index(), destination.index(), flag)
    }

    pub fn origin(self) -> Square {
        Square::from_index(((self.0 >> Move::ORIGIN_SHIFT) & Move::SQUARE_MASK) as u8)
    }
    pub fn destination(self) -> Square {
        Square::from_index(((self.0 >> Move::DESTINATION_SHIFT) & Move::SQUARE_MASK) as u8)
    }
    fn flags(self) -> u16 {
        (self.0 >> Move::FLAGS_SHIFT) & Move::FLAGS_MASK
    }
    pub fn kind(self) -> MoveKind {
        match self.flags() {
            flags::QUIET => MoveKind::Quiet,
            flags::DOUBLE_PAWN_PUSH => MoveKind::DoublePawnPush,
            flags::KING_CASTLE => MoveKind::KingCastle,
            flags::QUEEN_CASTLE => MoveKind::QueenCastle,
            flags::CAPTURE => MoveKind::Capture,
            flags::EP_CAPTURE => MoveKind::EnPassantCapture,
            flags::KNIGHT_PROMOTION => MoveKind::Promotion(PieceKind::Knight),
            flags::BISHOP_PROMOTION => MoveKind::Promotion(PieceKind::Bishop),
            flags::ROOK_PROMOTION => MoveKind::Promotion(PieceKind::Rook),
            flags::QUEEN_PROMOTION => MoveKind::Promotion(PieceKind::Queen),
            flags::KNIGHT_PROMO_CAPTURE => MoveKind::PromotionCapture(PieceKind::Knight),
            flags::BISHOP_PROMO_CAPTURE => MoveKind::PromotionCapture(PieceKind::Bishop),
            flags::ROOK_PROMO_CAPTURE => MoveKind::PromotionCapture(PieceKind::Rook),
            flags::QUEEN_PROMO_CAPTURE => MoveKind::PromotionCapture(PieceKind::Queen),
            _ => panic!("Invalid flags"),
        }
    }
    pub fn is_none(self) -> bool {
        self.0 == 0
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

    // Don't care about valid origin-destination pairs right now
    fn origin() -> Square {
        Square::from_index(0)
    }
    fn destination() -> Square {
        Square::from_index(12)
    }

    #[test]
    fn test_quiet() {
        assert!(!Move::quiet(origin(), destination()).is_capture());
        assert!(!Move::quiet(origin(), destination()).is_promotion());
        assert!(!Move::quiet(origin(), destination()).is_castling());
        assert!(!Move::quiet(origin(), destination()).is_double_pawn_push());
    }
    #[test]
    fn test_double_pawn_push() {
        assert!(Move::double_pawn_push(origin(), destination()).is_double_pawn_push());
    }
    #[test]
    fn test_capture() {
        assert!(Move::capture(origin(), destination()).is_capture());
        assert!(Move::en_passant(origin(), destination()).is_capture());
    }
    #[test]
    fn test_castling() {
        assert!(Move::king_castle(origin(), destination()).is_castling());
        assert!(Move::queen_castle(origin(), destination()).is_castling());
    }
    #[test]
    fn test_promotion() {
        let piece = PieceKind::Queen;
        assert!(Move::promotion(origin(), destination(), piece).is_promotion());
        assert!(!Move::promotion(origin(), destination(), piece).is_capture());
    }
    #[test]
    fn test_promotion_capture() {
        let piece = PieceKind::Queen;
        assert!(Move::promotion_capture(origin(), destination(), piece).is_promotion());
        assert!(Move::promotion_capture(origin(), destination(), piece).is_capture());
    }

    #[test]
    fn test_kind() {
        assert_eq!(Move::quiet(origin(), destination()).kind(), MoveKind::Quiet);
        assert_eq!(
            Move::double_pawn_push(origin(), destination()).kind(),
            MoveKind::DoublePawnPush
        );
        assert_eq!(
            Move::capture(origin(), destination()).kind(),
            MoveKind::Capture
        );
        assert_eq!(
            Move::en_passant(origin(), destination()).kind(),
            MoveKind::EnPassantCapture
        );
        assert_eq!(
            Move::king_castle(origin(), destination()).kind(),
            MoveKind::KingCastle
        );
        assert_eq!(
            Move::queen_castle(origin(), destination()).kind(),
            MoveKind::QueenCastle
        );
        assert_eq!(
            Move::promotion(origin(), destination(), PieceKind::Queen).kind(),
            MoveKind::Promotion(PieceKind::Queen)
        );
        assert_eq!(
            Move::promotion_capture(origin(), destination(), PieceKind::Knight).kind(),
            MoveKind::PromotionCapture(PieceKind::Knight)
        );
    }
}
