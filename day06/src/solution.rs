pub fn parse<'a>(input: &str) -> String {
    input.to_string()
}

pub struct Input {
    numbers: Vec<u128>,
    operations: Vec<Operator>,
}

pub enum Operator {
    Plus,
    Mul,
}

pub fn parse_part_1<'a>(input: String) -> Input {
    let mut numbers = vec![];
    let mut operations = vec![];
    for line in input.lines() {
        if line.starts_with('+') || line.starts_with('*') {
            for op in line
                .split(" ")
                .map(|s| s.trim())
                .flat_map(|c| c.chars())
                .map(|c| match c {
                    '+' => Operator::Plus,
                    '*' => Operator::Mul,
                    _ => panic!("Invalid character {c} in characters"),
                })
            {
                operations.push(op);
            }
        } else {
            for num in line
                .split(" ")
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u128>().unwrap())
            {
                numbers.push(num);
            }
        }
    }

    Input {
        numbers,
        operations,
    }
}

pub fn solve_part_one<'a>(input: String) -> String {
    let input = parse_part_1(input);

    let mut total = 0;
    let op_count = input.operations.len();
    for (i, op) in input.operations.iter().enumerate() {
        let mut column_nums = vec![];
        for j in (i..(input.numbers.len())).step_by(op_count) {
            let num = input.numbers[j];
            column_nums.push(num);
        }

        match op {
            Operator::Plus => total += column_nums.into_iter().sum::<u128>(),
            Operator::Mul => {
                total += column_nums
                    .into_iter()
                    .reduce(|a, b| a * b)
                    .expect("There to be an element in the column")
            }
        }
    }

    total.to_string()
}

pub fn solve_part_two<'a>(input: String) -> String {
    let mut lines = input.lines().collect::<Vec<_>>();

    let op_row = lines.pop().unwrap();
    let operators: Vec<Operator> = op_row
        .split(" ")
        .map(|s| s.trim())
        .flat_map(|c| c.chars())
        .map(|c| match c {
            '+' => Operator::Plus,
            '*' => Operator::Mul,
            _ => panic!("Invalid character {c} in characters"),
        })
        .collect();

    let longest_line = lines
        .iter()
        .map(|l| l.len())
        .reduce(|a, b| a.max(b))
        .unwrap();

    let char_lines = lines
        .into_iter()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut sum = 0;
    let mut op_index = 0;
    let mut column_numbers: Vec<u128> = vec![];
    for col in 0..longest_line {
        let mut is_blank = true;
        let mut curr_num = String::new();
        for line in char_lines.iter() {
            if let Some(c) = line.get(col) {
                if c == &' ' {
                    continue;
                }

                is_blank = false;
                curr_num = format!("{curr_num}{c}");
            }
        }

        if is_blank {
            let col_sum = match operators[op_index] {
                Operator::Plus => column_numbers.into_iter().sum(),
                Operator::Mul => column_numbers.into_iter().reduce(|a, b| a * b).unwrap(),
            };

            sum += col_sum;
            op_index += 1;
            column_numbers = Vec::new()
        } else {
            column_numbers.push(curr_num.parse::<u128>().unwrap());
        }
    }
    // Handle last column numbers too.
    let col_sum = match operators[op_index] {
        Operator::Plus => column_numbers.into_iter().sum(),
        Operator::Mul => column_numbers.into_iter().reduce(|a, b| a * b).unwrap(),
    };

    sum += col_sum;

    sum.to_string()
}
