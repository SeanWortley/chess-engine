use crate::{Board, Move, move_ordering::OrderingContext};
pub const BAND: i32 = 1 << 20;
pub trait MoveScorer {
    fn score_move(&self, board: &Board, mv: Move, context: OrderingContext) -> Option<i32>;
    fn on_beta_cutoff(&mut self, mv: Move, root_distance: u8, depth: u8);
}

impl<A: MoveScorer, B: MoveScorer> MoveScorer for (A, B) {
    fn score_move(&self, board: &Board, mv: Move, context: OrderingContext) -> Option<i32> {
        if let Some(score) = self.0.score_move(board, mv, context) {
            return Some(score + BAND);
        }
        self.1.score_move(board, mv, context)
    }

    fn on_beta_cutoff(&mut self, mv: Move, root_distance: u8, depth: u8) {
        self.0.on_beta_cutoff(mv, root_distance, depth);
        self.1.on_beta_cutoff(mv, root_distance, depth);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Square;

    // Claims exactly one move, with the given within-band score.
    struct ClaimOne {
        target: Move,
        score: i32,
    }
    impl MoveScorer for ClaimOne {
        fn score_move(&self, _b: &Board, mv: Move, _c: OrderingContext) -> Option<i32> {
            if mv == self.target { Some(self.score) } else { None }
        }
        fn on_beta_cutoff(&mut self, _mv: Move, _rd: u8, _d: u8) {}
    }

    // Claims every move with the given within-band score.
    struct ClaimAll(i32);
    impl MoveScorer for ClaimAll {
        fn score_move(&self, _b: &Board, _mv: Move, _c: OrderingContext) -> Option<i32> {
            Some(self.0)
        }
        fn on_beta_cutoff(&mut self, _mv: Move, _rd: u8, _d: u8) {}
    }

    fn mv(from: &str, to: &str) -> Move {
        Move::quiet(Square::from_name(from), Square::from_name(to))
    }

    #[test]
    fn test_earlier_scorer_outranks_later_despite_smaller_score() {
        // First scorer claims e2e4 weakly; second claims everything with the
        // largest legal within-band score. Position must beat magnitude.
        let special = mv("e2", "e4");
        let other = mv("a2", "a3");
        let stack = (
            ClaimOne { target: special, score: 1 },
            ClaimAll(BAND - 1),
        );

        let board = Board::starting_position();
        let ctx = OrderingContext::empty();
        let special_score = stack.score_move(&board, special, ctx).unwrap();
        let other_score = stack.score_move(&board, other, ctx).unwrap();

        assert!(
            special_score > other_score,
            "band position must outrank within-band magnitude: {special_score} vs {other_score}"
        );
    }

    #[test]
    fn test_earlier_scorer_shadows_later_for_same_move() {
        // Both scorers claim the same move; the earlier one's opinion wins.
        let target = mv("e2", "e4");
        let stack = (
            ClaimOne { target, score: 7 },
            ClaimAll(500),
        );
        let board = Board::starting_position();
        let score = stack.score_move(&board, target, ctx_empty()).unwrap();
        assert_eq!(score, 7 + BAND, "first claim wins; later opinions are shadowed");
    }

    #[test]
    fn test_unclaimed_move_falls_through_to_none() {
        let stack = (
            ClaimOne { target: mv("e2", "e4"), score: 1 },
            ClaimOne { target: mv("d2", "d4"), score: 1 },
        );
        let board = Board::starting_position();
        assert_eq!(stack.score_move(&board, mv("h2", "h3"), ctx_empty()), None);
    }

    fn ctx_empty() -> OrderingContext {
        OrderingContext::empty()
    }
}
