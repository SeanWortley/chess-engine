use engine_lib::{
    core::{board::Board, move_list::MoveList},
    move_gen::{MoveGenerator, naive::NaiveMoveGenerator},
    transition::{TransitionManager, copy_make::CopyMakeTransition},
};

fn perft<MG: MoveGenerator, TM: TransitionManager>(
    board: &mut Board,
    mg: &mut MG,
    tm: &mut TM,
    depth: u8,
) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut moves = MoveList::new();
    mg.generate_moves(board, &mut moves);

    let mut total = 0u64;
    for mv in moves.iter() {
        tm.make(board, *mv);
        total += perft(board, mg, tm, depth - 1);
        tm.unmake(board, *mv);
    }

    total
}

#[cfg(test)]
mod tests {
    use engine_lib::move_gen::attacks::ray_is_attacked;

    use super::*;
    #[test]
    fn perft_depth_1() {
        let mut board = Board::starting_position();
        let tm = CopyMakeTransition::new();
        let mut mg = NaiveMoveGenerator::new(tm, ray_is_attacked);
        let mut perft_tm = CopyMakeTransition::new();

        let result = perft(&mut board, &mut mg, &mut perft_tm, 1);
        assert_eq!(result, 20u64);
    }

    #[test]
    fn perft_depth_2() {
        let mut board = Board::starting_position();
        let tm = CopyMakeTransition::new();
        let mut mg = NaiveMoveGenerator::new(tm, ray_is_attacked);
        let mut perft_tm = CopyMakeTransition::new();

        let result = perft(&mut board, &mut mg, &mut perft_tm, 2);
        assert_eq!(result, 400u64);
    }

    #[test]
    fn perft_depth_3() {
        let mut board = Board::starting_position();
        let tm = CopyMakeTransition::new();
        let mut mg = NaiveMoveGenerator::new(tm, ray_is_attacked);
        let mut perft_tm = CopyMakeTransition::new();

        let result = perft(&mut board, &mut mg, &mut perft_tm, 3);
        assert_eq!(result, 8902u64);
    }

    #[test]
    fn perft_depth_4() {
        let mut board = Board::starting_position();
        let tm = CopyMakeTransition::new();
        let mut mg = NaiveMoveGenerator::new(tm, ray_is_attacked);
        let mut perft_tm = CopyMakeTransition::new();

        let result = perft(&mut board, &mut mg, &mut perft_tm, 4);
        assert_eq!(result, 197281u64);
    }

    #[test]
    #[ignore]
    fn perft_depth_5() {
        let mut board = Board::starting_position();
        let tm = CopyMakeTransition::new();
        let mut mg = NaiveMoveGenerator::new(tm, ray_is_attacked);
        let mut perft_tm = CopyMakeTransition::new();

        let result = perft(&mut board, &mut mg, &mut perft_tm, 5);
        assert_eq!(result, 4865609u64);
    }

    #[test]
    #[ignore]
    fn perft_depth_6() {
        let mut board = Board::starting_position();
        let tm = CopyMakeTransition::new();
        let mut mg = NaiveMoveGenerator::new(tm, ray_is_attacked);
        let mut perft_tm = CopyMakeTransition::new();

        let result = perft(&mut board, &mut mg, &mut perft_tm, 6);
        assert_eq!(result, 119060324u64);
    }

    #[test]
    fn kiwipete_depth_1() {
        let mut board =
            Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
        let tm = CopyMakeTransition::new();
        let mut mg = NaiveMoveGenerator::new(tm, ray_is_attacked);
        let mut perft_tm = CopyMakeTransition::new();

        let result = perft(&mut board, &mut mg, &mut perft_tm, 1);
        assert_eq!(result, 48u64);
    }

    #[test]
    fn kiwipete_depth_2() {
        let mut board =
            Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
        let tm = CopyMakeTransition::new();
        let mut mg = NaiveMoveGenerator::new(tm, ray_is_attacked);
        let mut perft_tm = CopyMakeTransition::new();

        let result = perft(&mut board, &mut mg, &mut perft_tm, 2);
        assert_eq!(result, 2039u64);
    }

    #[test]
    fn kiwipete_depth_3() {
        let mut board =
            Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
        let tm = CopyMakeTransition::new();
        let mut mg = NaiveMoveGenerator::new(tm, ray_is_attacked);
        let mut perft_tm = CopyMakeTransition::new();

        let result = perft(&mut board, &mut mg, &mut perft_tm, 3);
        assert_eq!(result, 97862u64);
    }

    #[test]
    #[ignore]
    fn kiwipete_depth_4() {
        let mut board =
            Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
        let tm = CopyMakeTransition::new();
        let mut mg = NaiveMoveGenerator::new(tm, ray_is_attacked);
        let mut perft_tm = CopyMakeTransition::new();

        let result = perft(&mut board, &mut mg, &mut perft_tm, 4);
        assert_eq!(result, 4085603u64);
    }
}
