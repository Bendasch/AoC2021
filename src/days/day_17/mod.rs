use super::get_contents;

pub fn main() {
    println!("Day-17 part 1: {}", part_one());
    println!("Day-17 part 2: {}", part_two());
}

fn part_one() -> i32 {
    let contents = get_contents("src/days/day_17/input.txt");
    let target = get_target_from_input(contents);
    let max_v = get_max_y_v(target);
    calc_height_from_sv(max_v)
}

fn part_two() -> usize {
    let contents = get_contents("src/days/day_17/input.txt");
    let target = get_target_from_input(contents);
    let vs = get_all_vs(&target);
    vs.len()
}

fn calc_height_from_sv(yv: i32) -> i32 {
    if yv > 0 {
        return (yv.pow(2) + yv) / 2;
    }
    0
}

fn get_max_y_v(target: Target) -> i32 {
    let y_vs = get_all_vs(&target);
    y_vs.into_iter().map(|(_, yv)| yv).max().unwrap()
}

fn get_all_vs(target: &Target) -> Vec<(i32, i32)> {
    let mut vs: Vec<(i32, i32)> = Vec::new();
    get_all_x_vs(target).into_iter().for_each(|xv| {
        let yv_bounds: (i32, i32) = get_yv_bounds(target, xv);
        for yv in yv_bounds.0..=yv_bounds.1 {
            let mut y_pos: i32 = 0;
            let mut x_pos: i32 = 0;
            let mut x_v_cur: i32 = xv;
            let mut y_v_cur: i32 = yv;
            while x_pos <= target.x1 && y_pos >= target.y1 {
                if is_pos_on_target(target, (x_pos, y_pos)) {
                    vs.push((xv, yv));
                    break;
                }
                x_pos += x_v_cur;
                y_pos += y_v_cur;
                x_v_cur = i32::max(0, x_v_cur - 1);
                y_v_cur -= 1;
            }
        }
    });
    vs
}

fn is_pos_on_target(target: &Target, pos: (i32, i32)) -> bool {
    pos.0 >= target.x0 && pos.0 <= target.x1 && pos.1 <= target.y0 && pos.1 >= target.y1
}

fn get_all_x_vs(target: &Target) -> Vec<i32> {
    let mut vs: Vec<i32> = Vec::new();
    for sv in 1..=target.x1 {
        let mut position: i32 = 0;
        let mut velocity = sv;
        while position <= target.x1 {
            if target.x0 <= position && target.x1 >= position {
                vs.push(sv);
                break;
            }
            if velocity == 0 {
                break;
            }
            position += velocity;
            velocity = i32::max(0, velocity - 1);
        }
    }
    vs
}

fn get_yv_bounds(target: &Target, xv: i32) -> (i32, i32) {
    (target.y1, get_upper_yv_bound(target, xv))
}

fn get_upper_yv_bound(target: &Target, xv: i32) -> i32 {
    let max_steps = get_max_num_steps(target, xv);
    let bound = if max_steps == i32::MAX {
        i32::abs(target.y1)
    } else {
        (max_steps - 1) / 2
    };
    for yv in (0..bound).rev() {
        let mut steps = 0;
        let mut y_pos: i32 = 0;
        let mut y_v_cur: i32 = yv;
        while steps <= max_steps && y_pos >= target.y1 {
            if y_pos <= target.y0 {
                return yv;
            };
            y_pos += y_v_cur;
            y_v_cur -= 1;
            steps += 1;
        }
    }
    0
}

fn get_max_num_steps(target: &Target, xv: i32) -> i32 {
    let mut steps = 0;
    let mut position: i32 = 0;
    let mut velocity: i32 = xv;
    while position <= target.x1 {
        if velocity == 0 {
            return i32::MAX;
        }
        steps += 1;
        position += velocity;
        velocity = i32::max(0, velocity - 1);
    }
    steps
}

#[derive(Debug)]
struct Target {
    x0: i32,
    x1: i32,
    y0: i32,
    y1: i32,
}

fn get_target_from_input(input: String) -> Target {
    let x = get_i32_from_input('x', &input);
    let y = get_i32_from_input('y', &input);
    Target {
        x0: x.0,
        x1: x.1,
        y0: y.1, // start and end are swapped here
        y1: y.0, // start and end are swapped here
    }
}

fn get_i32_from_input(dim: char, input: &str) -> (i32, i32) {
    let idx = match dim {
        'x' => input.find("x=").unwrap(),
        'y' => input.find("y=").unwrap(),
        _ => panic!("Invalid dimension!"),
    };
    let start = &input
        .chars()
        .skip(idx + 2)
        .take_while(|&x| x == '-' || x.is_digit(10))
        .collect::<String>()
        .parse::<i32>()
        .unwrap();
    let end = &input
        .chars()
        .skip(idx + 2 + start.to_string().len() + 2)
        .take_while(|&x| x == '-' || x.is_digit(10))
        .collect::<String>()
        .parse::<i32>()
        .unwrap();
    (start.to_owned(), end.to_owned())
}
