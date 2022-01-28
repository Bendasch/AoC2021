use super::get_contents;

pub fn main() {
    println!("Day-02 part 1: {}", part_one());
    println!("Day-02 part 2: {}", part_two());
}

fn part_one() -> i32 {
    let contents = get_contents("src/days/day_02/input_1.txt");
    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;
    for line in contents.lines() {
        let commands: Vec<&str> = line.split(' ').collect();
        let value = commands[1].parse::<i32>().unwrap();
        match commands[0] {
            "forward" => horizontal += value,
            "down" => depth += value,
            "up" => depth -= value,
            _ => panic!("This should not happen according to the instructions."),
        };
    }
    horizontal * depth
}

fn part_two() -> i32 {
    let contents = get_contents("src/days/day_02/input_2.txt");
    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;
    for line in contents.lines() {
        let commands: Vec<&str> = line.split(' ').collect();
        let value = commands[1].parse::<i32>().unwrap();
        match commands[0] {
            "forward" => {
                horizontal += value;
                depth += aim * value;
            }
            "down" => aim += value,
            "up" => aim -= value,
            _ => panic!("This should not happen according to the instructions."),
        };
    }
    horizontal * depth
}
