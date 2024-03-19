
use std::{collections::HashMap, io::{stdout, Write}, str::FromStr};

use cadabra::*;
use chess::*;

fn main() {
    validate_move_gen();
}

fn debug_perft(pos: &Position, depth: u8) -> Result<HashMap<String, u64>, (String, Position)> {
    let moves = pos.generate_moves();

    let mut result: HashMap<String, u64> = HashMap::new();

    for m in moves {
        let mut copy = *pos;
        copy.make_move(m);
        if copy.zobrist_hash != Position::from_fen(&copy.fen_string()).unwrap().zobrist_hash {
            return Err((format!("Wrong zobrist after move {m}"), *pos))
        }
        let sub_nodes = if depth >2 {
            copy.perft::<false>(depth - 1)
        } else if depth > 1 {
            debug_perft(&copy, depth - 1)?.iter().map(|m| m.1).sum()
        } else {
            1
        };
        result.insert(format!("{m}"), sub_nodes);
    }

    Ok(result)
}

fn ref_debug_perft(pos: chess::Board, depth: u8) -> HashMap<String, u64> {
    let moves = MoveGen::new_legal(&pos);

    let mut result: HashMap<String, u64> = HashMap::new();

    for m in moves {
        let copy = pos.make_move_new(m);
        let sub_nodes = if depth > 2 {
            MoveGen::movegen_perft_test(&copy, depth as usize - 1) as u64
        } else if depth > 1 {
            ref_debug_perft(copy, depth - 1).iter().map(|m| m.1).sum()
        } else {
            1
        };
        result.insert(m.to_string(), sub_nodes);
    }

    result
}

fn validate_move_gen() {
    let positions = TEST_POSITIONS.iter();

    for (name, fen, depth) in positions {
        print!(" {name} at depth {depth} ... ");
        stdout().flush().unwrap();

        for depth in 1..=*depth-1 {
            if let Err((err, pos)) = validate_position(fen.to_string(), name, depth, false) {
                println!("Error at {name}:\n{err}\n");
                println!("{}", pos);
                
                panic!("Validation failed");
            }
        }

        println!("\tok")
    }

    println!("Validated all test positions")
}

fn validate_position(fen: String, name: &str, depth: u8, tracing: bool) -> Result<(), (String, Position)> {
    let mut pos = Position::from_fen(fen.as_str()).unwrap();
    let own_res = debug_perft(&pos, depth)?;

    let ref_res = ref_debug_perft(Board::from_str(&fen).unwrap(), depth);

    if depth == 1 {
        let missed_moves = ref_res.iter().filter(|m| !own_res.contains_key(m.0)).map(|m| m.0).collect::<Vec<&String>>();
        if !missed_moves.is_empty() {
            return Err((format!("Missed {} legal: {missed_moves:?}", missed_moves.len()), pos))
        }

        let extra_moves = own_res.iter().filter(|m| !ref_res.contains_key(m.0)).map(|m| m.0).collect::<Vec<&String>>();
        if !extra_moves.is_empty() {
            return Err((format!("Found {} too many: {extra_moves:?}", extra_moves.len()), pos))
        }

        if tracing {
            return Err(("Could not determine error. Probably an error in the move generator".to_string(), pos));
        }
    } else {
        for (key, ref_nodes) in ref_res {
            let own_nodes = *own_res.get(&key).unwrap();
            if ref_nodes != own_nodes {
                pos.make_uci_move(&key).unwrap();
                println!("Wrong move count on {name} at depth {depth}! Expected {ref_nodes}, got {own_nodes} Tracing with {key}");
                return validate_position(pos.fen_string(), name, depth - 1, true);
            };
        }
    }
    Ok(())
}

