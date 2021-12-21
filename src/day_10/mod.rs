use crate::get_contents;

use std::collections::HashMap;
use std::collections::LinkedList;

pub fn main() {
    let (part_one_total, incomplete_lines) = part_one();
    println!("Day-10 part 1: {}", part_one_total);
    println!("Day-10 part 2: {}", part_two(incomplete_lines));
}

fn part_one() -> (i32, Vec<String>) {
    let contents = get_contents("src/day_10/input.txt");
    let bracket_map = get_bracket_map();
    let mut open_brackets = LinkedList::new();
    let mut total = 0;
    let mut incomplete_lines = Vec::new(); // for part 2
    for line in contents.lines() {
        let mut last_bracket = ' ';
        for char in line.chars() {
            match char {
                '(' | '[' | '{' | '<' => open_brackets.push_back(char),
                ')' | ']' | '}' | '>' => {
                    let last_opening_bracket = open_brackets.pop_back();
                    if last_opening_bracket.is_none() {
                        last_bracket = char;
                        break;
                    }
                    let expected_bracket =
                        *bracket_map.get(&last_opening_bracket.unwrap()).unwrap();
                    if expected_bracket != char {
                        last_bracket = char;
                        break;
                    }
                }
                _ => panic!("Unexpected character: {}", char),
            }
        }
        match last_bracket {
            ')' => total += 3,
            ']' => total += 57,
            '}' => total += 1197,
            '>' => total += 25137,
            _ => incomplete_lines.push(line.to_string()),
        }
    }
    (total, incomplete_lines)
}

fn part_two(incomplete_lines: Vec<String>) -> u64 {
    let mut totals = Vec::new();
    for line in incomplete_lines {
        let mut open_brackets = LinkedList::new();
        for char in line.chars() {
            match char {
                '(' | '[' | '{' | '<' => open_brackets.push_back(char),
                ')' | ']' | '}' | '>' => {
                    // here we assume, no lines are corrupt!
                    open_brackets.pop_back();
                }
                _ => panic!("Unexpected character: {}", char),
            }
        }

        let mut line_total: u64 = 0;
        for char in open_brackets.into_iter().rev() {
            line_total *= 5;
            match char {
                '(' => line_total += 1,
                '[' => line_total += 2,
                '{' => line_total += 3,
                '<' => line_total += 4,
                _ => continue,
            }
        }
        totals.push(line_total);
    }
    totals.sort_unstable();
    totals[totals.len() / 2]
}

fn get_bracket_map() -> HashMap<char, char> {
    let mut brackets = HashMap::new();
    brackets.insert('(', ')');
    brackets.insert('[', ']');
    brackets.insert('{', '}');
    brackets.insert('<', '>');
    brackets
}
