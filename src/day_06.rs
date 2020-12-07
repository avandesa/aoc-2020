use std::collections::{HashMap, HashSet};

fn coalesce_group_any_answer(group: &str, hash_set: &mut HashSet<char>) -> usize {
    hash_set.clear();
    group
        .lines()
        .fold(hash_set, |answers, person| {
            for answer in person.chars() {
                answers.insert(answer);
            }
            answers
        })
        .len()
}

fn coalesce_group_all_answer(group: &str, hash_map: &mut HashMap<char, usize>) -> usize {
    hash_map.clear();
    let num_people = group.lines().count();
    for answer in group.chars().filter(|c| c.is_ascii_lowercase()) {
        hash_map
            .entry(answer)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    hash_map
        .iter()
        .filter(|(_, count)| **count == num_people)
        .count()
}

pub fn part1(input: &str) -> usize {
    // Potential further optimization: convert each line to a 26-bit bitmap and OR/AND them all
    // together

    // Pre-allocate and re-use the hash set
    let mut set = HashSet::with_capacity(26);
    input
        .split("\n\n")
        .map(|group| coalesce_group_any_answer(group, &mut set))
        .sum()
}

pub fn part2(input: &str) -> usize {
    let mut map = HashMap::with_capacity(26);
    input
        .split("\n\n")
        .map(|group| coalesce_group_all_answer(group, &mut map))
        .sum()
}
