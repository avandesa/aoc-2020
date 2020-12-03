use aoc_2020::day_03::*;

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let input = std::fs::read_to_string(filename).expect("Couldn't read input file");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
