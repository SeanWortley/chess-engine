use crate::{Board, Evaluator, search::LeafPolicy};

pub struct StaticLeaf<E: Evaluator> {
    e: E,
}

impl<E: Evaluator> StaticLeaf<E> {
    pub fn new(evaluator: E) -> Self {
        Self { e: evaluator }
    }
}

impl<E: Evaluator> LeafPolicy for StaticLeaf<E> {
    fn evaluate_leaf(&mut self, board: &mut Board, _alpha: i16, _beta: i16) -> i16 {
        self.e.evaluate(board)
    }
}
