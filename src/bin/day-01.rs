use std::{collections::HashSet, io::prelude::*};

use aoc_2020::day_01::*;

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).expect("Couldn't open input file");
    let input: HashSet<i64> = std::io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    if let Some(solution) = part1(&input) {
        println!("Part 1: {}", solution);
    } else {
        println!("No solution for part 1");
    }

    if let Some(solution) = part2(&input) {
        println!("Part 2: {}", solution);
    } else {
        println!("No solution for part 2");
    }
}
