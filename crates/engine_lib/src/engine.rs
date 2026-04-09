use crate::{Board, Evaluator, MoveGenerator, MoveList, SearchResult, Searcher};

pub struct Engine<S, MG, E>
where
    S: Searcher,
    MG: MoveGenerator,
    E: Evaluator,
{
    searcher: S,
    move_generator: MG,
    evaluator: E,
}

impl<S, MG, E> Engine<S, MG, E>
where
    S: Searcher,
    MG: MoveGenerator,
    E: Evaluator,
{
    pub fn new(searcher: S, move_generator: MG, evaluator: E) -> Self {
        Engine {
            searcher,
            move_generator,
            evaluator,
        }
    }

    pub fn search(&mut self, board: &mut Board) -> SearchResult {
        self.searcher.start_search(board)
    }

    pub fn generate_moves(&mut self, board: &mut Board) -> MoveList {
        let mut moves = MoveList::new();
        self.move_generator.generate_moves(board, &mut moves);
        moves
    }

    pub fn evaluate(&self, board: &Board) -> i16 {
        self.evaluator.evaluate(board)
    }
}
