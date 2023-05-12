use std::{process::{Command, Stdio, Child}, thread, io::{BufReader, BufWriter, BufRead, Write, stdout}, collections::HashMap, sync::mpsc::{Receiver, Sender, channel}, env};

use cadabra::*;
mod test_positions;
use test_positions::*;

// Run with something like this, if 'stockfish.exe' is in executing directory
// "cargo validate" is alias for "cargo run --release --bin validator stockfish"
// Use "cargo validate short" for a reduced validation

fn main() {
    run_perft_tests()
}

fn debug_perft(pos: &Position, depth: u8, rep_table: &mut RepetitionTable) -> HashMap<String, u64> {
    let moves = pos.generate_moves();

    let mut result: HashMap<String, u64> = HashMap::new();

    for m in moves {
        let mut copy = *pos;
        copy.make_move(m, rep_table);
        let sub_nodes = if depth >2 {
            copy.perft::<false>(depth - 1)
        } else if depth > 1 {
            debug_perft(&copy, depth - 1, rep_table).iter().map(|m| m.1).sum()
        } else {
            1
        };
        result.insert(format!("{m}"), sub_nodes);
    }

    result
}

fn run_perft_tests() {
    let args: Vec<String> = env::args().collect();

    let (mut send_task, mut recv_result, handle) = {
        let (send_task, recv_task) = channel();
        let (send_result, recv_result) = channel();
        let args = args.clone();

        let handle = thread::spawn(move || {
            let ref_engine = Command::new(args[1].as_str())
                                    .stdin(Stdio::piped())
                                    .stdout(Stdio::piped())
                                    .spawn()
                                    .expect(format!("Could not launch reference engine \"{}\". Check path.", args[1]).as_str());
                                
            ref_engine_loop(ref_engine, (send_result, recv_task))
        });

        (send_task, recv_result, handle)
    };
    
    let short = args.contains(&"short".to_string());

    let positions = TEST_POSITIONS.iter().take(if short {24} else {TEST_POSITIONS.len()});

    for (name, fen, mut depth) in positions {
        if short {
            depth -= 1;
        }

        print!(" {name} at depth {depth} ... ");
        stdout().flush().unwrap();

        for depth in 1..=depth {
            if let Err((err, pos)) = validate_position(fen.to_string(), name, depth, false, (&mut send_task, &mut recv_result)) {
                println!("Error at {name}:\n{err}\n");
                println!("{}", pos);
                
                assert!(false)
            }
        }

        println!("\tok")
    }

    send_task.send(("close".to_string(), 0)).unwrap();

    handle.join().unwrap();

    println!("Validated all test positions")
}

fn ref_engine_loop(mut ref_engine: Child, (send_result, recv_task): (Sender<HashMap<String, u64>>, Receiver<(String, u8)>)) {
    let ref_in = ref_engine.stdin.take().unwrap();
    let ref_out = ref_engine.stdout.take().unwrap();

    let mut reader = BufReader::new(ref_out);
    let mut writer = BufWriter::new(ref_in);

    loop {
        let buffer = reader.fill_buf().unwrap();
        let length = buffer.len();
        reader.consume(length);
        
        let (fen, depth) = match recv_task.recv() {
            Ok((s, _)) if s == "close" => break,
            Ok(rec) => rec,
            Err(_) => break,
        };

        writeln!(writer, "position fen {}", fen).unwrap();
        writeln!(writer, "go perft {}", depth).unwrap();
        writer.flush().unwrap();

        let mut results = HashMap::new();
        loop {
            let mut buf = String::new();

            reader.read_line(&mut buf).unwrap();

            let split: Vec<&str> = buf.split(':').collect();
            if split.len() != 2 {
                break;
            }
            results.insert(split[0].trim().to_string(), split[1].trim().parse::<u64>().unwrap());
        }
        send_result.send(results).unwrap();
    }

    ref_engine.kill().unwrap();
}

fn validate_position(fen: String, name: &str, depth: u8, tracing: bool, (send_task, recv_result): (&mut Sender<(String, u8)>, &mut Receiver<HashMap<String, u64>>)) -> Result<(), (String, Position)> {
    // Reference engine io
    send_task.send((fen.clone(), depth)).unwrap();

    let mut pos = Position::from_fen(fen.as_str()).unwrap();
    let own_res = debug_perft(&pos, depth, &mut RepetitionTable::new());

    let ref_res = recv_result.recv().unwrap();

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
            return Err(("This is weird, Probably an error in the move generator".to_string(), pos));
        }
    } else {
        for (key, nodes) in ref_res {
            if nodes != *own_res.get(&key).unwrap() {
                pos.make_uci_move(&key, &mut RepetitionTable::new()).unwrap();
                println!("Wrong move count on {name} at depth {depth}! Tracing with {key}");
                return validate_position(pos.fen_string(), name, depth - 1, true, (send_task, recv_result));
            };
        }
    }
    Ok(())
}