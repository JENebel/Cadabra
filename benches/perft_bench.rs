use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cadabra::{perft::*, Position};

pub fn bench_perft(c: &mut Criterion) {
    c.bench_function("Perft Startpos", |b|  b.iter( ||
        perft::<false>(black_box(&Position::start_pos()), black_box(4))
    ));
}

criterion_group!(benches, bench_perft);
criterion_main!(benches);