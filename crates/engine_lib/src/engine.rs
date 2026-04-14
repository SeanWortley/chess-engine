use crate::{
    Board, CopyMakeTransition, Evaluator, MoveGenerator, MoveList, NaiveMoveGenerator,
    PureNegamaxSearcher, RandomEvaluator, SearchResult, Searcher,
    eval::material::MaterialEvaluator, move_gen::attacks::ray_is_attacked,
};

pub type BestEngine = Engine<
    PureNegamaxSearcher<
        CopyMakeTransition,
        NaiveMoveGenerator<CopyMakeTransition>,
        MaterialEvaluator,
    >,
    NaiveMoveGenerator<CopyMakeTransition>,
    MaterialEvaluator,
>;
pub type RandomEngine = Engine<
    PureNegamaxSearcher<
        CopyMakeTransition,
        NaiveMoveGenerator<CopyMakeTransition>,
        RandomEvaluator,
    >,
    NaiveMoveGenerator<CopyMakeTransition>,
    RandomEvaluator,
>;

impl BestEngine {
    pub fn best() -> Self {
        let mg = NaiveMoveGenerator::new(CopyMakeTransition::new(), ray_is_attacked);
        let evaluator = MaterialEvaluator::new();
        let searcher = PureNegamaxSearcher::new(
            CopyMakeTransition::new(),
            NaiveMoveGenerator::new(CopyMakeTransition::new(), ray_is_attacked),
            MaterialEvaluator::new(),
            6,
            ray_is_attacked,
        );
        Engine::new(searcher, mg, evaluator)
    }
}
impl RandomEngine {
    pub fn random() -> Self {
        let mg = NaiveMoveGenerator::new(CopyMakeTransition::new(), ray_is_attacked);
        let evaluator = RandomEvaluator::new();
        let searcher = PureNegamaxSearcher::new(
            CopyMakeTransition::new(),
            NaiveMoveGenerator::new(CopyMakeTransition::new(), ray_is_attacked),
            RandomEvaluator::new(),
            1,
            ray_is_attacked,
        );
        Engine::new(searcher, mg, evaluator)
    }
}

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
