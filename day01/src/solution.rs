#[derive(Debug, Clone)]
pub enum Rot {
    Left(i64),
    Right(i64),
}

pub fn parse<'a>(input: &'a str) -> impl Iterator<Item = Rot> + 'a {
    input.lines().map(|l| {
        if let Some(n) = l.strip_prefix('R') {
            Rot::Right(n.parse::<i64>().unwrap())
        } else if let Some(n) = l.strip_prefix('L') {
            Rot::Left(n.parse::<i64>().unwrap())
        } else {
            panic!("Failed to strip prefix {l}");
        }
    })
}

pub fn solve_part_one<'a>(input: impl Iterator<Item = Rot>) -> String {
    let mut dial = 50;
    let mut count = 0;

    for rot in input {
        match rot {
            Rot::Left(n) => dial -= n,
            Rot::Right(n) => dial += n,
        }

        while dial < 0 {
            dial += 100;
        }

        while dial >= 100 {
            dial -= 100;
        }

        if dial == 0 {
            count += 1;
        }
    }

    count.to_string()
}

pub fn solve_part_two<'a>(input: impl Iterator<Item = Rot>) -> String {
    let mut dial = 50;
    let mut count = 0;

    for rot in input {
        println!("Rot: {rot:?}");
        let mut was_zero = dial == 0;
        match rot {
            Rot::Left(n) => {
                dial -= n;
            }
            Rot::Right(n) => {
                dial += n;
            }
        }

        while dial < 0 {
            if !was_zero {
                count += 1;
            } else {
                was_zero = false;
            }
            dial += 100;
        }

        if dial == 0 {
            count += 1;
        }

        while dial >= 100 {
            count += 1;
            dial -= 100;
        }

        println!("\tDIAL: {dial} COUNT: {count}");
    }

    count.to_string()
}
