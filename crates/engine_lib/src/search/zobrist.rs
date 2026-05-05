use crate::Move;
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
}
