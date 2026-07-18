use crate::{
    Board, Evaluator, SearchControl,
    search::{LeafPolicy, metrics::SearchMetrics},
};

pub struct StaticLeaf<E: Evaluator> {
    e: E,
}

impl<E: Evaluator> StaticLeaf<E> {
    pub fn new(evaluator: E) -> Self {
        Self { e: evaluator }
    }
}

impl<E: Evaluator> LeafPolicy for StaticLeaf<E> {
    fn evaluate_leaf(
        &mut self,
        board: &mut Board,
        _alpha: i16,
        _beta: i16,
        _control: &SearchControl,
        _metrics: &mut SearchMetrics,
    ) -> i16 {
        self.e.evaluate(board)
    }
}
