pub type Input = Vec<(i64, i64)>;

pub fn parse<'a>(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

pub fn solve_part_one<'a>(input: Input) -> String {
    let mut largest = 0;
    for (x, y) in input.iter() {
        for (x2, y2) in input.iter() {
            if x == x2 && y == y2 {
                continue;
            }

            let width = 1 + x.max(x2) - x.min(x2);
            let height = 1 + y.max(y2) - y.min(y2);
            let area = width * height;

            if area > largest {
                largest = area;
            }
        }
    }

    largest.to_string()
}

pub fn solve_part_two<'a>(input: Input) -> String {
    let rects = get_rects(&input);

    for &((ax, ay), (bx, by)) in rects.iter() {
        if check_rect(ax, ay, bx, by, &input) {
            let width = (ax - bx).abs() + 1;
            let height = (ay - by).abs() + 1;
            let area = width * height;
            return area.to_string();
        }
    }

    panic!("no solutions found :(");
}

fn check_rect(ax: i64, ay: i64, bx: i64, by: i64, input: &Input) -> bool {
    let min_x = ax.min(bx);
    let min_y = ay.min(by);
    let max_x = ax.max(bx);
    let max_y = ay.max(by);

    // Check all edges.
    for (i, &(p1x, p1y)) in input.iter().enumerate() {
        let next_index = if i < (input.len() - 1) { i + 1 } else { 0 };
        let (p2x, p2y) = input[next_index];

        if p1x == p2x {
            let edge_y_min = p1y.min(p2y);
            let edge_y_max = p1y.max(p2y);
            if min_x < p1x && max_x > p1x && !(min_y >= edge_y_max || max_y <= edge_y_min) {
                return false;
            }
        } else if p1y == p2y {
            let edge_x_min = p1x.min(p2x);
            let edge_x_max = p1x.max(p2x);
            if min_y < p1y && max_y > p1y && !(min_x >= edge_x_max || max_x <= edge_x_min) {
                return false;
            }
        } else {
            panic!("Diagonal lines??")
        }
    }

    true
}

fn get_rects(input: &Input) -> Vec<((i64, i64), (i64, i64))> {
    let mut rects = vec![];
    for i in 0..(input.len() - 1) {
        for j in (i + 1)..input.len() {
            rects.push((input[i], input[j]));
        }
    }
    rects.sort_by(|&((ax1, ay1), (ax2, ay2)), &((bx1, by1), (bx2, by2))| {
        let a_area = calc_area(ax1, ay1, ax2, ay2);
        let b_area = calc_area(bx1, by1, bx2, by2);
        b_area.cmp(&a_area)
    });
    // println!("Rects: {rects:?}");
    rects
}

#[inline(always)]
fn calc_area(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
    let width = (x1 - x2).abs() + 1;
    let height = (y1 - y2).abs() + 1;
    width * height
}
