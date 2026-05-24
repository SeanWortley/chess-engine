use crate::Board;
use crate::CastlingRights;
use crate::Color;
use crate::Move;
use crate::Square;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

pub const SEED: u64 = 1234;

pub struct SearchContext {
    pub tt: Option<TranspositionTable>,
    pub zobrist: Option<ZobristTable>,
}

impl SearchContext {
    pub fn with_tt(size_mb: usize) -> Self {
        SearchContext {
            tt: Some(TranspositionTable::new(size_mb)),
            zobrist: Some(ZobristTable::new()),
        }
    }

    pub fn without_tt() -> Self {
        SearchContext {
            tt: None,
            zobrist: None,
        }
    }
}

pub struct TranspositionTable {
    entries: Vec<Option<TTEntry>>, // This will be heap regardless, so might as well use a vec
    size: usize,
}

impl TranspositionTable {
    pub fn new(size_mb: usize) -> Self {
        let size = (size_mb * 1024 * 1024) / std::mem::size_of::<Option<TTEntry>>();
        let size = size.next_power_of_two();
        let entries: Vec<Option<TTEntry>> = vec![None; size];
        Self { entries, size }
    }

    fn index(&self, hash: u64) -> usize {
        (hash as usize) & (self.size - 1) // Bitwise AND - SUPA FAST 
    }

    pub fn probe(&self, hash: u64, depth: u8) -> Option<&TTEntry> {
        let entry = self.entries[self.index(hash)].as_ref();
        match entry {
            Some(entry) => {
                if entry.key == hash && entry.depth >= depth {
                    Some(entry)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    pub fn probe_move(&self, hash: u64) -> Option<Move> {
        let entry = self.entries[self.index(hash)].as_ref();
        match entry {
            Some(entry) => {
                if entry.key == hash {
                    Some(entry.best_move)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    pub fn store(&mut self, entry: TTEntry) {
        let idx = self.index(entry.key);
        self.entries[idx] = Some(entry);
    }
}

#[derive(Clone)]
pub struct TTEntry {
    pub key: u64, // hash
    pub score: i16,
    pub best_move: Move,
    pub depth: u8,
    pub flag: TTFlag,
}

#[derive(Clone)]
pub enum TTFlag {
    Exact,
    LowerBound,
    UpperBound,
}

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
