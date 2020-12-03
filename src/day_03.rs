#[derive(Clone, Copy, Eq, PartialEq)]
enum Tile {
    Open,
    Tree,
}

impl Tile {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '.' => Some(Self::Open),
            '#' => Some(Self::Tree),
            _ => None,
        }
    }
}

struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl Map {
    pub fn from_str(input: &str) -> Self {
        Self {
            width: input.find('\n').unwrap(),
            height: input.lines().count(),
            tiles: input.chars().filter_map(Tile::from_char).collect(),
        }
    }

    pub fn tile(&self, x: usize, y: usize) -> Tile {
        assert!(y < self.height);

        let trunc_x = x % self.width;
        let i = y * self.width + trunc_x;

        self.tiles[i]
    }

    pub fn trees_on_slope(&self, delta_x: usize, delta_y: usize) -> u64 {
        let mut x = 0;
        let mut y = 0;
        let mut trees_encountered = 0;

        while y < self.height {
            if self.tile(x, y) == Tile::Tree {
                trees_encountered += 1;
            }

            x += delta_x;
            y += delta_y;
        }

        trees_encountered
    }
}

pub fn part1(input: &str) -> u64 {
    Map::from_str(input).trees_on_slope(1, 3)
}

pub fn part2(input: &str) -> u64 {
    let map = Map::from_str(input);

    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(delta_x, delta_y)| map.trees_on_slope(*delta_x, *delta_y))
        .product()
}
