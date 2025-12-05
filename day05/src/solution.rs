pub struct Input {
    pub ranges: Vec<(u64, u64)>,
    pub available_ids: Vec<u64>,
}

pub fn parse<'a>(input: &str) -> Input {
    let (ranges, available) = input.split_once("\n\n").unwrap();

    let ranges = ranges
        .lines()
        .map(|l| {
            let (low, high) = l.split_once("-").unwrap();
            (low.parse().unwrap(), high.parse().unwrap())
        })
        .collect::<Vec<_>>();

    let available_ids = available
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<_>>();

    Input {
        ranges,
        available_ids,
    }
}

pub fn solve_part_one<'a>(input: Input) -> String {
    let mut count = 0;
    'outer: for id in input.available_ids.iter() {
        for (low, high) in input.ranges.iter() {
            if id >= low && id <= high {
                count += 1;
                continue 'outer;
            }
        }
    }

    count.to_string()
}

pub fn solve_part_two<'a>(input: Input) -> String {
    let mut sets = input.ranges;

    let mut keep_going = true;
    'outer: while keep_going {
        keep_going = false;
        for i in 0..sets.len() {
            let (low, high) = sets[i];
            for j in 0..sets.len() {
                if i == j {
                    continue;
                }

                let (jlow, jhigh) = sets[j];

                if low >= jlow && low <= jhigh {
                    // Ranges overlap, merge
                    sets[j] = (jlow, high.max(jhigh));
                    sets.remove(i);
                    keep_going = true;
                    continue 'outer;
                }
            }
        }
    }

    let mut sum = 0;
    for (low, high) in sets.into_iter() {
        sum += (high - low) + 1;
    }

    sum.to_string()
}
