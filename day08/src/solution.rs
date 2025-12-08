use std::collections::{HashMap, HashSet};

pub struct Input {
    box_positions: Vec<Position>,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Position {
    x: i64,
    y: i64,
    z: i64,
}

impl Position {
    fn dist_to(&self, other: &Position) -> f64 {
        let squares = ((self.x - other.x).pow(2)
            + (self.y - other.y).pow(2)
            + (self.z - other.z).pow(2)) as f64;
        squares.sqrt()
    }
}

pub fn parse<'a>(input: &str) -> Input {
    let box_positions = input
        .lines()
        .map(|l| {
            let (x, rest) = l.split_once(',').unwrap();
            let (y, z) = rest.split_once(',').unwrap();
            Position {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
                z: z.parse().unwrap(),
            }
        })
        .collect();

    Input { box_positions }
}

// const ROUNDS: usize = 10;
const ROUNDS: usize = 1000;

pub fn solve_part_one<'a>(input: Input) -> String {
    let mut pairs = Vec::new();
    let mut pair_length = HashMap::new();

    for pi in 0..(input.box_positions.len()) {
        let pos = &input.box_positions[pi];
        for pj in (pi + 1)..(input.box_positions.len()) {
            let pos2 = &input.box_positions[pj];
            // println!("Dealing with {pi}, {pj} => {pos:?}, {pos2:?}");
            pairs.push((pos, pos2));
            pair_length.insert((pos, pos2), pos.dist_to(pos2));
        }
    }

    pairs.sort_by(|p1, p2| {
        let p1_len = *pair_length.get(p1).unwrap();
        let p2_len = *pair_length.get(p2).unwrap();

        p1_len.total_cmp(&p2_len)
    });

    let mut next_set_id = 0;
    let mut set_member: HashMap<&Position, usize> = HashMap::new();

    for (a, b) in pairs.iter() {
        let merge = match (set_member.get(a), set_member.get(b)) {
            (None, None) => {
                /* neither pos is in a set, create a new set */
                set_member.insert(a, next_set_id);
                set_member.insert(b, next_set_id);
                next_set_id += 1;
                None
            }
            (None, Some(b_set)) => {
                /* B is a part of a set, add A to that set */
                set_member.insert(a, *b_set);
                None
            }
            (Some(a_set), None) => {
                /* A is a part of a set, add B to that set */
                set_member.insert(b, *a_set);
                None
            }
            (Some(a_set), Some(b_set)) => {
                if a_set == b_set {
                    /* both are already members of the same set, do nothing */
                    continue;
                }

                /* Merge set B into set A */
                let mut to_merge = vec![];
                for (p, set) in set_member.iter() {
                    if set == b_set {
                        to_merge.push(*p);
                    }
                }

                Some((a_set.clone(), to_merge.clone()))
            }
        };

        if let Some((set, to_merge)) = merge {
            for pos in to_merge.into_iter() {
                set_member.entry(pos).and_modify(|curr| *curr = set);
            }
        }
    }

    let mut set_member_counts = HashMap::new();
    for (_, set) in set_member.into_iter() {
        set_member_counts
            .entry(set)
            .and_modify(|n| *n += 1)
            .or_insert(1);
    }

    let mut set_member_counts = set_member_counts.values().collect::<Vec<_>>();
    set_member_counts.sort_by(|a, b| b.cmp(a));

    set_member_counts
        .into_iter()
        .take(3)
        .fold(1, |a, b| a * b)
        .to_string()
}

struct Sets<'a> {
    sets: Vec<HashSet<&'a Position>>,
}

impl<'a> Sets<'a> {
    fn new() -> Self {
        Self { sets: vec![] }
    }

    fn new_set(&mut self) -> usize {
        self.sets.push(HashSet::new());
        self.sets.len() - 1
    }

    fn insert(&mut self, set_id: usize, pos: &'a Position) {
        self.sets[set_id].insert(pos);
    }

    fn set_for(&self, pos: &'a Position) -> Option<usize> {
        for (set_id, set) in self.sets.iter().enumerate() {
            if set.contains(pos) {
                return Some(set_id);
            }
        }

        None
    }

    fn merge_sets(&mut self, id1: usize, id2: usize) {
        let merge_from = id1.max(id2);
        let merge_to = id1.min(id2);
        let removed_set = self.sets.remove(merge_from);
        for p in removed_set.into_iter() {
            self.sets[merge_to].insert(p);
        }
    }

    fn set_count(&self) -> usize {
        self.sets.len()
    }

    fn set_len(&self, set_id: usize) -> usize {
        self.sets[set_id].len()
    }
}

pub fn solve_part_two<'a>(input: Input) -> String {
    let mut pairs = Vec::new();
    let mut pair_length = HashMap::new();

    for pi in 0..(input.box_positions.len()) {
        let pos = &input.box_positions[pi];
        for pj in (pi + 1)..(input.box_positions.len()) {
            let pos2 = &input.box_positions[pj];
            pairs.push((pos, pos2));
            pair_length.insert((pos, pos2), pos.dist_to(pos2));
        }
    }

    pairs.sort_by(|p1, p2| {
        let p1_len = *pair_length.get(p1).unwrap();
        let p2_len = *pair_length.get(p2).unwrap();

        p1_len.total_cmp(&p2_len)
    });

    println!("Done with prep!");

    let mut sets = Sets::new();

    for (a, b) in pairs.iter() {
        match (sets.set_for(a), sets.set_for(b)) {
            (None, None) => {
                /* neither pos is in a set, create a new set */
                let set_id = sets.new_set();
                sets.insert(set_id, a);
                sets.insert(set_id, b);
            }
            (None, Some(b_set)) => {
                /* B is a part of a set, add A to that set */
                sets.insert(b_set, a);
            }
            (Some(a_set), None) => {
                /* A is a part of a set, add B to that set */
                sets.insert(a_set, b);
            }
            (Some(a_set), Some(b_set)) => {
                if a_set == b_set {
                    /* both are already members of the same set, do nothing */
                    continue;
                }

                sets.merge_sets(a_set, b_set);
            }
        };

        if sets.set_count() == 1 && sets.set_len(0) == input.box_positions.len() {
            // We've reached the end!
            return (a.x * b.x).to_string();
        }
    }

    panic!("Failed to find solution");
}
