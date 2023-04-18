use std::{time::Instant, io::{stdout, Write, self, BufRead}, fs::File, path::PathBuf};
use colored::{Colorize, ColoredString};
use std::hint::black_box;
use lazy_static::lazy_static;

use crate::engine::*;

lazy_static!(
    pub static ref POSITIONS: Vec<Position> = vec![
        Position::start_pos(),
        Position::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -").unwrap(),    // Kiwipete
        Position::from_fen("r2q1rk1/5pp1/1ppp3p/2n1nbN1/4p1PP/P1P1P3/Q2PBP2/R1B2RK1 b - - 1 17").unwrap(),  // JENCE
        Position::from_fen("1r1q1rk1/5pp1/1NRpb2p/p3p3/8/P2P2P1/1P2PPBP/3Q1RK1 b - - 0 19").unwrap(),       // Kasparov v Georgiev
        Position::from_fen("r1bq1rk1/3n1pbp/2pQ2p1/4n3/1p2PP2/1P2B3/P3N1PP/1N1RKB1R b K - 1 15").unwrap()   // Karpov v Kasparov
    ];
);

const ITERATIONS: u16 = 1;

pub fn run_bench(save: bool) {
    // PERFT BENCH
    let perft_mnps = perft_bench();

    println!();

    // SEARCH TIME TO DEPTH (ttd)
    let search_ttd = search_bench();

    println!();

    // TODO
    // Total nodes
    // Nodes/Sec
    // Stats for multithreaded
    // TT fill% + hits


    // Load baseline
    if let Ok(file) = File::open(baseline_path()) {
        let lines = io::BufReader::new(file).lines().map(|l| l.unwrap()).collect::<Vec<String>>();

        println!("  Changes:");
        // Perft percentage
        {
            let prev = lines.iter().find(|l| l.starts_with("perft")).unwrap().split_once(':').unwrap().1.parse::<f64>().unwrap();
            let diff_perc = ((perft_mnps - prev) / prev) * 100.;
            let diff_str = color_string_percent(format!("{:.3} MNodes/s", perft_mnps - prev).normal(), diff_perc, true);
            let diff_perc_str = color_string_percent(format!("{:.3}%", diff_perc).normal(), diff_perc, true);
            println!(" Perft performance changed by {} ≈ {}", diff_str, diff_perc_str);
        }

        // Search TTD percentage
        {
            let prev = lines.iter().find(|l| l.starts_with("s_ttd")).unwrap().split_once(':').unwrap().1.parse::<u128>().unwrap();
            let diff_perc = ((search_ttd as f64 - prev as f64) / prev as f64) * 100.;
            let diff_str = color_string_percent(format!("{} ms", search_ttd as i128 - prev  as i128).normal(), diff_perc, false);
            let diff_perc_str = color_string_percent(format!("{:.3}%", diff_perc).normal(), diff_perc, false);
            println!(" Search time to depth changed by {} ≈ {}", diff_str, diff_perc_str);
        }

        println!()
    }

    // Save results as baseline if relevant
    if save {
        let mut file = File::create(baseline_path()).expect(" Could not create baseline file");
        writeln!(file, "perft:{}", perft_mnps).unwrap();
        writeln!(file, "s_ttd:{}", search_ttd).unwrap();
        println!("  Baseline saved");
    }
}

fn color_string_percent(str: ColoredString, percent: f64, increase_is_desired: bool) -> ColoredString {
    if percent < -0.5 {
        if increase_is_desired {
            str.red()
        } else {
            str.green()
        }
    }
    else if percent > 0.5 {
        if increase_is_desired {
            str.green()
        } else {
            str.red()
        }
    } else {
        str
    }
}

fn perft_bench() -> f64{
    println!(" Running perft benchmark...");
    print!(" Warming up...\t");

    let before_wu = Instant::now();
    
    stdout().flush().unwrap();
    for pos in POSITIONS.iter() {
        black_box(pos).perft::<false>(black_box(5));
    }
    println!("Done");
    println!(" Estimated bench time: {:.2}s", (before_wu.elapsed().as_millis() as f64 / 1000.) * ITERATIONS as f64);

    let mut nodes = 0;

    let before = Instant::now();

    for _ in 1..=ITERATIONS {
        for pos in POSITIONS.iter() {
            nodes += black_box(pos).perft::<false>(black_box(5));
        }
        stdout().flush().unwrap();
    }

    let perft_time = before.elapsed().as_millis();
    
    let perft_mnps = (nodes as f64 / perft_time as f64) / 1000.;

    println!(" Finished perft bench in {:.2}s", perft_time as f64 / 1000.);
    println!(" Speed was: {perft_mnps:.2} MNodes/s");

    perft_mnps
}

fn search_bench() -> u128 {
    println!(" Running search time to depth benchmark...");
    print!(" Warming up...\t");

    let before_wu = Instant::now();

    let search = Search::new(Settings::default());
    let meta = SearchMeta::new(8);
    
    stdout().flush().unwrap();
    for pos in POSITIONS.iter() {
        let s = search.clone();
        black_box(s).start(*pos, meta, false);
    }
    println!("Done");
    println!(" Estimated bench time: {:.2} s", (before_wu.elapsed().as_millis() as f64 / 1000.) * ITERATIONS as f64);

    let before = Instant::now();

    for pos in POSITIONS.iter() {
        let s = search.clone();
        black_box(s).start(*pos, meta, false);
    }

    let search_time = before.elapsed().as_millis();

    println!(" Finished search bench in {} ms", search_time);

    search_time
}

fn baseline_path() -> PathBuf {
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    path.join("bench_baseline.txt")
}