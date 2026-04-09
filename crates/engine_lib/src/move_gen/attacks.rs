use crate::Direction::*;
use crate::PieceKind::*;
use crate::{Board, Color, Direction, Piece, PieceKind, Square};

pub type IsAttackedFn = fn(&Board, Square, Color) -> bool;

struct RayAttackContext<'a> {
    // Doesn't live longer than the values inside :)
    board: &'a Board,
    square: Square,
    attacking_color: Color,
}

enum RayStepResult {
    OffBoard,
    Empty(Square),
    Piece(Piece),
}

// Same logic as ray based move gen :)
#[inline]
pub fn ray_is_attacked(board: &Board, square: Square, attacking_color: Color) -> bool {
    let context = RayAttackContext {
        board,
        square,
        attacking_color,
    };

    is_pawn_attack(&context)
        || is_knight_attack(&context)
        || is_bishop_attack(&context)
        || is_rook_attack(&context)
        || is_king_attack(&context)
}
#[inline]
fn is_pawn_attack(context: &RayAttackContext) -> bool {
    let color = context.attacking_color.opponent();
    let directions: [Direction; 2] = if color == Color::White {
        [NorthWest, NorthEast]
    } else {
        [SouthWest, SouthEast]
    };
    for direction in directions {
        if let RayStepResult::Piece(piece) = take_ray_step(context.board, context.square, direction)
        {
            if piece.kind == Pawn && piece.color == context.attacking_color {
                return true;
            }
        }
    }
    false
}

#[inline]
fn is_knight_attack(context: &RayAttackContext) -> bool {
    for direction in [
        NorthNorthEast,
        EastNorthEast,
        EastSouthEast,
        SouthSouthEast,
        SouthSouthWest,
        WestSouthWest,
        WestNorthWest,
        NorthNorthWest,
    ] {
        if let RayStepResult::Piece(piece) = take_ray_step(context.board, context.square, direction)
        {
            if piece.kind == PieceKind::Knight && piece.color == context.attacking_color {
                return true;
            }
        }
    }
    false
}

fn is_bishop_attack(context: &RayAttackContext) -> bool {
    for direction in [NorthEast, SouthEast, SouthWest, NorthWest] {
        let mut current = context.square;
        loop {
            match take_ray_step(context.board, current, direction) {
                RayStepResult::OffBoard => break,
                RayStepResult::Empty(next_square) => current = next_square,
                RayStepResult::Piece(piece) => {
                    if (piece.kind == Bishop || piece.kind == Queen)
                        && piece.color == context.attacking_color
                    {
                        return true;
                    }
                    break;
                }
            }
        }
    }
    false
}
fn is_rook_attack(context: &RayAttackContext) -> bool {
    for direction in [North, East, South, West] {
        let mut current = context.square;
        loop {
            match take_ray_step(context.board, current, direction) {
                RayStepResult::OffBoard => break,
                RayStepResult::Empty(next_square) => current = next_square,
                RayStepResult::Piece(piece) => {
                    if (piece.kind == Rook || piece.kind == Queen)
                        && piece.color == context.attacking_color
                    {
                        return true;
                    }
                    break;
                }
            }
        }
    }
    false
}
fn is_king_attack(context: &RayAttackContext) -> bool {
    for direction in [
        North, NorthEast, East, SouthEast, South, SouthWest, West, NorthWest,
    ] {
        if let RayStepResult::Piece(piece) = take_ray_step(context.board, context.square, direction)
        {
            if piece.kind == PieceKind::King && piece.color == context.attacking_color {
                return true;
            }
        }
    }
    false
}
fn take_ray_step(board: &Board, origin: Square, direction: Direction) -> RayStepResult {
    let (x, y) = direction.offset();
    let file = origin.file() as i8 + x;
    let rank = origin.rank() as i8 + y;

    if !(0..8).contains(&file) || !(0..8).contains(&rank) {
        return RayStepResult::OffBoard;
    }

    let destination = Square::new(file as u8, rank as u8);

    match board.get_piece(destination) {
        Some(piece) => RayStepResult::Piece(piece),
        None => RayStepResult::Empty(destination),
    }
}
