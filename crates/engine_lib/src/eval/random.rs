use rand::random;

use crate::core::board::Board;
use crate::eval::Evaluator;

pub struct RandomEvaluator;

impl Evaluator for RandomEvaluator {
    fn evaluate(&self, _board: &Board) -> i16 {
        let evaluation: i16 = random();
        evaluation
    }
}

impl RandomEvaluator {
    pub fn new() -> Self {
        RandomEvaluator
    }
}
