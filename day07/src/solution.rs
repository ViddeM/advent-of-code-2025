use std::{
    collections::{HashMap, HashSet},
    path,
};

pub struct Input {
    map: Vec<Vec<Tile>>,
    start_x: usize,
    start_y: usize,
    width: usize,
    height: usize,
}

#[derive(Debug, Clone)]
pub enum Tile {
    Empty,
    Splitter,
}

pub fn parse<'a>(input: &str) -> Input {
    let mut start_pos: Option<(usize, usize)> = None;
    let mut map = Vec::new();
    for (y, row) in input.lines().enumerate() {
        let mut r = Vec::new();
        for (x, c) in row.chars().enumerate() {
            let tile = match c {
                '.' => Tile::Empty,
                'S' => {
                    start_pos = Some((x, y));
                    Tile::Empty
                }
                '^' => Tile::Splitter,
                _ => panic!("ILLEGAL CHARACTER {c}"),
            };
            r.push(tile);
        }
        map.push(r);
    }

    let (sx, sy) = start_pos.unwrap();

    let width = map[0].len();
    let height = map.len();

    Input {
        map,
        start_x: sx,
        start_y: sy,
        width,
        height,
    }
}

pub fn solve_part_one<'a>(input: Input) -> String {
    let mut split_count = 0;
    let mut beams = vec![(input.start_x, input.start_y)];

    // Assumes no splitters on lowest level.
    for y in 0..(input.height - 1) {
        // print!("=====================================");
        // print!("ROW: {y}");
        // print!("=====================================\n");

        let mut next_beams = HashSet::new();
        while let Some((bx, _)) = beams.pop() {
            // println!("Dealing with beam at ({bx}, {y}) ");
            let next_y = y + 1;
            let next_tile = &input.map[next_y][bx];
            // println!("\tNext tile is {next_tile:?}");
            match next_tile {
                Tile::Empty => {
                    next_beams.insert((bx, next_y));
                }
                Tile::Splitter => {
                    split_count += 1;
                    if bx > 0 {
                        next_beams.insert((bx - 1, next_y));
                    }
                    if bx < input.width - 1 {
                        next_beams.insert((bx + 1, next_y));
                    }
                }
            }
        }
        beams = next_beams.into_iter().collect();

        // println!("\n\n");
    }

    split_count.to_string()
}

fn part_2_rec(
    input: &Input,
    curr_x: usize,
    curr_y: usize,
    split_path_count: &mut HashMap<(usize, usize), usize>,
) -> usize {
    let next_y = curr_y + 1;
    if next_y == input.height - 1 {
        return 1;
    }

    let next_tile = &input.map[next_y][curr_x];
    match next_tile {
        Tile::Empty => part_2_rec(input, curr_x, next_y, split_path_count),
        Tile::Splitter => {
            if let Some(total) = split_path_count.get(&(curr_x, curr_y)) {
                return *total;
            }

            let left_path = if curr_x > 0 {
                part_2_rec(input, curr_x - 1, next_y, split_path_count)
            } else {
                0
            };

            let right_path = if curr_x < input.width - 1 {
                part_2_rec(input, curr_x + 1, next_y, split_path_count)
            } else {
                0
            };

            let total = left_path + right_path;
            split_path_count.insert((curr_x, curr_y), total);
            total
        }
    }
}

pub fn solve_part_two<'a>(input: Input) -> String {
    let mut path_count: HashMap<(usize, usize), usize> = HashMap::new();
    let total = part_2_rec(&input, input.start_x, input.start_y, &mut path_count);

    total.to_string()
}
