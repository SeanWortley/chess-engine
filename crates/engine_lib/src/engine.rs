use crate::{
    Board, CopyMakeTransition, Evaluator, Move, MoveGenerator, MoveList, NaiveMoveGenerator,
    PureNegamaxSearcher, RandomEvaluator, SearchResult, Searcher,
    eval::{material::MaterialEvaluator, pesto::PestoEvaluator},
    move_gen::attacks::ray_is_attacked,
    move_ordering::{MvvLva, NoOrdering},
    search::{
        OrderingPolicy, SearchDriver, SearchReporter,
        alpha_beta::AlphaBetaSearcher,
        control::{SearchConstraint, SearchControl},
        quiescent_leaf::QuiescentLeaf,
        static_leaf::StaticLeaf,
    },
};

// V1: CopyMakeTransition + PureNegamaxSearcher + NaiveMoveGenerator + RandomEvaluator + NoOrdering.
pub type V1Engine = Engine<
    SearchDriver<
        PureNegamaxSearcher<
            CopyMakeTransition,
            NaiveMoveGenerator<CopyMakeTransition>,
            StaticLeaf<RandomEvaluator>,
            NoOrdering,
        >,
    >,
    NaiveMoveGenerator<CopyMakeTransition>,
    RandomEvaluator,
    NoOrdering,
>;

// V2: CopyMakeTransition + PureNegamaxSearcher + NaiveMoveGenerator + MaterialEvaluator + NoOrdering.
pub type V2Engine = Engine<
    SearchDriver<
        PureNegamaxSearcher<
            CopyMakeTransition,
            NaiveMoveGenerator<CopyMakeTransition>,
            StaticLeaf<MaterialEvaluator>,
            NoOrdering,
        >,
    >,
    NaiveMoveGenerator<CopyMakeTransition>,
    MaterialEvaluator,
    NoOrdering,
>;

// V3: CopyMakeTransition + DeepeningSearcher(PureNegamax core) + NaiveMoveGenerator + MaterialEvaluator + NoOrdering.
pub type V3Engine = Engine<
    SearchDriver<
        PureNegamaxSearcher<
            CopyMakeTransition,
            NaiveMoveGenerator<CopyMakeTransition>,
            StaticLeaf<MaterialEvaluator>,
            NoOrdering,
        >,
    >,
    NaiveMoveGenerator<CopyMakeTransition>,
    MaterialEvaluator,
    NoOrdering,
>;

// V4: CopyMakeTransition + DeepeningSearcher(AlphaBeta core) + NaiveMoveGenerator + MaterialEvaluator + NoOrdering.
pub type V4Engine = Engine<
    SearchDriver<
        AlphaBetaSearcher<
            CopyMakeTransition,
            NaiveMoveGenerator<CopyMakeTransition>,
            StaticLeaf<MaterialEvaluator>,
            NoOrdering,
        >,
    >,
    NaiveMoveGenerator<CopyMakeTransition>,
    MaterialEvaluator,
    NoOrdering,
>;

pub type V5Engine = Engine<
    SearchDriver<
        AlphaBetaSearcher<
            CopyMakeTransition,
            NaiveMoveGenerator<CopyMakeTransition>,
            StaticLeaf<PestoEvaluator>,
            NoOrdering,
        >,
    >,
    NaiveMoveGenerator<CopyMakeTransition>,
    PestoEvaluator,
    NoOrdering,
>;

pub type V6Engine = Engine<
    SearchDriver<
        AlphaBetaSearcher<
            CopyMakeTransition,
            NaiveMoveGenerator<CopyMakeTransition>,
            StaticLeaf<PestoEvaluator>,
            MvvLva,
        >,
    >,
    NaiveMoveGenerator<CopyMakeTransition>,
    PestoEvaluator,
    MvvLva,
>;

