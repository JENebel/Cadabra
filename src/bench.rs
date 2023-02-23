use std::{time::Instant, io::{stdout, Write}};

use crate::Position;

pub const BENCH_POSITIONS: [(&'static str, &'static str, u8); 5] = [
	("Startpos",            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",           6),
    ("Kiwipete",            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -",   5),
    ("JENCE",               "r2q1rk1/5pp1/1ppp3p/2n1nbN1/4p1PP/P1P1P3/Q2PBP2/R1B2RK1 b - - 1 17", 5),
    ("Kasparov v Georgiev", "1r1q1rk1/5pp1/1NRpb2p/p3p3/8/P2P2P1/1P2PPBP/3Q1RK1 b - - 0 19",      5),
    ("Karpov v Kasparov",   "r1bq1rk1/3n1pbp/2pQ2p1/4n3/1p2PP2/1P2B3/P3N1PP/1N1RKB1R b K - 1 15", 5),
];

const ITERATIONS: u8 = 10;

pub fn run_bench() {
    let positions = BENCH_POSITIONS.iter().map(|(_, fen, depth)| (Position::from_fen(fen).unwrap(), *depth)).collect::<Vec<(Position, u8)>>();

    println!("\nWarming up...");
    let mut before = Instant::now();
    for (pos, depth) in &positions {
        pos.perft::<false>(*depth);
    }
    let elapsed = before.elapsed().as_millis();

    println!("Running perft bench. Done in approximately {:.2}s...", (elapsed as f64 / 1000.) * ITERATIONS as f64);

    let mut nodes = 0;

    before = Instant::now();

    for i in 1..=ITERATIONS {
        print!(" Iteration {i}/{ITERATIONS} ...\t");
        stdout().flush().unwrap();
        for (pos, depth) in &positions {
            nodes += pos.perft::<false>(*depth);
        }
        println!("Done");
        stdout().flush().unwrap();
    }

    let perft_time = before.elapsed().as_millis();
    println!("\nFinished perft bench in {:.2}s", perft_time as f64 / 1000.);
    let perft_mnps = (nodes as f64 / perft_time as f64) / 1000.;
    println!("Speed was: {perft_mnps:.2} MNodes/s\n")
}