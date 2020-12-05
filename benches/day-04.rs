use aoc_2020::day_04::*;

use criterion::{criterion_group, criterion_main, Criterion};

pub fn benchmark(c: &mut Criterion) {
    let input = std::fs::read_to_string("input/day-04.txt").expect("Couldn't read input file");

    c.bench_function("Day 04 Part 1", |b| b.iter(|| part1(&input)));
    c.bench_function("Day 04 Part 2", |b| b.iter(|| part2(&input)));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
