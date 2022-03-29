use criterion::{criterion_group, criterion_main, Criterion, BatchSize,Throughput};
use zaia::engine::gc::Heap;
use zaia::engine::value::{Table, Value};

const OBJECTS_COUNT: u64 = 100000;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse");
    group.throughput(Throughput::Elements(OBJECTS_COUNT));
    group.bench_function(format!("sweep {}", OBJECTS_COUNT), |b| {
        b.iter_batched(|| {
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
                        table.insert(Value::from_int(i+1), Value::from_int(i+1));
                        heap.insert(table);
                    },
                    _ => unreachable!(),
                }
            }

            heap
        }, |heap| {
            heap.collect(|_| (), |_| ());
        }, BatchSize::LargeInput);
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
