use crate::UciMove;

pub enum UciCommand {
    Uci,
    IsReady,
    UciNewGame,
    Position {
        fen: Option<String>,
        moves: Vec<UciMove>,
    },
    Go {
        depth: Option<u8>,
        movetime: Option<u64>,
    },
    GoPerft {
        depth: u8,
    },
    Quit,
    Unkown(String),
}

pub fn parse_command(input: &str) -> UciCommand {
    match input.find(" ") {
        Some(idx) => {
            let command = &input[0..idx];
            let remainder = &input[idx + 1..];
            match command {
                "position" => {
                    let (fen, moves) = if remainder.starts_with("startpos") {
                        let rest = remainder.strip_prefix("startpos").unwrap_or("").trim();
                        let moves = match rest.strip_prefix("moves ") {
                            Some(moves_str) => {
                                moves_str.split_whitespace().map(parse_uci_move).collect()
                            }
                            None => vec![],
                        };
                        (None, moves)
                    } else {
                        let rest = remainder.strip_prefix("fen ").unwrap_or("").trim();
                        match rest.split_once(" moves ") {
                            Some((fen_str, moves_str)) => (
                                Some(fen_str.trim().to_string()),
                                moves_str.split_whitespace().map(parse_uci_move).collect(),
                            ),
                            None => (Some(rest.to_string()), vec![]),
                        }
                    };

                    return UciCommand::Position { fen, moves };
                }
                "go" => {
                    let rest = input.trim();
                    if let Some(perft_part) = rest.strip_prefix("go perft ") {
                        return UciCommand::GoPerft {
                            depth: perft_part.trim().parse().expect("Not u8 compatable"),
                        };
                    }

                    let (_rest, depth) = input.rsplit_once(" ").unwrap();
                    return UciCommand::Go {
                        depth: Some(depth.parse().expect("Not u8 compatable")),
                        movetime: None,
                    };
                }
                _ => return UciCommand::Unkown(String::from(input)),
            }
        }
        None => match input {
            "uci" => return UciCommand::Uci,
            "isready" => return UciCommand::IsReady,
            "ucinewgame" => return UciCommand::UciNewGame,
            "quit" => return UciCommand::Quit,
            _ => return UciCommand::Unkown(String::from(input)),
        },
    }
}

fn parse_uci_move(mv: &str) -> UciMove {
    UciMove {
        origin: mv[0..2].to_string(),
        destination: mv[2..4].to_string(),
        promoted_to: if mv.len() == 5 {
            Some(mv[4..5].to_string())
        } else {
            None
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{UciCommand, parse_command};

    #[test]
    fn parse_position_startpos_with_moves() {
        let cmd = parse_command("position startpos moves e2e4 e7e5");
        match cmd {
            UciCommand::Position { fen, moves } => {
                assert!(fen.is_none());
                assert_eq!(moves.len(), 2);
                assert_eq!(moves[0].origin, "e2");
                assert_eq!(moves[0].destination, "e4");
                assert_eq!(moves[1].origin, "e7");
                assert_eq!(moves[1].destination, "e5");
            }
            _ => panic!("Expected position command"),
        }
    }

    #[test]
    fn parse_position_fen_with_moves() {
        let cmd = parse_command(
            "position fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 moves e2e4 e7e5",
        );
        match cmd {
            UciCommand::Position { fen, moves } => {
                assert_eq!(
                    fen.as_deref(),
                    Some("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
                );
                assert_eq!(moves.len(), 2);
                assert_eq!(moves[0].origin, "e2");
                assert_eq!(moves[0].destination, "e4");
                assert_eq!(moves[1].origin, "e7");
                assert_eq!(moves[1].destination, "e5");
            }
            _ => panic!("Expected position command"),
        }
    }
}
