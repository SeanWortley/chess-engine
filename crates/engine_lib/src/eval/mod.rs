use crate::Board;

pub mod material;
pub mod pesto;
pub mod random;

pub use random::RandomEvaluator;

pub const NEG_INF: i16 = -30_000;
pub const POS_INF: i16 = 30_000;
pub const DRAW: i16 = 0;

pub const MATE_THRESHOLD: i16 = 29_000;

pub trait Evaluator {
    fn evaluate(&self, board: &Board) -> i16;
}