pub type V7Engine = Engine<
    SearchDriver<
        AlphaBetaSearcher<
            CopyMakeTransition,
            NaiveMoveGenerator<CopyMakeTransition>,
            StaticLeaf<PestoEvaluator>,
            MvvLva,
        >,
    >,
    NaiveMoveGenerator<CopyMakeTransition>,
    PestoEvaluator,
    MvvLva,
>;

// V8: V7.1 composition with QuiescentLeaf replacing StaticLeaf.
pub type V8Engine = Engine<
    SearchDriver<
        AlphaBetaSearcher<
            CopyMakeTransition,
            NaiveMoveGenerator<CopyMakeTransition>,
            QuiescentLeaf<
                CopyMakeTransition,
                NaiveMoveGenerator<CopyMakeTransition>,
                PestoEvaluator,
                MvvLva,
            >,
            MvvLva,
        >,
    >,
    NaiveMoveGenerator<CopyMakeTransition>,
    PestoEvaluator,
    MvvLva,
>;

impl V8Engine {
    pub fn v8() -> Self {
        let mg = NaiveMoveGenerator::new(CopyMakeTransition::new(), ray_is_attacked);
        let evaluator = PestoEvaluator::new();
        let ordering = MvvLva {};

        let leaf = QuiescentLeaf::new(
            CopyMakeTransition::new(),
            NaiveMoveGenerator::new(CopyMakeTransition::new(), ray_is_attacked),
            PestoEvaluator::new(),
            MvvLva {},
        );

        let core = AlphaBetaSearcher::new(
            CopyMakeTransition::new(),
            NaiveMoveGenerator::new(CopyMakeTransition::new(), ray_is_attacked),
            leaf,
            ordering.clone(),
            ray_is_attacked,
        );

        let searcher = SearchDriver::iterative_tt(core);

        Engine::new(searcher, mg, evaluator, ordering)
    }
}

impl V2Engine {
    pub fn v2() -> Self {
        let mg = NaiveMoveGenerator::new(CopyMakeTransition::new(), ray_is_attacked);
        let evaluator = MaterialEvaluator::new();
        let ordering = NoOrdering {};
        let core = PureNegamaxSearcher::new(
            CopyMakeTransition::new(),
            NaiveMoveGenerator::new(CopyMakeTransition::new(), ray_is_attacked),
            StaticLeaf::new(MaterialEvaluator::new()),
            ordering.clone(),
            ray_is_attacked,
        );
        let searcher = SearchDriver::fixed_no_tt(core);
        Engine::new(searcher, mg, evaluator, ordering)
    }
}
impl V1Engine {
    pub fn v1() -> Self {
        let mg = NaiveMoveGenerator::new(CopyMakeTransition::new(), ray_is_attacked);
        let evaluator = RandomEvaluator::new();
        let ordering = NoOrdering {};
        let core = PureNegamaxSearcher::new(
            CopyMakeTransition::new(),
            NaiveMoveGenerator::new(CopyMakeTransition::new(), ray_is_attacked),
            StaticLeaf::new(RandomEvaluator::new()),
            ordering.clone(),
            ray_is_attacked,
        );
        let searcher = SearchDriver::fixed_no_tt(core);
        Engine::new(searcher, mg, evaluator, ordering)
    }
}

impl V3Engine {
    pub fn v3() -> Self {
        let mg = NaiveMoveGenerator::new(CopyMakeTransition::new(), ray_is_attacked);
        let evaluator = MaterialEvaluator::new();
        let ordering = NoOrdering {};
        let core = PureNegamaxSearcher::new(
            CopyMakeTransition::new(),
            NaiveMoveGenerator::new(CopyMakeTransition::new(), ray_is_attacked),
            StaticLeaf::new(MaterialEvaluator::new()),
            ordering.clone(),
            ray_is_attacked,
        );
        let searcher = SearchDriver::iterative_no_tt(core);

        Engine::new(searcher, mg, evaluator, ordering)
    }
}

