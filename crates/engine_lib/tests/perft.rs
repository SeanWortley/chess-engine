use engine_lib::{
    core::{board::Board, move_list::MoveList},
    move_gen::{MoveGenerator, naive::NaiveMoveGenerator},
    transition::copy_make::CopyMakeTransition,
};

fn perft<MG: MoveGenerator>(mut board: Board, mg: &MG, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut moves = MoveList::new();
    mg.generate_moves(&mut board, &mut moves);

    let mut total = 0u64;
    for mv in moves.iter() {
        let mut new_board = board.clone();
        new_board.apply(*mv);
        total += perft(new_board, mg, depth - 1);
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn perft_depth_1() {
        let board = Board::starting_position();
        let tm = CopyMakeTransition;
        let mg = NaiveMoveGenerator::new(tm);

        let result = perft(board, &mg, 1);
        assert_eq!(result, 20u64);
    }

    #[test]
    fn perft_depth_2() {
        let board = Board::starting_position();
        let tm = CopyMakeTransition;
        let mg = NaiveMoveGenerator::new(tm);

        let result = perft(board, &mg, 2);
        assert_eq!(result, 400u64);
    }

    #[test]
    fn perft_depth_3() {
        let board = Board::starting_position();
        let tm = CopyMakeTransition;
        let mg = NaiveMoveGenerator::new(tm);

        let result = perft(board, &mg, 3);
        assert_eq!(result, 8902u64);
    }

    #[test]
    fn perft_depth_4() {
        let board = Board::starting_position();
        let tm = CopyMakeTransition;
        let mg = NaiveMoveGenerator::new(tm);

        let result = perft(board, &mg, 4);
        assert_eq!(result, 197281u64);
    }

    #[test]
    fn perft_depth_5() {
        let board = Board::starting_position();
        let tm = CopyMakeTransition;
        let mg = NaiveMoveGenerator::new(tm);

        let result = perft(board, &mg, 5);
        assert_eq!(result, 4865609u64);
    }

    #[test]
    fn perft_depth_6() {
        let board = Board::starting_position();
        let tm = CopyMakeTransition;
        let mg = NaiveMoveGenerator::new(tm);

        let result = perft(board, &mg, 6);
        assert_eq!(result, 119060324u64);
    }
}
