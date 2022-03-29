#![feature(int_log)]

use criterion::{criterion_group, criterion_main, BatchSize, Criterion, Throughput};
use zaia::engine::{
    gc::{Heap, Trace},
    value::{Table, Value},
};

const OBJECTS_COUNT: u64 = 100000;
const GRAPH_BREADTH: u64 = 3;

fn mark(c: &mut Criterion) {
    let mut group = c.benchmark_group("mark");
    let depth = OBJECTS_COUNT.log(GRAPH_BREADTH) as u64;
    let actual_count = GRAPH_BREADTH.pow(depth as u32);
    group.throughput(Throughput::Elements(actual_count));
    group.bench_function("mark", |b| {
        b.iter_batched(
            || {
                fn widen(heap: &Heap, node: &mut Table, depth: u64) {
                    if depth == 0 {
                        return;
                    }

                    for i in 0..GRAPH_BREADTH {
                        let mut table = Table::new(heap.clone());
                        widen(heap, &mut table, depth - 1);
                        let value = Value::from_table(heap.insert(table));
                        node.insert(Value::from_int(i as i32), value);
                    }
                }

                let heap = Heap::new();
                let mut root = Table::new(heap.clone());
                widen(&heap, &mut root, depth);
                let handle = heap.insert(root);
                (heap, handle)
            },
            |(heap, handle)| {
                heap.collect(
                    |visitor| {
                        unsafe {
                            handle.get_unchecked_mut().visit(visitor);
                        }

                        visitor.mark(handle.tagged());
                    },
                    |_| unreachable!(),
                );
            },
            BatchSize::LargeInput,
        );
    });

    group.finish();
}

fn sweep(c: &mut Criterion) {
    let mut group = c.benchmark_group("sweep");
    group.throughput(Throughput::Elements(OBJECTS_COUNT));
    group.bench_function("sweep", |b| {
        b.iter_batched(
            || {
                let heap = Heap::new();

                for i in 0..OBJECTS_COUNT {
                    match i % 3 {
                        0 | 1 => {
                            heap.insert_string(b"Foo Fighters!");
                        },
                        2 => {
                            let mut table = Table::new(heap.clone());
                            let i = i as i32;
                            table.insert(Value::from_int(i), Value::from_int(i));
                            table.insert(Value::from_int(i + 1), Value::from_int(i + 1));
                            heap.insert(table);
                        },
                        _ => unreachable!(),
                    }
                }

                heap
            },
            |heap| {
                heap.collect(|_| (), |_| ());
            },
            BatchSize::LargeInput,
        );
    });

    group.finish();
}

criterion_group!(benches, mark, sweep);
criterion_main!(benches);
