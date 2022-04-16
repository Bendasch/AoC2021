use super::get_contents;

pub fn main() {
    println!("Day-17 part 1: {}", part_one());
    println!("Day-17 part 2: {}", part_two());
}

fn part_one() -> i32 {
    let contents = get_contents("src/days/day_17/input.txt");
    let target = get_target_from_input(contents);
    let max_v = find_max_possible_velocity(target);
    calc_height_from_sv(max_v)
}

fn part_two() -> i32 {
    let contents = get_contents("src/days/day_17/example.txt");
    println!("{:?}", get_target_from_input(contents));
    0
}

fn calc_height_from_sv(sv: i32) -> i32 {
    return (sv.pow(2) + sv) / 2;
}

fn find_max_possible_velocity(target: Target) -> i32 {
    let min_x_v: i32 = get_min_x_v(&target);
    let max_y_v: i32 = get_max_y_v(&target, min_x_v);
    for sv in (1..max_y_v).rev() {
        let mut position: i32 = 0;
        let mut velocity: i32 = -sv;
        while position > target.y1 {
            if target.y0 >= position && target.y1 <= position {
                return sv;
            }
            position += velocity;
            velocity -= 1;
        }
    }
    return 0;
}

fn get_min_x_v(target: &Target) -> i32 {
    for sv in 1..target.x1 {
        let mut position: i32 = 0;
        let mut velocity = sv;
        while position < target.x1 {
            if target.x0 <= position && target.x1 >= position {
                return sv;
            }
            if velocity == 0 {
                break;
            }
            position += velocity;
            velocity = i32::max(0, velocity - 1);
        }
    }
    return 0;
}

fn get_max_num_steps(target: &Target, min_x_v: i32) -> i32 {
    let mut steps = 0;
    let mut position: i32 = 0;
    let mut velocity: i32 = min_x_v;
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

fn get_max_y_v(target: &Target, min_x_v: i32) -> i32 {
    i32::min(i32::abs(target.y1), get_max_num_steps(target, min_x_v))
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
    let start = i32::from_str_radix(
        &input
            .chars()
            .skip(idx + 2)
            .take_while(|&x| x == '-' || x.is_digit(10))
            .collect::<String>(),
        10,
    )
    .unwrap();
    let end = i32::from_str_radix(
        &input
            .chars()
            .skip(idx + 2 + start.to_string().len() + 2)
            .take_while(|&x| x == '-' || x.is_digit(10))
            .collect::<String>(),
        10,
    )
    .unwrap();
    (start, end)
}
