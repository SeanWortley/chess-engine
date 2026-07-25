use crate::{
    Board, Move, MoveList,
    move_ordering::{OrderingContext, OrderingPolicy, scorer::MoveScorer},
};

#[derive(Clone, Copy)]
pub struct ScoredOrdering<MS: MoveScorer> {
    ms: MS,
}

impl<MS: MoveScorer> OrderingPolicy for ScoredOrdering<MS> {
    fn order_moves(&mut self, board: &mut Board, moves: &mut MoveList, context: OrderingContext) {
        moves.sort_by_key(|mv| self.ms.score_move(board, *mv, context).unwrap_or(i32::MIN));
    }

    fn on_beta_cutoff(&mut self, mv: Move, root_distance: u8, depth: u8) {
        self.ms.on_beta_cutoff(mv, root_distance, depth);
    }
}

impl<MS: MoveScorer> ScoredOrdering<MS> {
    pub fn new(ms: MS) -> Self {
        ScoredOrdering { ms }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Square,
        move_ordering::{mvv_lva::MvvLvaScorer, scorer::BAND, tt_move::TtMoveScorer},
    };

    // After 1.e4 d5: exd5 is a capture; Nf3 and a3 are quiet.
    const FEN: &str = "rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2";

    fn sq(name: &str) -> Square {
        Square::from_name(name)
    }

    fn ordered(ordering: &mut impl OrderingPolicy, tt_move: Option<Move>) -> Vec<Move> {
        let mut board = Board::from_fen(FEN);
        let mut moves = MoveList::new();
        moves.push(Move::quiet(sq("a2"), sq("a3")));
        moves.push(Move::capture(sq("e4"), sq("d5")));
        moves.push(Move::quiet(sq("g1"), sq("f3")));

        ordering.order_moves(&mut board, &mut moves, OrderingContext::new(tt_move, 0));
        moves.iter().copied().collect()
    }

    #[test]
    fn test_tt_move_first_then_captures_then_quiets() {
        let tt = Move::quiet(sq("g1"), sq("f3"));
        let mut ordering = ScoredOrdering {
            ms: (TtMoveScorer, MvvLvaScorer),
        };

        let order = ordered(&mut ordering, Some(tt));

        assert_eq!(order[0], tt, "quiet TT move must sort above captures");
        assert_eq!(order[1], Move::capture(sq("e4"), sq("d5")));
        assert_eq!(order[2], Move::quiet(sq("a2"), sq("a3")));
    }

    #[test]
    fn test_flipped_stack_puts_captures_above_tt_move() {
        // Same scorers, swapped priority: the user's "captures first" experiment.
        let tt = Move::quiet(sq("g1"), sq("f3"));
        let mut ordering = ScoredOrdering {
            ms: (MvvLvaScorer, TtMoveScorer),
        };

        let order = ordered(&mut ordering, Some(tt));

        assert_eq!(order[0], Move::capture(sq("e4"), sq("d5")));
        assert_eq!(order[1], tt, "TT move still outranks unclaimed quiets");
        assert_eq!(order[2], Move::quiet(sq("a2"), sq("a3")));
    }

    #[test]
    fn test_no_tt_move_leaves_captures_first() {
        let mut ordering = ScoredOrdering {
            ms: (TtMoveScorer, MvvLvaScorer),
        };

        let order = ordered(&mut ordering, None);

        assert_eq!(order[0], Move::capture(sq("e4"), sq("d5")));
    }

    // Passes on every move; counts cutoff notifications it receives.
    struct CountingScorer {
        cutoffs: u32,
    }
    impl MoveScorer for CountingScorer {
        fn score_move(&self, _b: &Board, _mv: Move, _c: OrderingContext) -> Option<i32> {
            None
        }
        fn on_beta_cutoff(&mut self, _mv: Move, _rd: u8, _d: u8) {
            self.cutoffs += 1;
        }
    }

    #[test]
    fn test_cutoff_hook_reaches_every_scorer_in_the_stack() {
        // If this breaks, a stateful scorer (killers/history) would sit on a
        // dead wire and silently learn nothing.
        let mut ordering = ScoredOrdering {
            ms: (CountingScorer { cutoffs: 0 }, CountingScorer { cutoffs: 0 }),
        };

        let mv = Move::quiet(sq("g1"), sq("f3"));
        ordering.on_beta_cutoff(mv, 3, 5);
        ordering.on_beta_cutoff(mv, 4, 5);

        assert_eq!(
            ordering.ms.0.cutoffs, 2,
            "first scorer missed cutoff notifications"
        );
        assert_eq!(
            ordering.ms.1.cutoffs, 2,
            "second scorer missed cutoff notifications"
        );
    }

    // Keep BAND referenced so the import stays honest if tests are trimmed.
    #[test]
    fn test_band_headroom_is_sane() {
        // Largest MVV-LVA-family score (promotion capture) must fit in a band.
        assert!(605 + 900 + 5 < BAND);
    }
}
