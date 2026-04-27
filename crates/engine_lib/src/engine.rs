use crate::{
    Board, CopyMakeTransition, Evaluator, Move, MoveGenerator, MoveList, NaiveMoveGenerator,
    PureNegamaxSearcher, RandomEvaluator, SearchResult, Searcher,
    eval::{material::MaterialEvaluator, pesto::PestoEvaluator},
    move_gen::attacks::ray_is_attacked,
    search::{
        SearchDriver, SearchReporter,
        alpha_beta::AlphaBetaSearcher,
        control::{SearchConstraint, SearchControl},
        static_leaf::StaticLeaf,
    },
};

// V1: CopyMakeTransition + PureNegamaxSearcher + NaiveMoveGenerator + RandomEvaluator.
pub type V1Engine = Engine<
    SearchDriver<
        PureNegamaxSearcher<
            CopyMakeTransition,
            NaiveMoveGenerator<CopyMakeTransition>,
            StaticLeaf<RandomEvaluator>,
        >,
    >,
    NaiveMoveGenerator<CopyMakeTransition>,
    RandomEvaluator,
>;

// V2: CopyMakeTransition + PureNegamaxSearcher + NaiveMoveGenerator + MaterialEvaluator.
pub type V2Engine = Engine<
    SearchDriver<
        PureNegamaxSearcher<
            CopyMakeTransition,
            NaiveMoveGenerator<CopyMakeTransition>,
            StaticLeaf<MaterialEvaluator>,
        >,
    >,
    NaiveMoveGenerator<CopyMakeTransition>,
    MaterialEvaluator,
>;

// V3: CopyMakeTransition + DeepeningSearcher(PureNegamax core) + NaiveMoveGenerator + MaterialEvaluator.
pub type V3Engine = Engine<
    SearchDriver<
        PureNegamaxSearcher<
            CopyMakeTransition,
            NaiveMoveGenerator<CopyMakeTransition>,
            StaticLeaf<MaterialEvaluator>,
        >,
    >,
    NaiveMoveGenerator<CopyMakeTransition>,
    MaterialEvaluator,
>;

// V4: CopyMakeTransition + DeepeningSearcher(AlphaBeta core) + NaiveMoveGenerator + MaterialEvaluator.
pub type V4Engine = Engine<
    SearchDriver<
        AlphaBetaSearcher<
            CopyMakeTransition,
            NaiveMoveGenerator<CopyMakeTransition>,
            StaticLeaf<MaterialEvaluator>,
        >,
    >,
    NaiveMoveGenerator<CopyMakeTransition>,
    MaterialEvaluator,
>;

pub type V5Engine = Engine<
    SearchDriver<
        AlphaBetaSearcher<
            CopyMakeTransition,
            NaiveMoveGenerator<CopyMakeTransition>,
            StaticLeaf<PestoEvaluator>,
        >,
    >,
    NaiveMoveGenerator<CopyMakeTransition>,
    PestoEvaluator,
>;

impl V2Engine {
    pub fn v2() -> Self {
        let mg = NaiveMoveGenerator::new(CopyMakeTransition::new(), ray_is_attacked);
        let evaluator = MaterialEvaluator::new();
        let core = PureNegamaxSearcher::new(
            CopyMakeTransition::new(),
            NaiveMoveGenerator::new(CopyMakeTransition::new(), ray_is_attacked),
            StaticLeaf::new(MaterialEvaluator::new()),
            ray_is_attacked,
        );
        let searcher = SearchDriver::fixed(core);
        Engine::new(searcher, mg, evaluator)
    }
}
impl V1Engine {
    pub fn v1() -> Self {
        let mg = NaiveMoveGenerator::new(CopyMakeTransition::new(), ray_is_attacked);
        let evaluator = RandomEvaluator::new();
        let core = PureNegamaxSearcher::new(
            CopyMakeTransition::new(),
            NaiveMoveGenerator::new(CopyMakeTransition::new(), ray_is_attacked),
            StaticLeaf::new(RandomEvaluator::new()),
            ray_is_attacked,
        );
        let searcher = SearchDriver::fixed(core);
        Engine::new(searcher, mg, evaluator)
    }
}

impl V3Engine {
    pub fn v3() -> Self {
        let mg = NaiveMoveGenerator::new(CopyMakeTransition::new(), ray_is_attacked);
        let evaluator = MaterialEvaluator::new();
        let core = PureNegamaxSearcher::new(
            CopyMakeTransition::new(),
            NaiveMoveGenerator::new(CopyMakeTransition::new(), ray_is_attacked),
            StaticLeaf::new(MaterialEvaluator::new()),
            ray_is_attacked,
        );
        let searcher = SearchDriver::iterative(core);

        Engine::new(searcher, mg, evaluator)
    }
}

impl V4Engine {
    pub fn v4() -> Self {
        let mg = NaiveMoveGenerator::new(CopyMakeTransition::new(), ray_is_attacked);
        let evaluator = MaterialEvaluator::new();
        let core = AlphaBetaSearcher::new(
            CopyMakeTransition::new(),
            NaiveMoveGenerator::new(CopyMakeTransition::new(), ray_is_attacked),
            StaticLeaf::new(MaterialEvaluator::new()),
            ray_is_attacked,
        );
        let searcher = SearchDriver::iterative(core);

        Engine::new(searcher, mg, evaluator)
    }
}

impl V5Engine {
    pub fn v5() -> Self {
        let mg = NaiveMoveGenerator::new(CopyMakeTransition::new(), ray_is_attacked);
        let evaluator = PestoEvaluator::new();

        let core = AlphaBetaSearcher::new(
            CopyMakeTransition::new(),
            NaiveMoveGenerator::new(CopyMakeTransition::new(), ray_is_attacked),
            StaticLeaf::new(PestoEvaluator::new()),
            ray_is_attacked,
        );

        let searcher = SearchDriver::iterative(core);

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

    pub fn search(
        &mut self,
        board: &mut Board,
        constraint: SearchConstraint,
        control: SearchControl,
        reporter: &dyn SearchReporter,
    ) -> SearchResult {
        self.searcher
            .start_search(board, constraint, &control, reporter)
    }

    pub fn generate_moves(&mut self, board: &mut Board) -> MoveList {
        let mut moves = MoveList::new();
        self.move_generator.generate_moves(board, &mut moves);
        moves
    }

    pub fn evaluate(&self, board: &Board) -> i16 {
        self.evaluator.evaluate(board)
    }

    pub fn make(&mut self, board: &mut Board, mv: Move) {
        self.searcher.make(board, mv);
    }

    pub fn unmake(&mut self, board: &mut Board, mv: Move) {
        self.searcher.unmake(board, mv);
    }
}
