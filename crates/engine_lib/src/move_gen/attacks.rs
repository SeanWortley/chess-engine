use crate::core::board::{Board, Color, Piece, PieceKind};
use crate::core::direction::Direction;
use crate::core::direction::Direction::*;
use crate::core::square::Square;

pub type IsAttackedFn = fn(&Board, Square, Color) -> bool;

struct RayAttackContext<'a> {
    // Doesn't live longer than the values inside :)
    board: &'a Board,
    square: Square,
    attacking_color: Color,
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
        if take_ray_step(context, direction) == Some(context.attacking_color) {
            return true;
        }
    }
    return false;
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
        if take_ray_step(context, direction) == Some(context.attacking_color) {
            return true;
        }
    }
    return false;
}
#[inline]
fn take_ray_step(context: &RayAttackContext<'_>, direction: Direction) -> Option<Piece> {
    let board = context.board;

    let (x, y) = direction.offset();
    let file = context.square.file() as i8 + x;
    let rank = context.square.rank() as i8 + y;

    if !(0..8).contains(&file) || !(0..8).contains(&rank) {
        return None;
    }

    let destination = Square::new(file as u8, rank as u8);

    // Check for piece
    match board.get_piece(destination) {
        Some(piece) => {
            return Some(piece.color);
        }
        None => {
            return None;
        }
    }
}
