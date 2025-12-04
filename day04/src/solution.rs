pub struct Map {
    pub map: Vec<Vec<Tile>>,
    pub width: usize,
    pub height: usize,
}

impl Map {
    fn get_neighbours(&self, x: usize, y: usize) -> Vec<&Tile> {
        let min_x = if x > 0 { x - 1 } else { x };
        let min_y = if y > 0 { y - 1 } else { y };
        let max_x = if x < self.width - 1 { x + 1 } else { x };
        let max_y = if y < self.height - 1 { y + 1 } else { y };

        let mut neighbours = Vec::new();
        for yi in min_y..=max_y {
            for xi in min_x..=max_x {
                if xi == x && yi == y {
                    continue;
                }

                neighbours.push(&self.map[yi][xi]);
            }
        }

        neighbours
    }

    fn count_roll_neighbours(&self, x: usize, y: usize) -> usize {
        self.get_neighbours(x, y)
            .into_iter()
            .map(|neigh| if neigh == &Tile::Roll { 1 } else { 0 })
            .sum()
    }

    fn remove_roll(&mut self, x: usize, y: usize) {
        self.map[y][x] = Tile::Empty;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Tile {
    Empty,
    Roll,
}

pub fn parse<'a>(input: &str) -> Map {
    let map: Vec<Vec<Tile>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '@' => Tile::Roll,
                    _ => panic!("Unsupported tile {c}"),
                })
                .collect()
        })
        .collect();
    let height = map.len();
    let width = map[0].len();

    Map { map, height, width }
}

pub fn solve_part_one<'a>(input: Map) -> String {
    let mut reachable = 0;
    for (y, row) in input.map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if tile == &Tile::Empty {
                continue;
            }

            if input.count_roll_neighbours(x, y) < 4 {
                reachable += 1;
            }
        }
    }

    reachable.to_string()
}

pub fn solve_part_two<'a>(input: Map) -> String {
    let mut prev_removed = 0;
    let mut removed = 0;
    let mut map = input;

    while removed == 0 || prev_removed != removed {
        prev_removed = removed;
        for y in 0..map.height {
            for x in 0..map.width {
                let tile = &map.map[y][x];
                if tile == &Tile::Empty {
                    continue;
                }

                if map.count_roll_neighbours(x, y) < 4 {
                    removed += 1;
                    map.remove_roll(x, y);
                }
            }
        }
    }

    removed.to_string()
}
