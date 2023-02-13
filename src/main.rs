use std::{time::Instant, io::{self, Write, BufReader, BufRead}, fs::File};

use cadabra::*;
use Square::*;
use PieceType::*;
use MoveType::*;
//use Color::*;

fn main() {
    let lines = read_lines("./perft_results.txt");

    let mut positions = Vec::new();

    positions.push(("Startpos".to_string(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()));
    positions.push(("Kiwipete".to_string(), "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -".to_string()));
    positions.push(("Position 3".to_string(), "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -".to_string()));
    positions.push(("Position 4".to_string(), "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1".to_string()));
    positions.push(("Position 5".to_string(), "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8".to_string()));
    positions.push(("Position 6".to_string(), "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10".to_string()));

    let mut i = 0;
    for l in lines {
        i += 1;
        let line = l.unwrap();

        let fen = line.split(" ").take(6).map(|f| format!("{f} ")).collect::<String>().trim().to_string();

        positions.push((format!("Draft {i}"), fen))
    }

    let path = "./lines.txt";

    let mut output = File::create(path).unwrap();
    write!(output, "pub const TEST_POSITIONS: Vec<(&str, &str)> = vec![\n").unwrap();
    for pos in positions {
        write!(output, "\t(\"{}\", \"{}\"),\n", pos.0, pos.1).unwrap()
    }
    write!(output, "];").unwrap()
}

fn read_lines(filename: &str) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
    let file = File::open(filename).unwrap(); 
    // Read the file line by line, and return an iterator of the lines of the file.
    return io::BufReader::new(file).lines(); 
}