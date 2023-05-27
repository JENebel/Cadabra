use std::{time::Instant, io::{stdout, Write, self, BufRead}, fs::File, path::PathBuf};
use colored::{Colorize, ColoredString};
use std::hint::black_box;
use lazy_static::lazy_static;

use crate::engine::*;

lazy_static!(
    pub static ref POSITIONS: Vec<(i8, Position)> = vec![
        (2, Position::start_pos()),
        (-4, Position::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -").unwrap()),     // Kiwipete
        (0, Position::from_fen("r2q1rk1/5pp1/1ppp3p/2n1nbN1/4p1PP/P1P1P3/Q2PBP2/R1B2RK1 b - - 1 17").unwrap()),    // JENCE
        (1, Position::from_fen("1r1q1rk1/5pp1/1NRpb2p/p3p3/8/P2P2P1/1P2PPBP/3Q1RK1 b - - 0 19").unwrap()),         // Kasparov v Georgiev
        (-4, Position::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap()), // Tricky position
        (1, Position::from_fen("rnbqkb1r/pp1p1pPp/8/2p1pP2/1P1P4/3P3P/P1P1P3/RNBQKBNR w KQkq e6 0 1").unwrap()),   // Killer position
        (-5, Position::from_fen("r2q1rk1/ppp2ppp/2n1bn2/2b1p3/3pP3/3P1NPP/PPP1NPB1/R1BQ1RK1 b - - 0 9").unwrap()), // CMK position
        (1, Position::from_fen("6k1/3q1pp1/pp5p/1r5n/8/1P3PP1/PQ4BP/2R3K1 w - - 0 1").unwrap()),
        (3, Position::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 10").unwrap()),
        (0, Position::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1").unwrap()),
        (2, Position::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap()),
        (-4, Position::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10").unwrap()),
    ];
);

const ITERATIONS: u16 = 1;

pub fn run_bench(save: bool) {
    println!();

    // PERFT BENCH
    //let perft_mnps = 0.0; 
    let (perft_time, perft_mnps) = perft_bench();

    println!();

    // SEARCH TIME TO DEPTH (ttd)
    let (search_time, nodes, search_mnps, tt_hits) = search_bench();

    // TODO
    // Stats for multithreaded
    // TT fill% + hits


    // Print results
    println!();
    println!("*** RESULTS ***");
    println!();

    println!(" Perft:");
    println!("  Perft bench took {} ms", perft_time);
    println!("  Speed was {perft_mnps:.2} MNodes/s");

    println!();

    println!(" Search:");
    println!("  Search bench took {} ms", search_time);
    println!("  Searched {} nodes ≈ {:.4} MNodes", nodes, (nodes as f64) / 1000000_f64);
    println!("  Speed was {search_mnps:.4} MNodes/s");
    println!("  TT hits: {tt_hits}");
    
    println!();

    // Load baseline
    if let Ok(file) = File::open(baseline_path()) {
        let lines = io::BufReader::new(file).lines().map(|l| l.unwrap()).collect::<Vec<String>>();

        println!(" Changes:");
        // Perft
        {
            let prev = lines.iter().find(|l| l.starts_with("perft")).unwrap().split_once(':').unwrap().1.parse::<f64>().unwrap();
            let diff_perc = ((perft_mnps - prev) / prev) * 100.;
            let diff_str = color_string_percent(format!("{:>+.3} MNodes/s", perft_mnps - prev).normal(), diff_perc, true);
            let diff_perc_str = color_string_percent(format!("{:>+.3}%", diff_perc).normal(), diff_perc, true);
            println!("  Perft performance changed by {} ≈ {}", diff_str, diff_perc_str);
        }

        // Search node count
        {
            let prev = lines.iter().find(|l| l.starts_with("nodes")).unwrap().split_once(':').unwrap().1.parse::<u128>().unwrap();
            let diff_perc = ((nodes as f64 - prev as f64) / prev as f64) * 100.;
            let diff_str = color_string_percent(format!("{:>+} nodes", nodes as i128 - prev  as i128).normal(), diff_perc, false);
            let diff_perc_str = color_string_percent(format!("{:>+.3}%", diff_perc).normal(), diff_perc, false);
            println!("  Search node count changed by {} ≈ {}", diff_str, diff_perc_str);
        }

        // Search TTD
        {
            let prev = lines.iter().find(|l| l.starts_with("s_ttd")).unwrap().split_once(':').unwrap().1.parse::<u128>().unwrap();
            let diff_perc = ((search_time as f64 - prev as f64) / prev as f64) * 100.;
            let diff_str = color_string_percent(format!("{:>+} ms", search_time as i128 - prev  as i128).normal(), diff_perc, false);
            let diff_perc_str = color_string_percent(format!("{:>+.3}%", diff_perc).normal(), diff_perc, false);
            println!("  Search time to depth changed by {} ≈ {}", diff_str, diff_perc_str);
        }

        // Search MNodes/s
        {
            let prev = lines.iter().find(|l| l.starts_with("search_mnps")).unwrap().split_once(':').unwrap().1.parse::<f64>().unwrap();
            let diff_perc = ((search_mnps - prev) / prev) * 100.;
            let diff_str = color_string_percent(format!("{:>+.4} MNodes/s", search_mnps - prev).normal(), diff_perc, true);
            let diff_perc_str = color_string_percent(format!("{:>+.3}%", diff_perc).normal(), diff_perc, true);
            println!("  Search MNodes/s changed by {} ≈ {}", diff_str, diff_perc_str);
        }
    }

    // Save results as baseline if relevant
    if save {
        let mut file = File::create(baseline_path()).expect(" Could not create baseline file");
        writeln!(file, "perft:{}", perft_mnps).unwrap();
        writeln!(file, "s_ttd:{}", search_time).unwrap();
        writeln!(file, "nodes:{}", nodes).unwrap();
        writeln!(file, "search_mnps:{}", search_mnps).unwrap();
        
        println!();
        println!(" Baseline saved");
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

fn perft_bench() -> (u128, f64) {
    println!(" Running perft benchmark...");
    print!(" Warming up...\t");

    let before_wu = Instant::now();
    
    stdout().flush().unwrap();
    for (_, pos) in POSITIONS.iter() {
        black_box(pos).perft::<false>(black_box(5));
    }
    println!("Done");
    println!(" Estimated bench time: {:.2}s", (before_wu.elapsed().as_millis() as f64 / 1000.) * ITERATIONS as f64);

    let mut nodes = 0;

    let before = Instant::now();

    for _ in 1..=ITERATIONS {
        for (_, pos) in POSITIONS.iter() {
            nodes += black_box(pos).perft::<false>(black_box(5));
        }
        stdout().flush().unwrap();
    }

    let perft_time = before.elapsed().as_millis();
    
    let perft_mnps = mega_nodes_pr_sec(nodes as u128, perft_time);

    (perft_time, perft_mnps)
}
                //(millis, nodes, MNPS)
fn search_bench() -> (u128, u128, f64, u128) {
    println!(" Running search time to depth benchmark...");

    let base_depth = 8;

    let mut nodes = 0;
    let mut tt_hits = 0;

    let mut search_time = 0;

    for (bias, pos) in POSITIONS.iter() {
        let meta = SearchArgs::new_simple_depth(black_box((base_depth as i8 + bias) as u8));
        let search = Search::new(Settings::default().transposition_table_mb(128).threads(1));
        let res = black_box(search).start(*pos, meta, true);
        search_time += res.time;
        nodes += res.nodes;
        tt_hits += res.tt_hits;
    }

    let search_mnps = mega_nodes_pr_sec(nodes, search_time);

    (search_time, nodes, search_mnps, tt_hits)
}

fn baseline_path() -> PathBuf {
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    path.join("bench_baseline.txt")
}