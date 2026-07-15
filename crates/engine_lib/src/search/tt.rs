use crate::{Board, Move, eval::MATE_THRESHOLD};

pub fn to_tt_score(score: i16, root_distance: u8) -> i16 {
    if score >= MATE_THRESHOLD {
        return score + root_distance as i16;
    }

    if score <= -MATE_THRESHOLD {
        return score - root_distance as i16;
    }

    score
}

pub fn from_tt_score(score: i16, root_distance: u8) -> i16 {
    if score >= MATE_THRESHOLD {
        return score - root_distance as i16;
    }

    if score <= -MATE_THRESHOLD {
        return score + root_distance as i16;
    }

    score
}

pub struct SearchContext {
    pub tt: Option<TranspositionTable>,
    pub history: Vec<u64>,
}

impl SearchContext {
    pub fn with_tt(size_mb: usize) -> Self {
        SearchContext {
            tt: Some(TranspositionTable::new(size_mb)),
            history: Vec::new(),
        }
    }

    pub fn without_tt() -> Self {
        SearchContext {
            tt: None,
            history: Vec::new(),
        }
    }

    pub fn is_draw_by_rule(&self, board: &Board) -> bool {
        if board.halfmove_clock() >= 100 {
            return true;
        }

        let current = board.hash();

        // steps backwards through history untio last capture or pawn push, until it finds a
        // repetition then returns true on repition, false on no repition found. Only need to check
        // a single repetition here :)
        self.history
            .iter()
            .rev()
            .skip(2)
            .step_by(2)
            .take(board.halfmove_clock() as usize / 2)
            .any(|&hash| hash == current)
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

#[cfg(test)]
mod tests {
    use super::*;

    // Bare-kings position; only hash() and halfmove_clock() matter here.
    fn board_with_clock(halfmove_clock: u8) -> Board {
        Board::from_fen(&format!("8/4k3/8/8/8/8/4K3/8 w - - {} 40", halfmove_clock))
    }

    fn context(history: Vec<u64>) -> SearchContext {
        SearchContext { tt: None, history }
    }

    #[test]
    fn test_repetition_two_plies_back_is_draw() {
        let board = board_with_clock(8);
        let current = board.hash();
        // [repeat, opponent, current] - current position already seen two plies ago
        let ctx = context(vec![current, 0xBEEF, current]);
        assert!(ctx.is_draw_by_rule(&board));
    }

    #[test]
    fn test_same_hash_in_opponent_slot_is_ignored() {
        let board = board_with_clock(8);
        let current = board.hash();
        // Matching hash one ply back (opponent to move) - wrong parity, not a repetition
        let ctx = context(vec![0xBEEF, current, current]);
        assert!(!ctx.is_draw_by_rule(&board));
    }

    #[test]
    fn test_repetition_outside_halfmove_window_is_ignored() {
        // Match sits four plies back, but halfmove_clock = 2 means an irreversible
        // move happened since then - that repetition is impossible
        let board = board_with_clock(2);
        let current = board.hash();
        let ctx = context(vec![current, 0xB, 0xC, 0xD, current]);
        assert!(!ctx.is_draw_by_rule(&board));
    }

    #[test]
    fn test_repetition_inside_halfmove_window_is_found() {
        // Same history as above, but the window now reaches four plies back
        let board = board_with_clock(4);
        let current = board.hash();
        let ctx = context(vec![current, 0xB, 0xC, 0xD, current]);
        assert!(ctx.is_draw_by_rule(&board));
    }

    #[test]
    fn test_short_history_is_not_a_draw() {
        let board = board_with_clock(8);
        let current = board.hash();
        assert!(!context(vec![current]).is_draw_by_rule(&board));
        assert!(!context(vec![0xBEEF, current]).is_draw_by_rule(&board));
    }

    #[test]
    fn test_fifty_move_rule() {
        let board = board_with_clock(100);
        assert!(context(vec![board.hash()]).is_draw_by_rule(&board));

        let board = board_with_clock(99);
        assert!(!context(vec![board.hash()]).is_draw_by_rule(&board));
    }

    #[test]
    fn test_tt_score_frame_conversion() {
        // Node at ply 3 sees mate at root-ply 7 (score 29_993). Stored
        // node-relative it must say "mate 4 plies from here" (29_996), and a
        // different path probing at ply 5 must read mate at root-ply 9.
        let stored = to_tt_score(29_993, 3);
        assert_eq!(stored, 29_996);
        assert_eq!(from_tt_score(stored, 5), 29_991);

        // Mirror for the losing side
        assert_eq!(from_tt_score(to_tt_score(-29_993, 3), 5), -29_991);

        // Ordinary scores pass through untouched
        assert_eq!(to_tt_score(150, 7), 150);
        assert_eq!(from_tt_score(-150, 7), -150);
        assert_eq!(from_tt_score(to_tt_score(0, 9), 9), 0);
    }
}