pub const TEST_POSITIONS: [(&str, &str, u8); 56] = [
    ("Startpos", "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 6),
    ("Kiwipete", "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -", 5),
    ("Position 3", "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -", 7),
    ("Position 4", "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1", 5),
    ("Position 5", "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8", 5),
    ("Position 6", "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10", 5),
    ("Draft 1", "rnbqkbnr/1ppppppp/p7/8/8/N7/PPPPPPPP/R1BQKBNR w KQkq - 0 2", 6),
    ("Draft 2", "rnbqkbnr/1ppppppp/p7/8/8/7N/PPPPPPPP/RNBQKB1R w KQkq - 0 2", 6),
    ("Draft 3", "rnbqkbnr/1ppppppp/p7/8/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 0 2", 6),
    ("Draft 4", "rnbqkbnr/1ppppppp/p7/8/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 2", 6),
    ("Draft 5", "rnbqkbnr/1ppppppp/8/p7/8/N7/PPPPPPPP/R1BQKBNR w KQkq - 0 2", 6),
    ("Draft 6", "rnbqkbnr/1ppppppp/8/p7/8/7N/PPPPPPPP/RNBQKB1R w KQkq - 0 2", 6),
    ("Draft 7", "rnbqkbnr/1ppppppp/8/p7/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 0 2", 6),
    ("Draft 8", "rnbqkbnr/1ppppppp/8/p7/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 2", 6),
    ("Draft 9", "rnbqkbnr/p1pppppp/1p6/8/8/N7/PPPPPPPP/R1BQKBNR w KQkq - 0 2", 6),
    ("Draft 10", "rnbqkbnr/p1pppppp/1p6/8/8/7N/PPPPPPPP/RNBQKB1R w KQkq - 0 2", 6),
    ("Draft 11", "rnbqkbnr/p1pppppp/1p6/8/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 2", 6),
    ("Draft 12", "rnbqkbnr/p1pppppp/1p6/8/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 0 2", 6),
    ("Draft 13", "rnbqkbnr/p1pppppp/8/1p6/8/N7/PPPPPPPP/R1BQKBNR w KQkq - 0 2", 6),
    ("Draft 14", "rnbqkbnr/p1pppppp/8/1p6/8/7N/PPPPPPPP/RNBQKB1R w KQkq - 0 2", 6),
    ("Draft 15", "rnbqkbnr/p1pppppp/8/1p6/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 0 2", 6),
    ("Draft 16", "rnbqkbnr/p1pppppp/8/1p6/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 2", 6),
    ("Draft 17", "rnbqkbnr/pp1ppppp/2p5/8/8/N7/PPPPPPPP/R1BQKBNR w KQkq - 0 2", 6),
    ("Draft 18", "rnbqkbnr/pp1ppppp/2p5/8/8/7N/PPPPPPPP/RNBQKB1R w KQkq - 0 2", 6),
    ("Draft 19", "rnbqkbnr/pp1ppppp/2p5/8/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 2", 6),
    ("Draft 20", "rnbqkbnr/pp1ppppp/2p5/8/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 0 2", 6),
    ("Draft 21", "rnbqkbnr/pp1ppppp/8/2p5/8/N7/PPPPPPPP/R1BQKBNR w KQkq - 0 2", 6),
    ("Draft 22", "rnbqkbnr/pp1ppppp/8/2p5/8/7N/PPPPPPPP/RNBQKB1R w KQkq - 0 2", 6),
    ("Draft 23", "rnbqkbnr/pp1ppppp/8/2p5/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 2", 6),
    ("Draft 24", "rnbqkbnr/pp1ppppp/8/2p5/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 0 2", 6),
    ("Draft 25", "rnbqkbnr/ppp1pppp/3p4/8/8/N7/PPPPPPPP/R1BQKBNR w KQkq - 0 2", 6),
    ("Draft 26", "rnbqkbnr/ppp1pppp/3p4/8/8/7N/PPPPPPPP/RNBQKB1R w KQkq - 0 2", 6),
    ("Draft 27", "rnbqkbnr/ppp1pppp/3p4/8/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 2", 6),
    ("Draft 28", "rnbqkbnr/ppp1pppp/3p4/8/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 0 2", 6),
    ("Draft 29", "rnbqkbnr/ppp1pppp/8/3p4/8/N7/PPPPPPPP/R1BQKBNR w KQkq - 0 2", 6),
    ("Draft 30", "rnbqkbnr/ppp1pppp/8/3p4/8/7N/PPPPPPPP/RNBQKB1R w KQkq - 0 2", 6),
    ("Draft 31", "rnbqkbnr/ppp1pppp/8/3p4/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 2", 6),
    ("Draft 32", "rnbqkbnr/ppp1pppp/8/3p4/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 0 2", 6),
    ("Draft 33", "rnbqkbnr/pppp1ppp/4p3/8/8/N7/PPPPPPPP/R1BQKBNR w KQkq - 0 2", 6),
    ("Draft 34", "rnbqkbnr/pppp1ppp/4p3/8/8/7N/PPPPPPPP/RNBQKB1R w KQkq - 0 2", 6),
    ("Draft 35", "rnbqkbnr/pppp1ppp/4p3/8/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 0 2", 6),
    ("Draft 36", "rnbqkbnr/pppp1ppp/4p3/8/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 2", 6),
    ("Draft 37", "rnbqkbnr/pppp1ppp/8/4p3/8/N7/PPPPPPPP/R1BQKBNR w KQkq - 0 2", 6),
    ("Draft 38", "rnbqkbnr/pppp1ppp/8/4p3/8/7N/PPPPPPPP/RNBQKB1R w KQkq - 0 2", 6),
    ("Draft 39", "rnbqkbnr/pppp1ppp/8/4p3/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 0 2", 6),
    ("Draft 40", "rnbqkbnr/ppppp1pp/5p2/8/8/N7/PPPPPPPP/R1BQKBNR w KQkq - 0 2", 6),
    ("Draft 41", "rnbqkbnr/pppp1ppp/8/4p3/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 2", 6),
    ("Draft 42", "rnbqkbnr/ppppp1pp/5p2/8/8/7N/PPPPPPPP/RNBQKB1R w KQkq - 0 2", 6),
    ("Draft 43", "rnbqkbnr/ppppp1pp/5p2/8/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 0 2", 6),
    ("Draft 44", "rnbqkbnr/ppppp1pp/5p2/8/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 2", 6),
    ("Draft 45", "rnbqkbnr/ppppp1pp/8/5p2/8/N7/PPPPPPPP/R1BQKBNR w KQkq - 0 2", 6),
    ("Draft 46", "rnbqkbnr/ppppp1pp/8/5p2/8/7N/PPPPPPPP/RNBQKB1R w KQkq - 0 2", 6),
    ("Draft 47", "rnbqkbnr/ppppp1pp/8/5p2/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 0 2", 6),
    ("Draft 48", "rnbqkbnr/ppppp1pp/8/5p2/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 2", 6),
    ("Draft 49", "rnbqkbnr/pppppp1p/6p1/8/8/N7/PPPPPPPP/R1BQKBNR w KQkq - 0 2", 6),
    ("Draft 50", "rnbqkbnr/pppppp1p/6p1/8/8/7N/PPPPPPPP/RNBQKB1R w KQkq - 0 2", 6),
];