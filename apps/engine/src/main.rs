use engine_lib::move_gen::attacks::ray_is_attacked;
use engine_lib::prelude::*;

fn main() {
    let mut board = Board::starting_position();
    let mg_tm = CopyMakeTransition::new();
    let s_tm = CopyMakeTransition::new();
    let mut tm = CopyMakeTransition::new();

    let mg = NaiveMoveGenerator::new(mg_tm, ray_is_attacked);

    let evaluator = RandomEvaluator::new();
    let mut searcher = PureNegamaxSearcher::new(s_tm, mg, evaluator, 3, ray_is_attacked);
    for ply in 0..1000 {
        println!("{}", &board);
        println!("Side to move: {:?}", board.to_move());

        let result = searcher.start_search(&mut board);
        let Some(best_move) = result.best_move else {
            let side = board.to_move();
            let king_square = board
                .bitboard(side, PieceKind::King)
                .lsb()
                .map(Square::from_index);
            let terminal_state = match king_square {
                Some(square) => {
                    if ray_is_attacked(&board, square, side.opponent()) {
                        "checkmate"
                    } else {
                        "stalemate"
                    }
                }
                None => "invalid position (missing king bitboard)",
            };

            println!(
                "No legal moves at ply {}. Final score: {} ({})",
                ply, result.score, terminal_state
            );
            break;
        };

        tm.make(&mut board, best_move);
    }
}
