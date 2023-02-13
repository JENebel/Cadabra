#![feature(buf_read_has_data_left)]

mod common;

use std::{process::{Command, Stdio, Child}, thread, io::{BufReader, BufRead, BufWriter, Read}};

use cadabra::Position;
use common::{test_positions::TEST_POSITIONS, load_config};
use std::io::Write;

#[test]
#[ignore]
fn run_perft_tests() {
    let config = load_config();
    let ref_engine_path = config.get("reference_engine_path").expect("please provide 'reference_engine_path' in cfg");
    
    let mut ref_engine = Command::new(ref_engine_path).stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().expect("Could not launch reference engine. Check path.");

    let mut bufread = BufReader::new(ref_engine.stdout.as_mut().unwrap());

    thread::sleep(mi)

    println!("{:?}", bufread.buffer().len());

    return;

    println!("{:?}", ref_engine);

    for test_pos in TEST_POSITIONS {
        let name = test_pos.0;
        let fen = test_pos.1;
        validate_position(name.to_string(), fen.to_string(), 5, &mut ref_engine, Vec::new())
    }
}

fn validate_position(name: String, fen: String, depth: u8, ref_engine: &mut Child, moves: Vec<&str>) {
    
    // Reference engine io
    let stdin = ref_engine.stdin.take().unwrap();
    let stdout = ref_engine.stdout.take().unwrap();

    let thread = thread::spawn(move || {
        
        let mut bufread = BufReader::new(stdout);
        let mut bufwrite = BufWriter::new(stdin);

        // Send position, and start perft
        bufwrite.write_all(format!("position fen {}", fen).as_bytes()).unwrap();
        bufwrite.flush().unwrap();
        bufwrite.write_all(format!("go perft {}", depth).as_bytes()).unwrap();
        bufwrite.flush().unwrap();
        let mut buf = String::new();

        let mut results = Vec::new();

        loop {
            println!("reading...");
            bufread.read_line(&mut buf).unwrap();
            println!("{buf}");
            let split: Vec<&str> = buf.split(':').collect();

            if split.len() != 2 {
                break;
            }

            // Valid move
            results.push((split[0].trim().to_string(), split[0].trim().parse::<u64>()));
        }
        //thread::sleep(std::time::Duration::from_millis(25));
        bufread.read_to_end(&mut Vec::new()).unwrap();

    });
    
    //let pos = Position::new_from_fen(fen);


    let results = thread.join().unwrap();

    println!("{results:?}")
}