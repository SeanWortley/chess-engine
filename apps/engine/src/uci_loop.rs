use engine_lib::prelude::*;
use std::io::{self, BufRead};
use uci::{UciCommand, parse_command, types::UciMove};

const ID_NAME: &str = "PankBot";
const ID_AUTHOR: &str = "Pank";

pub fn run<S, MG, E>(mut engine: Engine<S, MG, E>)
where
    S: Searcher,
    MG: MoveGenerator,
    E: Evaluator,
{
    let mut board = Board::starting_position();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line = line.trim();

        match parse_command(line) {
            UciCommand::Uci => {
                println!("{}", uci::id_name(ID_NAME));
                println!("{}", uci::id_author(ID_AUTHOR));
                println!("{}", uci::uciok());
            }
            UciCommand::IsReady => {
                println!("{}", uci::readyok());
            }
            UciCommand::UciNewGame => {
                board = Board::starting_position();
            }
            UciCommand::Position { fen, moves } => {
                board = match fen {
                    Some(fen) => Board::from_fen(&fen),
                    None => Board::starting_position(),
                };

                for uci_move in moves {
                    let origin = Square::from_name(uci_move.origin.as_str());
                    let destination = Square::from_name(uci_move.destination.as_str());

                    let moving_piece = board
                        .get_piece(origin)
                        .expect("UCI Expected piece that isn't there.");

                    let mv: Move;
                    match moving_piece.kind {
                        PieceKind::Pawn => {
                            let is_capture = origin.file() != destination.file();
                            // Check for double push
                            if i8::abs(destination.rank() as i8 - origin.rank() as i8) != 1 {
                                mv = Move::double_pawn_push(origin, destination);
                            } else {
                                match uci_move.promoted_to {
                                    Some(piece_string) => {
                                        let promoted_to = match piece_string.as_str() {
                                            "n" => PieceKind::Knight,
                                            "b" => PieceKind::Bishop,
                                            "r" => PieceKind::Rook,
                                            "q" => PieceKind::Queen,
                                            _ => panic!("Not sure what to do here"),
                                        };
                                        if is_capture {
                                            mv = Move::promotion_capture(
                                                origin,
                                                destination,
                                                promoted_to,
                                            )
                                        } else {
                                            mv = Move::promotion(origin, destination, promoted_to)
                                        }
                                    }
                                    None => {
                                        if is_capture {
                                            if board.is_en_passant(destination) {
                                                mv = Move::en_passant(origin, destination);
                                            } else {
                                                mv = Move::capture(origin, destination);
                                            }
                                        } else {
                                            mv = Move::quiet(origin, destination);
                                        }
                                    }
                                }
                            }
                        }
                        PieceKind::Knight => {
                            if board.get_piece(destination).is_some() {
                                mv = Move::capture(origin, destination);
                            } else {
                                mv = Move::quiet(origin, destination);
                            }
                        }
                        PieceKind::Bishop => {
                            if board.get_piece(destination).is_some() {
                                mv = Move::capture(origin, destination);
                            } else {
                                mv = Move::quiet(origin, destination);
                            }
                        }
                        PieceKind::Rook => {
                            if board.get_piece(destination).is_some() {
                                mv = Move::capture(origin, destination);
                            } else {
                                mv = Move::quiet(origin, destination);
                            }
                        }
                        PieceKind::Queen => {
                            if board.get_piece(destination).is_some() {
                                mv = Move::capture(origin, destination);
                            } else {
                                mv = Move::quiet(origin, destination);
                            }
                        }
                        PieceKind::King => {
                            if board.get_piece(destination).is_some() {
                                mv = Move::capture(origin, destination);
                            } else {
                                // Check for castling
                                if origin.file() == 4 && destination.file() == 6 {
                                    mv = Move::king_castle(origin, destination);
                                } else if origin.file() == 4 && destination.file() == 2 {
                                    mv = Move::queen_castle(origin, destination);
                                } else {
                                    mv = Move::quiet(origin, destination);
                                }
                            }
                        }
                    }
                    board.apply(mv);
                }
            }
            UciCommand::Go { depth, movetime } => {
                let result = engine.search(&mut board);
                match result.best_move {
                    Some(mv) => {
                        let promoted_to = match mv.kind() {
                            Movekind::Promotion(PieceKind::Knight) => PieceKind::Knight,
                            _ => None
                        }
                        let uci_move = UciMove {
                            origin,
                            destination,

                        }
                        println("{}")
                    }
                }
            }
            UciCommand::Quit => {
                return;
            }
            UciCommand::Unkown(command) => {
                eprintln!("Unkown Command: {}", command);
            }
        }
    }
}
