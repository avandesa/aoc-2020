use aoc_2020::day_01::*;

use std::{collections::HashSet, io::prelude::*};

use criterion::{criterion_group, criterion_main, Criterion};

pub fn benchmark(c: &mut Criterion) {
    let file = std::fs::File::open("input/day-01.txt").expect("Couldn't open input file");
    let input: HashSet<i64> = std::io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    c.bench_function("Day 01 Part 1", |b| b.iter(|| part1(&input)));
    c.bench_function("Day 01 Part 2", |b| b.iter(|| part2(&input)));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
