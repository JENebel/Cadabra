use std::{time::Instant, io::{stdout, Write, self, BufRead}, fs::File, path::PathBuf};
use colored::Colorize;

use crate::Position;

const  BENCH_POSITIONS: [(&'static str, &'static str, u8); 5] = [
	("Startpos",            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",           6),
    ("Kiwipete",            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -",   5),
    ("JENCE",               "r2q1rk1/5pp1/1ppp3p/2n1nbN1/4p1PP/P1P1P3/Q2PBP2/R1B2RK1 b - - 1 17", 5),
    ("Kasparov v Georgiev", "1r1q1rk1/5pp1/1NRpb2p/p3p3/8/P2P2P1/1P2PPBP/3Q1RK1 b - - 0 19",      5),
    ("Karpov v Kasparov",   "r1bq1rk1/3n1pbp/2pQ2p1/4n3/1p2PP2/1P2B3/P3N1PP/1N1RKB1R b K - 1 15", 5),
];

const WARMUPS: u16 = 0;
const ITERATIONS: u16 = 5;

pub fn run_bench(save: bool) {
    let positions = BENCH_POSITIONS.iter().map(|(_, fen, depth)| (Position::from_fen(fen).unwrap(), *depth)).collect::<Vec<(Position, u8)>>();

    if WARMUPS > 0 {
        println!("Warming up...");
    }
    
    let before_wu = Instant::now();
    for i in 1..=WARMUPS {
        print!(" Warm up ite. {i}/{WARMUPS} ...\t");
        stdout().flush().unwrap();
        for (pos, depth) in &positions {
            pos.perft::<false>(*depth);
        }
        println!("Done");
        stdout().flush().unwrap();
        if i == 1 {
            println!("  Estimated bench time: {:.2}s", (before_wu.elapsed().as_millis() as f64 / 1000.) * (WARMUPS + ITERATIONS) as f64);
        }
    }

    let mut nodes = 0;

    println!("Running perft benchmark...");

    let before = Instant::now();

    for i in 1..=ITERATIONS {
        print!(" Bench ite. {i}/{ITERATIONS} ...\t");
        stdout().flush().unwrap();
        for (pos, depth) in &positions {
            nodes += pos.perft::<false>(*depth);
        }
        println!("Done");
        stdout().flush().unwrap();
    }

    let perft_time = before.elapsed().as_millis();
    
    let perft_mnps = (nodes as f64 / perft_time as f64) / 1000.;

    show_results(before_wu.elapsed().as_millis(), perft_mnps);

    // Save results as baseline if relevant
    if !save {
        return
    }

    let mut file = File::create(baseline_path()).expect("Could not create baseline file");
    writeln!(file, "perft:{}", perft_mnps).expect("Could not write benchmark");
    println!("Baseline saved");
}

fn show_results(perft_time: u128, perft_mnps: f64) {
    println!("Finished perft bench in {:.2}s", perft_time as f64 / 1000.);
    println!("Speed was: {perft_mnps:.2} MNodes/s");

    // Load baseline
    // TODO handle errors here
    let file = match File::open(baseline_path()) {
        Ok(file) => file,
        Err(_) => return,
    };
    
    let lines = io::BufReader::new(file).lines().map(|l| l.unwrap()).collect::<Vec<String>>();
    let prev_perft_line = lines.iter().find(|l| l.starts_with("perft")).unwrap();
    let prev_perft_mnps = prev_perft_line.split_once(':').unwrap().1.parse::<f64>().unwrap();

    let perft_diff_pc = f64::abs(perft_mnps - prev_perft_mnps)/((perft_mnps + prev_perft_mnps) / 2.) * 100.;
    let perft_diff_pc_str = format!("{:.3}", perft_diff_pc);

    // Set color
    let result_str = if perft_diff_pc > 0.5 {
        if perft_mnps < prev_perft_mnps {
            format!("-{}", perft_diff_pc_str).red()
        } else {
            format!("+{}", perft_diff_pc_str).green()
        }
    } 
    else {
        if perft_mnps < prev_perft_mnps {
            format!("-{}", perft_diff_pc_str).normal()
        } else {
            format!("+{}", perft_diff_pc_str).normal()
        }
    };

    println!("MNodes/s changed by {}%", result_str);
}

fn baseline_path() -> PathBuf {
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    path.join("bench_baseline.txt")
}