use std::mem::MaybeUninit;

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct BoardingPass {
    row: [FrontOrBack; 7],
    col: [LeftOrRight; 3],
}

impl BoardingPass {
    fn from_line(line: &str) -> Self {
        assert_eq!(line.len(), 10);

        let row = {
            let mut data: [MaybeUninit<FrontOrBack>; 7] =
                unsafe { MaybeUninit::uninit().assume_init() };

            let mut num_initialized = 0;
            for (i, c) in line.chars().take(7).enumerate() {
                data[i] = MaybeUninit::new(FrontOrBack::from_char(c));
                num_initialized += 1;
            }

            assert_eq!(num_initialized, 7);
            unsafe { std::mem::transmute(data) }
        };

        let col = {
            let mut data: [MaybeUninit<LeftOrRight>; 3] =
                unsafe { MaybeUninit::uninit().assume_init() };

            let mut num_initialized = 0;
            for (i, c) in line.chars().skip(7).enumerate() {
                data[i] = MaybeUninit::new(LeftOrRight::from_char(c));
                num_initialized += 1;
            }

            assert_eq!(num_initialized, 3);
            unsafe { std::mem::transmute(data) }
        };

        Self { row, col }
    }

    fn row(&self) -> u32 {
        find_row(0, 127, &self.row)
    }

    fn col(&self) -> u32 {
        find_col(0, 7, &self.col)
    }

    fn seat_id(&self) -> u32 {
        8 * self.row() + self.col()
    }
}

#[derive(Debug, Clone, Copy)]
enum FrontOrBack {
    Front,
    Back,
}

impl FrontOrBack {
    fn from_char(c: char) -> Self {
        match c {
            'F' => Self::Front,
            'B' => Self::Back,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum LeftOrRight {
    Left,
    Right,
}

impl LeftOrRight {
    fn from_char(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => unreachable!(),
        }
    }
}

// As of Rust 1.48.0, the tail call elimination optimization appears to be happening
// https://rust.godbolt.org/z/Grfz4Y
fn find_row(lower_bound: u32, upper_bound: u32, remaining: &[FrontOrBack]) -> u32 {
    if lower_bound == upper_bound {
        assert_eq!(remaining.len(), 0);
        lower_bound
    } else {
        let (new_lower, new_upper) = match remaining[0] {
            FrontOrBack::Front => (lower_bound, lower_bound + (upper_bound - lower_bound) / 2),
            FrontOrBack::Back => (
                lower_bound + (upper_bound - lower_bound) / 2 + 1,
                upper_bound,
            ),
        };

        find_row(new_lower, new_upper, &remaining[1..])
    }
}

fn find_col(lower_bound: u32, upper_bound: u32, remaining: &[LeftOrRight]) -> u32 {
    if lower_bound == upper_bound {
        assert_eq!(remaining.len(), 0);
        lower_bound
    } else {
        let (new_lower, new_upper) = match remaining[0] {
            LeftOrRight::Left => (lower_bound, lower_bound + (upper_bound - lower_bound) / 2),
            LeftOrRight::Right => (
                lower_bound + (upper_bound - lower_bound) / 2 + 1,
                upper_bound,
            ),
        };

        find_col(new_lower, new_upper, &remaining[1..])
    }
}

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| BoardingPass::from_line(line).seat_id())
        .max()
        .unwrap()
}

pub fn part2(input: &str) -> Option<u32> {
    let mut all_seats = input
        .lines()
        .map(|line| BoardingPass::from_line(line).seat_id())
        .collect::<Vec<_>>();

    all_seats.sort_unstable();

    all_seats
        .into_iter()
        .tuple_windows()
        .find(|(a, b)| *a == b - 2)
        .map(|(a, _)| a + 1)
}
