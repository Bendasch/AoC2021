use super::get_contents;

pub fn main() {
    println!("Day-18 part 1: {}", part_one());
    println!("Day-18 part 2: {}", part_two());
}

fn part_one() -> i32 {
    let _contents = get_contents("src/days/day_18/input.txt");
    0
}

fn part_two() -> usize {
    let _contents = get_contents("src/days/day_18/input.txt");
    0
}

fn calculate_magnitude(contents: String) -> u32 {
    0
}

fn calculate_final_sum(contents: String) -> String {
    contents
}

trait SnailFishNumber {
    fn is_valid(&self) -> bool;
    fn add(self, other: Self) -> Self;
    fn is_reduced(&self) -> bool;
    fn reduce(&mut self);
    fn needs_explode(&self) -> bool;
    fn explode(&mut self);
    fn needs_split(&self) -> bool;
    fn split(&mut self);
}

impl SnailFishNumber for String {
    fn is_valid(&self) -> bool {
        let mut open_brackets = 0;
        for c in self.chars() {
            match c {
                '[' => open_brackets += 1,
                ']' => {
                    if open_brackets == 0 {
                        return false;
                    } else {
                        open_brackets += 1;
                    }
                }
                _ => {}
            }
        }
        true
    }

    fn add(self, other: Self) -> Self {
        let mut new = String::new();
        new.push('[');
        new.push_str(self.as_str());
        new.push_str(other.as_str());
        new.push(']');
        new.reduce();
        new
    }

    fn is_reduced(&self) -> bool {
        !self.needs_split() && !self.needs_explode()
    }

    fn reduce(&mut self) {
        while !self.is_reduced() {
            self.explode();
            self.split();
        }
    }

    // If any pair is nested inside four pairs, the leftmost such pair explodes.
    fn needs_explode(&self) -> bool {
        let mut cur_depth = 0;
        for c in self.chars() {
            match c {
                '[' => {
                    cur_depth += 1;
                    if cur_depth > 4 {
                        return true;
                    };
                }
                ']' => {
                    if cur_depth == 0 {
                        panic!("More closing than opening brackets!");
                    } else {
                        cur_depth -= 1;
                    }
                }
                _ => {}
            }
        }
        false
    }

    fn explode(&mut self) {}

    // If any regular number is 10 or greater, the leftmost such regular number splits.
    fn needs_split(&self) -> bool {
        let mut buffer = Vec::<char>::new();
        for c in self.chars() {
            match c {
                '[' => {}
                ']' | ',' => {
                    if buffer.is_empty() {
                        continue;
                    }
                    if buffer
                        .iter()
                        .collect::<String>()
                        .as_str()
                        .parse::<u8>()
                        .unwrap()
                        >= 10
                    {
                        return true;
                    } else {
                        buffer.clear();
                    }
                }
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    buffer.push(c);
                }
                _ => panic!("Found invalid character: {}", c),
            }
        }
        false
    }

    fn split(&mut self) {}
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_magnitude_example_1() {
        let contents = get_contents("src/days/day_18/example.txt");
        assert_eq!(calculate_magnitude(contents), 4140);
    }

    #[test]
    fn test_final_sum_example_1() {
        let contents = get_contents("src/days/day_18/example.txt");
        let result = "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]";
        assert_eq!(calculate_final_sum(contents), result);
    }

    #[test]
    fn test_needs_explode() {
        let need_not_explode =
            String::from("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]");
        assert_eq!(need_not_explode.needs_explode(), false);
        let needs_to_explode =
            String::from("[[[[[3,1],6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]");
        assert_eq!(needs_to_explode.needs_explode(), true);
    }

    #[test]
    fn test_needs_split() {
        let need_not_split = String::from("[[5,[2,1]],[[5,[5,6]],[2,2]]]");
        assert_eq!(need_not_split.needs_split(), false);
        let needs_to_split = String::from("[[5,[2,1]],[[5,[5,12]],[2,2]]]");
        assert_eq!(needs_to_split.needs_split(), true);
    }
}
