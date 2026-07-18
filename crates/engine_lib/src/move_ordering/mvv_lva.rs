use crate::{
    Board, Move, MoveKind, MoveList,
    move_ordering::{OrderingContext, OrderingPolicy},
};

pub const MVV_LVA_TABLE: [[i16; 6]; 6] = [
    // Hundreds column = victim
    // Tens collumn = attacker
    // King is not a legal victim, but we'll put it in anyway :)
    //    P,   N,   B,   R,   Q,   K
    [105, 205, 305, 405, 505, 605], // P
    [104, 204, 304, 404, 504, 604], // N
    [103, 203, 303, 403, 503, 603], // B
    [102, 202, 302, 402, 502, 602], // R
    [101, 201, 301, 401, 501, 601], // Q
    [100, 200, 300, 400, 500, 600], // K
];
#[derive(Clone, Copy)]
pub struct MvvLva;

impl OrderingPolicy for MvvLva {
    fn order_moves(&mut self, board: &mut Board, moves: &mut MoveList, _context: OrderingContext) {
        moves.sort_by_key(|mv| self.score_move(board, *mv));
    }
}

impl MvvLva {
    fn score_move(&self, board: &Board, mv: Move) -> i16 {
        match mv.kind() {
            MoveKind::Capture => {
                let victim = board.get_piece(mv.destination()).unwrap().kind;
                let attacker = board.get_piece(mv.origin()).unwrap().kind;
                MVV_LVA_TABLE[victim as usize][attacker as usize]
            }
            MoveKind::EnPassantCapture => 105,
            MoveKind::Promotion(kind) => 900 + kind as i16,
            MoveKind::PromotionCapture(kind) => {
                let victim = board.get_piece(mv.destination()).unwrap().kind;
                let attacker = board.get_piece(mv.origin()).unwrap().kind;
                MVV_LVA_TABLE[victim as usize][attacker as usize] + 900 + kind as i16
            }
            _ => 0,
        }
    }
}
