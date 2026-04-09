use crate::core::board::Board;

pub mod random;

pub const NEG_INF: i16 = -30_000;
pub const POS_INF: i16 = 30_000;
pub const DRAW: i16 = 0;

pub trait Evaluator {
    fn evaluate(&self, board: &Board) -> i16;
}
