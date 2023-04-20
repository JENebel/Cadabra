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
    println!();

    // PERFT BENCH
    let perft_mnps = perft_bench();

    println!();

    // SEARCH TIME TO DEPTH (ttd)
    let (search_time, nodes, search_mnps) = search_bench();

    println!();

    // TODO
    // Stats for multithreaded
    // TT fill% + hits


    // Load baseline
    if let Ok(file) = File::open(baseline_path()) {
        let lines = io::BufReader::new(file).lines().map(|l| l.unwrap()).collect::<Vec<String>>();

        println!("  Changes:");
        // Perft
        {
            let prev = lines.iter().find(|l| l.starts_with("perft")).unwrap().split_once(':').unwrap().1.parse::<f64>().unwrap();
            let diff_perc = ((perft_mnps - prev) / prev) * 100.;
            let diff_str = color_string_percent(format!("{:>+.3} MNodes/s", perft_mnps - prev).normal(), diff_perc, true);
            let diff_perc_str = color_string_percent(format!("{:>+.3}%", diff_perc).normal(), diff_perc, true);
            println!(" Perft performance changed by {} ≈ {}", diff_str, diff_perc_str);
        }

        // Search node count
        {
            let prev = lines.iter().find(|l| l.starts_with("nodes")).unwrap().split_once(':').unwrap().1.parse::<u128>().unwrap();
            let diff_perc = ((nodes as f64 - prev as f64) / prev as f64) * 100.;
            let diff_str = color_string_percent(format!("{:>+} nodes", nodes as i128 - prev  as i128).normal(), diff_perc, false);
            let diff_perc_str = color_string_percent(format!("{:>+.3}%", diff_perc).normal(), diff_perc, false);
            println!(" Search node count changed by {} ≈ {}", diff_str, diff_perc_str);
        }

        // Search TTD
        {
            let prev = lines.iter().find(|l| l.starts_with("s_ttd")).unwrap().split_once(':').unwrap().1.parse::<u128>().unwrap();
            let diff_perc = ((search_time as f64 - prev as f64) / prev as f64) * 100.;
            let diff_str = color_string_percent(format!("{:>+} ms", search_time as i128 - prev  as i128).normal(), diff_perc, false);
            let diff_perc_str = color_string_percent(format!("{:>+.3}%", diff_perc).normal(), diff_perc, false);
            println!(" Search time to depth changed by {} ≈ {}", diff_str, diff_perc_str);
        }

        // Search MNodes/s
        {
            let prev = lines.iter().find(|l| l.starts_with("search_mnps")).unwrap().split_once(':').unwrap().1.parse::<f64>().unwrap();
            let diff_perc = ((search_mnps - prev) / prev) * 100.;
            let diff_str = color_string_percent(format!("{:>+.4} MNodes/s", search_mnps - prev).normal(), diff_perc, true);
            let diff_perc_str = color_string_percent(format!("{:>+.3}%", diff_perc).normal(), diff_perc, true);
            println!(" Search MNodes/s changed by {} ≈ {}", diff_str, diff_perc_str);
        }
    }

    // Save results as baseline if relevant
    if save {
        let mut file = File::create(baseline_path()).expect(" Could not create baseline file");
        writeln!(file, "perft:{}", perft_mnps).unwrap();
        writeln!(file, "s_ttd:{}", search_time).unwrap();
        writeln!(file, "nodes:{}", nodes).unwrap();
        writeln!(file, "search_mnps:{}", search_mnps).unwrap();
        println!("Baseline saved");
    }

    println!()
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

fn mega_nodes_pr_sec(nodes: u128, millis: u128) -> f64 {
    (nodes as f64 / millis as f64) / 1000.
}

fn perft_bench() -> f64 {
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
    
    let perft_mnps = mega_nodes_pr_sec(nodes as u128, perft_time);

    println!(" Finished perft bench in {:.2}s", perft_time as f64 / 1000.);
    println!("  Speed was {perft_mnps:.2} MNodes/s");

    perft_mnps
}
                //(millis, nodes)
fn search_bench() -> (u128, u128, f64) {
    println!(" Running search time to depth benchmark...");
    print!(" Warming up...\t");

    let before_wu = Instant::now();

    let search = Search::new(Settings::default());

    let depth = 4;
    let meta = SearchMeta::new(depth);
    
    stdout().flush().unwrap();
    for pos in POSITIONS.iter() {
        let s = search.clone();
        black_box(s).start(*pos, meta, false);
    }
    println!("Done");
    println!(" Estimated bench time: {:.2} s", (before_wu.elapsed().as_millis() as f64 / 1000.) * ITERATIONS as f64);

    let before = Instant::now();
    let mut nodes = 0;

    for pos in POSITIONS.iter() {
        let s = search.clone();
        let res = black_box(s).start(*pos, meta, false);
        nodes += res.nodes;
    }

    let search_time = before.elapsed().as_millis();

    let search_mnps = mega_nodes_pr_sec(nodes as u128, search_time);

    println!(" Finished search bench in {} ms", search_time);
    println!("  Searched {:.4} MNodes", (nodes as f64) / (1000000 as f64));
    println!("  Speed was {search_mnps:.4} MNodes/s");

    (search_time, nodes, search_mnps)
}

fn baseline_path() -> PathBuf {
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    path.join("bench_baseline.txt")
}