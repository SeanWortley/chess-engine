use crate::Board;
use crate::CastlingRights;
use crate::Color;
use crate::Square;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::sync::LazyLock;

pub const SEED: u64 = 1234;
pub static ZOBRIST: LazyLock<ZobristTable> = LazyLock::new(ZobristTable::new);

pub struct ZobristTable {
    pub pieces: [[[u64; 64]; 6]; 2], // Index with [piece_color][piece_kind][square]
    pub side_to_move: u64,
    pub castling_rights: [u64; 4], // KQkq
    pub en_passant: [u64; 8],      // One per file
}

impl ZobristTable {
    pub fn new() -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(SEED);

        let mut pieces = [[[0u64; 64]; 6]; 2];
        for color in 0..2 {
            for piece in 0..6 {
                for square in 0..64 {
                    pieces[color][piece][square] = rng.random::<u64>();
                }
            }
        }

        Self {
            pieces,
            side_to_move: rng.random::<u64>(),
            castling_rights: core::array::from_fn(|_| rng.random::<u64>()),
            en_passant: core::array::from_fn(|_| rng.random::<u64>()),
        }
    }

    pub fn compute_from_scratch(&self, board: &Board) -> u64 {
        // Can use mailbox here, since speed doesn't matter - only called once per search
        let mut hash = 0u64;

        // XOR Pieces
        for idx in 0..64 {
            if let Some(piece) = board.get_piece(Square::from_index(idx)) {
                hash ^= self.pieces[piece.color as usize][piece.kind as usize][idx as usize];
            }
        }

        // XOR Side to Move
        if board.to_move() == Color::Black {
            hash ^= self.side_to_move;
        }

        // XOR Castling Rights
        let castling_rights = board.rights();

        if castling_rights.has_rights(CastlingRights::WHITE_KINGSIDE) {
            hash ^= self.castling_rights[0]
        }
        if castling_rights.has_rights(CastlingRights::WHITE_QUEENSIDE) {
            hash ^= self.castling_rights[1]
        }
        if castling_rights.has_rights(CastlingRights::BLACK_KINGSIDE) {
            hash ^= self.castling_rights[2]
        }
        if castling_rights.has_rights(CastlingRights::BLACK_QUEENSIDE) {
            hash ^= self.castling_rights[3]
        }

        // XOR EP File
        if let Some(ep_square) = board.en_passant() {
            hash ^= self.en_passant[ep_square.file() as usize];
        }

        hash
    }
}
