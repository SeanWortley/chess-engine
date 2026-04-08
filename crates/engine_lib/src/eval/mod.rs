use crate::core::board::Board;

pub mod random;

pub struct Evaluation(i16);

impl Evaluation {
    pub fn score(self) -> i16 {
        self.0
    }
}

pub trait Evaluator {
    fn evaluate(&self, board: &Board) -> Evaluation;
}
