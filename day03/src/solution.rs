pub fn parse<'a>(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).expect("Failed to parse char as digit"))
                .collect()
        })
        .collect()
}

pub fn solve_part_one<'a>(input: Vec<Vec<u32>>) -> String {
    let mut total = 0;
    for pack in input.iter() {
        let mut first_digit = 0;
        let mut second_digit = 0;

        for i in 0..(pack.len() - 1) {
            let pi = pack[i];
            if pi > first_digit {
                first_digit = pi;
                second_digit = pack[i + 1];
            } else if pi > second_digit {
                second_digit = pi;
            }
        }

        if pack[pack.len() - 1] > second_digit {
            second_digit = pack[pack.len() - 1];
        }

        total += first_digit * 10 + second_digit;
    }

    total.to_string()
}

const NUM_DIGITS: usize = 12;

#[inline(always)]
fn to_number(index: usize, number: usize) -> usize {
    let scale = 10_usize.pow(((NUM_DIGITS - index) - 1) as u32);
    scale * number
}

pub fn solve_part_two<'a>(input: Vec<Vec<u32>>) -> String {
    let mut total = 0;
    for pack in input.iter() {
        let mut indices = [0; NUM_DIGITS];
        let mut digits = [0; NUM_DIGITS];

        for di in 0..NUM_DIGITS {
            let mut digit = digits[di];
            for pi in di..(pack.len() - (NUM_DIGITS - di - 1)) {
                let p = pack[pi];

                if di > 0 {
                    if indices[di - 1] >= pi {
                        // We should ignore anything before the previous digits index.
                        continue;
                    }
                }

                if p > digit {
                    digit = p;
                    indices[di] = pi;
                    digits[di] = digit;
                    for di_update in di..NUM_DIGITS {
                        let new_index = pi + (di_update - di);
                        digits[di_update] = pack[new_index];
                        indices[di_update] = new_index;
                    }
                }
            }
        }

        let pack_sum = digits
            .iter()
            .enumerate()
            .map(|(i, n)| to_number(i, *n as usize))
            .sum::<usize>();
        total += pack_sum;
    }

    total.to_string()
}
