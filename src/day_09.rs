use std::collections::HashSet;

const WINDOW_SIZE: usize = 25;

fn find_sum_in_window(haystack: &[u64], sum: u64) -> Option<(u64, u64)> {
    let set: HashSet<_> = haystack.iter().filter(|elem| **elem <= sum).collect();

    for left in set.iter() {
        let right = sum - *left;

        if set.contains(&right) {
            assert_eq!(*left + right, sum);
            return Some((**left, right));
        }
    }

    None
}

pub fn find_invalid_window(input: &[u64]) -> Option<u64> {
    // Potential optimization: maintain a hash set of values, replacing the old value with the new one as the window shifts
    for window in input.windows(WINDOW_SIZE + 1) {
        let last_elem = window[WINDOW_SIZE];
        let haystack = &window[0..WINDOW_SIZE];

        if find_sum_in_window(haystack, last_elem).is_none() {
            return Some(last_elem);
        }
    }

    None
}

pub fn part1(input: &str) -> Option<u64> {
    let numbers: Vec<u64> = input.lines().map(|line| line.parse().unwrap()).collect();
    assert!(numbers.len() > WINDOW_SIZE);
    find_invalid_window(&numbers)
}

pub fn find_contigious_range_of_size_n(input: &[u64], size: usize, sum: u64) -> Option<&[u64]> {
    input
        .windows(size)
        .map(|window| (window, window.iter().sum::<u64>()))
        .find(|(_, window_sum)| *window_sum == sum)
        .map(|(window, _)| window)
}

pub fn part2(input: &str) -> Option<u64> {
    let numbers: Vec<u64> = input.lines().map(|line| line.parse().unwrap()).collect();
    let target = find_invalid_window(&numbers).unwrap();

    for size in 2.. {
        if let Some(range) = find_contigious_range_of_size_n(&numbers, size, target) {
            let sum = range.iter().min().unwrap() + range.iter().max().unwrap();
            return Some(sum);
        }
    }

    None
}
