use crate::get_contents;

pub fn main() {
    println!("Day-04 part 1: {}", part_one());
    println!("Day-04 part 2: {}", part_two());
}

fn part_one() -> u32 {
    let contents = get_contents("src/day_04/input.txt");
    let mut line_iter = contents.lines().into_iter();
    let moves = line_iter.next().unwrap();
    println!("{}", moves);
    for line in line_iter {
        println!("{}", line);
    }
    0
}

fn part_two() -> u32 {
    ///let contents = get_contents("src/day_04/input.txt");
    0
}
