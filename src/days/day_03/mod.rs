use super::get_contents;

pub fn main() {
    println!("Day-03 part 1: {}", part_one());
    println!("Day-03 part 2: {}", part_two());
}

fn part_one() -> u32 {
    let contents = get_contents("src/days/day_03/input.txt");
    const WIDTH: usize = 12;

    let mut counter: [u32; WIDTH] = [0; WIDTH];
    let mut line_count = 0;
    for line in contents.lines() {
        for (i, count) in counter.iter_mut().enumerate().take(WIDTH) {
            *count += line.chars().collect::<Vec<char>>()[i].to_digit(2).unwrap();
        }
        line_count += 1;
    }

    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;
    for (i, count) in counter.iter_mut().enumerate().take(WIDTH) {
        if *count > (line_count / 2) as u32 {
            gamma += u32::pow(2, (WIDTH - (i + 1)) as u32);
        } else {
            epsilon += u32::pow(2, (WIDTH - (i + 1)) as u32);
        }
    }
    gamma * epsilon
}

fn part_two() -> u32 {
    let contents = get_contents("src/days/day_03/input.txt");
    const WIDTH: usize = 12;

    let mut oxy = contents.lines().collect::<Vec<&str>>();
    let mut co2 = contents.lines().collect::<Vec<&str>>();
    for i in 0..WIDTH {
        if oxy.len() > 1 {
            oxy = get_subset(oxy, i, Frequency::Most);
        }
        if co2.len() > 1 {
            co2 = get_subset(co2, i, Frequency::Least);
        }
    }

    let mut oxy_value = 0;
    let mut co2_value = 0;
    let oxy_chars = oxy[0].chars().collect::<Vec<char>>();
    let co2_chars = co2[0].chars().collect::<Vec<char>>();
    for i in 0..WIDTH {
        if oxy_chars[i] == '1' {
            oxy_value += u32::pow(2, (WIDTH - (i + 1)) as u32);
        }
        if co2_chars[i] == '1' {
            co2_value += u32::pow(2, (WIDTH - (i + 1)) as u32);
        }
    }
    oxy_value * co2_value
}

#[derive(PartialEq)]
enum Frequency {
    Most,
    Least,
}

#[allow(clippy::all)]
fn get_subset(lines: Vec<&str>, index: usize, freq: Frequency) -> Vec<&str> {
    let mut line_count = 0.;
    let mut zeros = Vec::new();
    let mut ones = Vec::new();

    for line in lines {
        match line.chars().collect::<Vec<char>>()[index] {
            '0' => zeros.push(line),
            '1' => ones.push(line),
            _ => panic!("This should not happen according to the instructions"),
        };
        line_count += 1.;
    }

    let ones_count = ones.len() as f32;
    match freq {
        Frequency::Most => {
            if ones_count >= (line_count / 2.) {
                return ones;
            } else {
                return zeros;
            }
        }
        Frequency::Least => {
            if (ones_count >= (line_count / 2.) && ones_count < line_count) || ones_count == 0. {
                return zeros;
            } else {
                return ones;
            }
        }
    };
}