impl V4Engine {
    pub fn v4() -> Self {
        let mg = NaiveMoveGenerator::new(CopyMakeTransition::new(), ray_is_attacked);
        let evaluator = MaterialEvaluator::new();
        let ordering = NoOrdering {};
        let core = AlphaBetaSearcher::new(
            CopyMakeTransition::new(),
            NaiveMoveGenerator::new(CopyMakeTransition::new(), ray_is_attacked),
            StaticLeaf::new(MaterialEvaluator::new()),
            ordering.clone(),
            ray_is_attacked,
        );
        let searcher = SearchDriver::iterative_no_tt(core);

        Engine::new(searcher, mg, evaluator, ordering)
    }
}

impl V5Engine {
    pub fn v5() -> Self {
        let mg = NaiveMoveGenerator::new(CopyMakeTransition::new(), ray_is_attacked);
        let evaluator = PestoEvaluator::new();
        let ordering = NoOrdering {};

        let core = AlphaBetaSearcher::new(
            CopyMakeTransition::new(),
            NaiveMoveGenerator::new(CopyMakeTransition::new(), ray_is_attacked),
            StaticLeaf::new(PestoEvaluator::new()),
            ordering.clone(),
            ray_is_attacked,
        );

        let searcher = SearchDriver::iterative_no_tt(core);

        Engine::new(searcher, mg, evaluator, ordering)
    }
}

impl V6Engine {
    pub fn v6() -> Self {
        let mg = NaiveMoveGenerator::new(CopyMakeTransition::new(), ray_is_attacked);
        let evaluator = PestoEvaluator::new();
        let ordering = MvvLva {};

        let core = AlphaBetaSearcher::new(
            CopyMakeTransition::new(),
            NaiveMoveGenerator::new(CopyMakeTransition::new(), ray_is_attacked),
            StaticLeaf::new(PestoEvaluator::new()),
            ordering.clone(),
            ray_is_attacked,
        );

        let searcher = SearchDriver::iterative_no_tt(core);

        Engine::new(searcher, mg, evaluator, ordering)
    }
}

impl V7Engine {
    pub fn v7() -> Self {
        let mg = NaiveMoveGenerator::new(CopyMakeTransition::new(), ray_is_attacked);
        let evaluator = PestoEvaluator::new();
        let ordering = MvvLva {};

        let core = AlphaBetaSearcher::new(
            CopyMakeTransition::new(),
            NaiveMoveGenerator::new(CopyMakeTransition::new(), ray_is_attacked),
            StaticLeaf::new(PestoEvaluator::new()),
            ordering.clone(),
            ray_is_attacked,
        );

        let searcher = SearchDriver::iterative_tt(core);

        Engine::new(searcher, mg, evaluator, ordering)
    }
}

pub struct Engine<S, MG, E, OP>
where
    S: Searcher,
    MG: MoveGenerator,
    E: Evaluator,
    OP: OrderingPolicy,
{
    searcher: S,
    move_generator: MG,
    evaluator: E,
    ordering_policy: OP,
}

impl<S, MG, E, OP> Engine<S, MG, E, OP>
where
    S: Searcher,
    MG: MoveGenerator,
    E: Evaluator,
    OP: OrderingPolicy,
{
    pub fn new(searcher: S, move_generator: MG, evaluator: E, ordering_policy: OP) -> Self {
        Engine {
            searcher,
            move_generator,
            evaluator,
            ordering_policy,
        }
    }

    pub fn search(
        &mut self,
        board: &mut Board,
        game_history: &[u64],
        constraint: SearchConstraint,
        control: SearchControl,
        reporter: &dyn SearchReporter,
    ) -> SearchResult {
        self.searcher
            .start_search(board, game_history, constraint, &control, reporter)
    }

    pub fn generate_moves(&mut self, board: &mut Board) -> MoveList {
        let mut moves = MoveList::new();
        self.move_generator.generate_moves(board, &mut moves, true);
        moves
    }

    pub fn order_moves(&mut self, board: &mut Board, moves: &mut MoveList) {
        self.ordering_policy.order_moves(board, moves);
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
