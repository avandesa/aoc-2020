use aoc_2020::day_05::*;

fn main() {
    let input_file = std::env::args().nth(1).unwrap();
    let input = std::fs::read_to_string(input_file).unwrap();

    println!("Part 1: {}", part1(&input));
    if let Some(solution) = part2(&input) {
        println!("Part 2: {}", solution);
    } else {
        println!("No solution for part 2");
    }
}
