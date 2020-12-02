use aoc_2020::day_02::*;

use std::io::prelude::*;

use criterion::{criterion_group, criterion_main, Criterion};

pub fn benchmark(c: &mut Criterion) {
    let file = std::fs::File::open("input/day-02.txt").expect("Couldn't open input file");
    let input: Vec<String> = std::io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    c.bench_function("Day 02 Part 1", |b| b.iter(|| part1(&input)));
    c.bench_function("Day 02 Part 2", |b| b.iter(|| part2(&input)));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
