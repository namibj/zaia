use std::fs;

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use cstree::NodeCache;
use zaia::parser::parse;

fn criterion_benchmark(c: &mut Criterion) {
    let source = fs::read_to_string("test-files/mixed.lua").unwrap();
    let mut group = c.benchmark_group("parse");
    let mut deferred = Vec::new();
    group.throughput(Throughput::Elements(source.lines().count() as u64));
    group.bench_function("mixed.lua", |b| {
        b.iter(|| parse_mixed(black_box(&source), &mut deferred));
    });

    group.finish();
}

fn parse_mixed(source: &str, deferred: &mut Vec<NodeCache<'static>>) {
    let mut cache = NodeCache::new();
    parse(&mut cache, source);
    deferred.push(cache);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
