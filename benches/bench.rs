use aoc2024::*;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(criterion: &mut Criterion) {
    for day in 1..=DAYS {
        let mut group = criterion.benchmark_group(format!("Day {day:02}"));

        group.warm_up_time(std::time::Duration::from_secs(1));

        if let Some(sample_size) = bench_sample_size(day) {
            group.sample_size(sample_size);
        }
    
        group.bench_function("Part 1", |b| b.iter(|| run(day, Part1)));

        group.bench_function("Part 2", |b| b.iter(|| run(day, Part2)));
    
        group.finish();
    
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
