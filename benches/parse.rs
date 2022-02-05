use std::fs;

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use zaia::parser::parse;
use zaia::intern::Interner;

fn criterion_benchmark(c: &mut Criterion) {
    let src = fs::read_to_string("test-files/mixed.lua").unwrap();
    let mut group = c.benchmark_group("parse");
    group.throughput(Throughput::Bytes(src.len() as u64));
    let mut interner = Interner::new();
    group.bench_function("parse mixed.lua", |b| b.iter(|| parse(&mut interner, black_box(&src))));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
