use std::collections::HashSet;

pub fn part1(input: &HashSet<i64>) -> Option<i64> {
    for num in input.iter() {
        let find = 2020 - num;

        if input.contains(&find) {
            return Some(num * find);
        }
    }

    None
}

pub fn part2(input: &HashSet<i64>) -> Option<i64> {
    for (a_index, a) in input.iter().enumerate() {
        for b in input.iter().skip(a_index) {
            if a == b {
                continue;
            }

            let c = 2020 - a - b;

            if c > 0 && input.contains(&c) {
                return Some(a * b * c);
            }
        }
    }

    None
}
