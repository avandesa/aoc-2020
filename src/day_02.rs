use regex::{Captures, Regex};

lazy_static::lazy_static! {
    static ref REGEX: Regex = Regex::new(
        r"^(?P<lower_bound>\d+)-(?P<upper_bound>\d+) (?P<required>[a-z]): (?P<password>[a-z]+)$",
    )
    .unwrap();

}

#[derive(Debug)]
struct PasswordEntry {
    pub lower_bound: usize,
    pub upper_bound: usize,
    pub required: char,
    pub password: String,
}

impl PasswordEntry {
    pub fn from_captures(caps: Captures) -> Self {
        let lower_bound = caps["lower_bound"].parse().unwrap();
        let upper_bound = caps["upper_bound"].parse().unwrap();
        let required = caps["required"].chars().next().unwrap();
        let password = caps["password"].to_string();

        Self {
            lower_bound,
            upper_bound,
            required,
            password,
        }
    }

    pub fn is_valid_part1(&self) -> bool {
        let count = self
            .password
            .chars()
            .filter(|c| *c == self.required)
            .count();

        self.lower_bound <= count && count <= self.upper_bound
    }

    pub fn is_valid_part2(&self) -> bool {
        let char1 = self.password.chars().nth(self.lower_bound - 1).unwrap();
        let char2 = self.password.chars().nth(self.upper_bound - 1).unwrap();

        (char1 == self.required) ^ (char2 == self.required)
    }
}

pub fn part1(input: &[String]) -> usize {
    input
        .iter()
        .map(|line| REGEX.captures(line).unwrap())
        .map(PasswordEntry::from_captures)
        .filter(|password| password.is_valid_part1())
        .count()
}

pub fn part2(input: &[String]) -> usize {
    input
        .iter()
        .map(|line| REGEX.captures(line).unwrap())
        .map(PasswordEntry::from_captures)
        .filter(|password| password.is_valid_part2())
        .count()
}
