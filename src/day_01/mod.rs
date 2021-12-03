use crate::get_contents;

pub fn main() {
    println!("Day-01 part 1: {}", part_one());
    println!("Day-01 part 2: {}", part_two());
}

fn part_one() -> u32 {
    let contents = get_contents("src/day_01/input_1.txt");
    let mut counter: u32 = 0;
    let mut line_iter = contents.lines().into_iter();
    let mut prev_value: i32 = line_iter.next().unwrap().parse::<i32>().unwrap();
    loop {
        match line_iter.next() {
            Some(value) => {
                let value = value.parse::<i32>().unwrap();
                if value > prev_value {
                    counter += 1;
                }
                prev_value = value;
            }
            None => break,
        }
    }
    counter
}

fn part_two() -> u32 {
    let contents = get_contents("src/day_01/input_2.txt");
    let mut line_iter = contents.lines().into_iter();
    let mut value_one = line_iter.next().unwrap().parse::<i32>().unwrap();
    let mut value_two = line_iter.next().unwrap().parse::<i32>().unwrap();
    let mut value_three = line_iter.next().unwrap().parse::<i32>().unwrap();
    let mut counter: u32 = 0;
    loop {
        match line_iter.next() {
            Some(line) => {
                let value_four = line.parse::<i32>().unwrap();
                if value_four > value_one {
                    counter += 1;
                }
                value_one = value_two;
                value_two = value_three;
                value_three = value_four;
            }
            None => break,
        }
    }
    counter
}
