use std::{fs, time::Duration};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cadabra::{perft::*, Position};

pub fn perft_bench(c: &mut Criterion) {
    let mut c = c.benchmark_group("perft_bench_group");
    c.sampling_mode(criterion::SamplingMode::Flat);
    
    // Load positions
    let lines = fs::read_to_string("./benches/bench_positions.txt")
        .expect("Should have been able to read the file");

    let positions: Vec<(&str, u8, &str)> = lines.split('\n').map(|l| {
        let parts = l.split(',').collect::<Vec<&str>>();
        let name = parts[0].trim();
        let depth = parts[1].trim().parse::<u8>().unwrap();
        let fen = parts[2].trim();
        (name, depth, fen)
    }).filter(|n| !n.0.starts_with("#")).collect();

    c.bench_function("Perft bench", |b|  b.iter(||
        for (_, depth, fen) in &positions {
            black_box(perft::<false>(black_box(&Position::from_fen(fen).unwrap()), *depth));
        }
    ));
}

criterion_group! {
    name = perft_bench_group;
    config = Criterion::default()
        .measurement_time(Duration::from_secs(60))
        .warm_up_time(Duration::from_secs(5))
        .sample_size(10)
        .confidence_level(0.9);
    targets = perft_bench
}

criterion_main!(perft_bench_group);