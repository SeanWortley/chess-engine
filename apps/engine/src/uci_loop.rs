use engine_lib::prelude::*;
use std::io::{self, BufRead, Write};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;
use uci::{UciCommand, parse_command, types::UciMove};

const ID_NAME: &str = "PankBot";
const ID_AUTHOR: &str = "Pank";

use engine_lib::search::SearchReporter;

struct UciReporter;

impl SearchReporter for UciReporter {
    fn report_depth(
        &self,
        depth: u8,
        nodes: u64,
        time_ms: u128,
        score: i16,
        best_move: Option<Move>,
    ) {
        // Build the move portion
        let move_str = match best_move {
            Some(mv) => format!(
                " currmove {}{}",
                mv.origin().to_name(),
                mv.destination().to_name()
            ),
            None => String::new(),
        };

        // Send it :)
        send(&format!(
            "info depth {} nodes {} time {} score cp {}{}",
            depth, nodes, time_ms, score, move_str
        ));
    }
}

enum WorkerCommand {
    Search {
        board: Board,
        game_history: Vec<u64>,
        constraint: SearchConstraint,
        control: SearchControl,
    },
    Perft {
        board: Board,
        depth: u8,
    },
    Quit,
}

use engine_lib::move_ordering::OrderingPolicy;

fn worker_loop<S, MG, E, OP>(mut engine: Engine<S, MG, E, OP>, command_rx: Receiver<WorkerCommand>)
where
    S: Searcher,
    MG: MoveGenerator,
    E: Evaluator,
    OP: OrderingPolicy,
{
    while let Ok(command) = command_rx.recv() {
        match command {
            WorkerCommand::Search {
                mut board,
                game_history,
                constraint,
                control,
            } => {
                let reporter = UciReporter;
                let result =
                    engine.search(&mut board, &game_history, constraint, control, &reporter);
                send_bestmove(result);
            }
            WorkerCommand::Perft { mut board, depth } => {
                let nodes = perft(&mut engine, &mut board, depth);
                send(&format!("perft {}", nodes));
            }
            WorkerCommand::Quit => {
                return;
            }
        }
    }
}

pub fn run<S, MG, E, OP>(engine: Engine<S, MG, E, OP>)
where
    S: Searcher + Send + 'static,
    MG: MoveGenerator + Send + 'static,
    E: Evaluator + Send + 'static,
    OP: OrderingPolicy + Send + 'static,
{
    let mut board = Board::starting_position();
    // Hash of every position this game has visited, oldest first,
    // always ending with the current position.
    let mut game_hashes: Vec<u64> = vec![board.hash()];
    let mut control_handle: Option<SearchControl> = None;

    let (command_tx, command_rx): (Sender<WorkerCommand>, Receiver<WorkerCommand>) =
        mpsc::channel();
    let worker = thread::spawn(move || worker_loop(engine, command_rx));

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line = line.trim();

        match parse_command(line) {
            UciCommand::Uci => {
                send(&uci::id_name(ID_NAME));
                send(&uci::id_author(ID_AUTHOR));
                send(&uci::uciok());
            }
            UciCommand::IsReady => {
                send(&uci::readyok());
            }
            UciCommand::UciNewGame => {
                if let Some(control) = &control_handle {
                    control.stop();
                }
                board = Board::starting_position();
                game_hashes = vec![board.hash()];
            }
            UciCommand::Position { fen, moves } => {
                board = match fen {
                    Some(fen) => Board::from_fen(&fen),
                    None => Board::starting_position(),
                };
                game_hashes.clear();
                game_hashes.push(board.hash());

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
                            if i8::abs(destination.rank() as i8 - origin.rank() as i8) == 2 {
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
                    game_hashes.push(board.hash());
                }
            }
            UciCommand::Go {
                depth,
                movetime,
                infinite,
            } => {
                let constraint = match (depth, movetime, infinite) {
                    (Some(depth), _, _) => SearchConstraint::fixed_depth(depth),
                    (None, Some(movetime), _) => {
                        SearchConstraint::movetime(Duration::from_millis(movetime))
                    }
                    // For go infinite, search deep and rely on stop/time checks in the search.
                    (None, None, true) => SearchConstraint::fixed_depth(u8::MAX),
                    (None, None, false) => SearchConstraint::fixed_depth(3),
                };

                if let Some(control) = &control_handle {
                    control.stop();
                }

                let control = SearchControl::new(constraint);
                control_handle = Some(control.clone());

                command_tx
                    .send(WorkerCommand::Search {
                        board: board.clone(),
                        game_history: game_hashes.clone(),
                        constraint,
                        control,
                    })
                    .expect("worker thread is unavailable");
            }
            UciCommand::GoPerft { depth } => {
                command_tx
                    .send(WorkerCommand::Perft {
                        board: board.clone(),
                        depth,
                    })
                    .expect("worker thread is unavailable");
            }
            UciCommand::Stop => {
                if let Some(control) = &control_handle {
                    control.stop();
                }
            }
            UciCommand::Quit => {
                if let Some(control) = &control_handle {
                    control.stop();
                }
                let _ = command_tx.send(WorkerCommand::Quit);
                let _ = worker.join();
                return;
            }
            UciCommand::Unkown(command) => {
                eprintln!("Unkown Command: {}", command);
            }
        }
    }

    if let Some(control) = &control_handle {
        control.stop();
    }
    let _ = command_tx.send(WorkerCommand::Quit);
    let _ = worker.join();
}

fn send_bestmove(result: SearchResult) {
    match result.best_move {
        Some(mv) => {
            let promoted_to = match mv.kind() {
                MoveKind::Promotion(piece) | MoveKind::PromotionCapture(piece) => Some(
                    match piece {
                        PieceKind::Knight => "n",
                        PieceKind::Bishop => "b",
                        PieceKind::Rook => "r",
                        PieceKind::Queen => "q",
                        _ => panic!("invalid promotion piece"),
                    }
                    .to_string(),
                ),
                _ => None,
            };
            let uci_move = UciMove {
                origin: mv.origin().to_name(),
                destination: mv.destination().to_name(),
                promoted_to,
            };
            send(&uci::bestmove(uci_move));
        }
        None => {
            // UCI requires a bestmove response even in terminal positions.
            send("bestmove 0000");
        }
    }
}

fn perft<S, MG, E, OP>(engine: &mut Engine<S, MG, E, OP>, board: &mut Board, depth: u8) -> u64
where
    S: Searcher,
    MG: MoveGenerator,
    E: Evaluator,
    OP: OrderingPolicy,
{
    if depth == 0 {
        return 1;
    }

    let moves = engine.generate_moves(board);

    let mut total: u64 = 0;
    for mv in moves.iter() {
        engine.make(board, *mv);
        total += perft(engine, board, depth - 1);
        engine.unmake(board, *mv);
    }
    total
}

fn send(msg: &str) {
    println!("{}", msg);
    std::io::stdout().flush().unwrap();
}
