pub mod core;
pub mod engine;
pub mod eval;
pub mod move_gen;
pub mod search;
pub mod transition;

pub use core::{
    Bitboard, Board, CastlingRights, Color, Direction, Move, MoveKind, MoveList, Piece, PieceKind,
    Square,
};
pub use engine::DefaultEngine;
pub use engine::Engine;
pub use eval::{DRAW, Evaluator, NEG_INF, POS_INF, RandomEvaluator};
pub use move_gen::{IsAttackedFn, MoveGenerator, NaiveMoveGenerator};
pub use search::{PureNegamaxSearcher, SearchResult, Searcher};
pub use transition::{CopyMakeTransition, TransitionManager};

pub mod prelude {
    pub use crate::{
        Bitboard, Board, CastlingRights, Color, CopyMakeTransition, DRAW, DefaultEngine, Direction,
        Engine, Evaluator, IsAttackedFn, Move, MoveGenerator, MoveKind, MoveList, NEG_INF,
        NaiveMoveGenerator, POS_INF, Piece, PieceKind, PureNegamaxSearcher, RandomEvaluator,
        SearchResult, Searcher, Square, TransitionManager,
    };
}
