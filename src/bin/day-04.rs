use aoc_2020::day_04::*;

fn main() {
    let input_file = std::env::args().nth(1).unwrap();
    let input = std::fs::read_to_string(input_file).unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
