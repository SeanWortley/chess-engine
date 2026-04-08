use rand::random;

use crate::core::board::Board;
use crate::eval::{Evaluation, Evaluator};

pub struct RandomEvaluator;

impl Evaluator for RandomEvaluator {
    fn evaluate(&self, _board: &Board) -> Evaluation {
        let num: i16 = random();
        Evaluation(num)
    }
}

impl RandomEvaluator {
    pub fn new() -> Self {
        RandomEvaluator
    }
}
