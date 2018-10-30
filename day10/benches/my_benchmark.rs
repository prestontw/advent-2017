#[macro_use]
extern crate criterion;
extern crate day10;

use criterion::Criterion;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("part 2", |b| b.iter(|| day10::string_to_hash_string("106,16,254,226,55,2,1,166,177,247,93,0,255,228,60,36")));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);