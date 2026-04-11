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
                    let (fen, rest) = if remainder.starts_with("startpos") {
                        (
                            None,
                            remainder.strip_prefix("startpos").unwrap_or("").trim(),
                        )
                    } else {
                        let rest = remainder.strip_prefix("fen ").unwrap_or("");
                        match rest.find(" moves ") {
                            Some(idx) => (Some(rest[..idx].to_string()), &rest[idx + 7..]),
                            None => (Some(rest.to_string()), ""),
                        }
                    };

                    let moves = match rest.strip_prefix("moves ") {
                        Some(moves_str) => {
                            moves_str.split_whitespace().map(parse_uci_move).collect()
                        }
                        None => vec![],
                    };

                    return UciCommand::Position { fen, moves };
                }
                "go" => {
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
