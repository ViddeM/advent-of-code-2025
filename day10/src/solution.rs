use std::{collections::BinaryHeap, usize};

use priority_queue::PriorityQueue;

pub type Input = Vec<Puzzle>;

#[derive(Debug, Clone)]
pub struct Puzzle {
    diagram: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    voltage: Vec<usize>,
}

pub fn parse<'a>(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            let (diagram, buttons_str) = l.split_once(' ').unwrap();
            let diagram = diagram
                .chars()
                .skip(1)
                .take_while(|&c| c != ']')
                .map(|c| {
                    if c == '#' {
                        true
                    } else if c == '.' {
                        false
                    } else {
                        panic!("Illegal char '{c}' in diagram")
                    }
                })
                .collect();

            let mut buttons = Vec::new();
            let mut voltage = None;
            for button in buttons_str.split(' ') {
                if button.starts_with('{') {
                    if voltage.is_some() {
                        panic!("More than one voltage in a row?");
                    }

                    voltage = Some(
                        button
                            .strip_prefix('{')
                            .unwrap()
                            .strip_suffix('}')
                            .unwrap()
                            .split(',')
                            .map(|n| n.parse().unwrap())
                            .collect(),
                    );
                } else {
                    buttons.push(
                        button
                            .strip_prefix('(')
                            .unwrap()
                            .strip_suffix(')')
                            .unwrap()
                            .split(',')
                            .map(|n| n.parse().unwrap())
                            .collect(),
                    );
                }
            }

            Puzzle {
                diagram,
                buttons,
                voltage: voltage.unwrap(),
            }
        })
        .collect()
}

fn display_state(state: &Vec<bool>) -> String {
    state.iter().map(|b| if *b { "#" } else { "." }).collect()
}

#[inline(always)]
fn apply_button_press(pre_state: &Vec<bool>, buttons: &Vec<usize>) -> Vec<bool> {
    let mut new_state = pre_state.clone();
    for &b in buttons.iter() {
        new_state[b] = !new_state[b];
    }
    new_state
}

fn solve_puzzle(target: &Vec<bool>, buttons: &Vec<Vec<usize>>) -> usize {
    let mut states = PriorityQueue::new();
    states.push(vec![false; target.len()], 0 as i64);

    while let Some((state, presses)) = states.pop() {
        // println!("\t{}, presses: {}", display_state(&state), presses.abs());
        // println!("\t\t\tSTATES: {states:?}");
        if &state == target {
            return presses.abs() as usize;
        }

        for button in buttons.iter() {
            let new_state = apply_button_press(&state, button);
            let presses = presses - 1;
            // println!(
            //     "\t\tPushed button: {button:?} which lead to new state {} at {} presses",
            //     display_state(&new_state),
            //     presses.abs()
            // );

            if !states.contains(&new_state) {
                // Already contains state and it should be lower than what we have now.
                states.push(new_state, presses);
            }
        }
    }

    panic!("No solution found")
}

pub fn solve_part_one<'a>(input: Input) -> String {
    let mut total = 0;
    for puzzle in input.into_iter() {
        // println!("Puzzle: {puzzle:?}");
        let puzzle_num = solve_puzzle(&puzzle.diagram, &puzzle.buttons);
        total += puzzle_num;
        // println!("\tPuzzle solution: {puzzle_num} total is now {total}\n\n\n");
    }
    total.to_string()
}

pub fn solve_part_two<'a>(input: Input) -> String {
    todo!("Part two is not yet implemented");
}
