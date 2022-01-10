use std::fs;

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use zaia::parser::parse;

fn criterion_benchmark(c: &mut Criterion) {
    let src = fs::read_to_string("benches/big.lua").unwrap();
    let mut group = c.benchmark_group("parse");
    group.throughput(Throughput::Bytes(src.len() as u64));
    group.bench_function("parse big.lua", |b| b.iter(|| parse(black_box(&src))));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
