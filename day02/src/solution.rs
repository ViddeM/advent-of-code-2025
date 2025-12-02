pub fn parse<'a>(input: &'a str) -> impl Iterator<Item = (usize, usize)> + 'a {
    input.strip_suffix("\n").unwrap().split(",").map(|range| {
        let (low, high) = range.split_once("-").expect("FAiled to split on -");
        (
            low.parse().expect("Failed to parse low"),
            high.parse().expect("Failed to parse high"),
        )
    })
}

pub fn solve_part_one<'a>(input: impl Iterator<Item = (usize, usize)>) -> String {
    let mut invalids = 0;
    for (low, high) in input {
        for n in low..=high {
            let ns = n.to_string().chars().collect::<Vec<_>>();
            if ns.len() % 2 != 0 {
                // Numbers of uneven length cannot have two repeated numbers in it.
                continue;
            }

            let midway = ns.len() / 2;
            let first = &ns[..midway];
            let second = &ns[midway..];
            if first == second {
                invalids += n;
            }
        }
    }

    invalids.to_string()
}

pub fn solve_part_two<'a>(input: impl Iterator<Item = (usize, usize)>) -> String {
    let mut invalids = 0;
    for (low, high) in input {
        for n in low..=high {
            let mut checked = false;
            let ns = n.to_string();
            for div in 2..=ns.len() {
                if ns.len() % div == 0 && !checked {
                    let to_test = &ns[0..ns.len() / div];
                    let mut passed = true;
                    for check in 1..div {
                        let counter = &ns[check * (ns.len() / div)..(check + 1) * (ns.len() / div)];
                        if to_test != counter {
                            passed = false;
                            break;
                        }
                    }

                    if passed {
                        invalids += n;
                        checked = true;
                    }
                }
            }
        }
    }

    invalids.to_string()
}
