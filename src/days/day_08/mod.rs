use super::get_contents;
use std::collections::HashMap;

pub fn main() {
    println!("Day-08 part 1: {}", part_one());
    println!("Day-08 part 2: {}", part_two());
}

fn part_one() -> i32 {
    let contents = get_contents("src/days/day_08/input.txt");
    let mut single_digit_count = 0;
    for line in contents.lines() {
        let output = line.split('|').next_back().unwrap();
        for digit in output.split_whitespace() {
            match digit.len() {
                2 | 3 | 4 | 7 => single_digit_count += 1,
                _ => continue,
            }
        }
    }
    single_digit_count
}

fn part_two() -> i32 {
    let contents = get_contents("src/days/day_08/input.txt");
    let mut total_sum: i32 = 0;
    for line in contents.lines() {
        let mut split = line.split('|');
        let patterns = split.next().unwrap();
        let pattern_map = get_pattern_map(patterns);
        let output = split.next().unwrap();
        let mut result = 0;
        for (k, digit) in output.split_whitespace().enumerate() {
            result +=
                10_i32.pow(3 - k as u32) * pattern_map.get(&get_key_from_pattern(digit)).unwrap();
        }
        total_sum += result;
    }
    total_sum
}

fn get_pattern_map(patterns: &str) -> HashMap<String, i32> {
    let mut pattern_map: HashMap<String, i32> = HashMap::new();
    let patterns: Vec<&str> = patterns.split_whitespace().collect();
    let translation_map = get_translation_map(&patterns);
    for pattern in patterns.into_iter() {
        pattern_map.insert(
            get_key_from_pattern(pattern),
            translate_pattern(pattern, &translation_map),
        );
    }
    pattern_map
}

fn get_key_from_pattern(pattern: &str) -> String {
    let mut key_vec: Vec<char> = pattern.chars().collect::<Vec<char>>();
    key_vec.sort_unstable();
    key_vec.iter().collect::<String>()
}

#[allow(clippy::all)]
fn get_translation_map(patterns: &[&str]) -> HashMap<char, char> {
    let mut translation_map: HashMap<char, char> = HashMap::new();
    let one = get_unique_pattern(1, patterns);
    let four = get_unique_pattern(4, patterns);
    let seven = get_unique_pattern(7, patterns);
    let eight = get_unique_pattern(8, patterns);
    let a = find_diff_char(&seven, &one);
    let six_digit_patterns = patterns
        .to_owned()
        .into_iter()
        .filter(|&x| x.chars().count() == 6)
        .collect::<Vec<&str>>();
    let (nine, g) = find_nine_and_g(&four, &a, &six_digit_patterns);
    let e = find_diff_char(&eight, &nine);
    let five_digit_patterns = patterns
        .to_owned()
        .into_iter()
        .filter(|&x| x.chars().count() == 5)
        .collect::<Vec<&str>>();
    let (five, six) = find_five_and_six(&five_digit_patterns, &six_digit_patterns, &e);
    let c = find_diff_char(&nine, &five);
    let f = find_diff_char(&one, c.to_string().as_str());
    let zero = find_zero(&six_digit_patterns, &six.as_str(), &nine.as_str());
    let d = find_diff_char(&eight, &zero);
    let b = find_diff_char(&four, vec![c, d, f].iter().collect::<String>().as_str());
    translation_map.insert(a, 'a');
    translation_map.insert(b, 'b');
    translation_map.insert(c, 'c');
    translation_map.insert(d, 'd');
    translation_map.insert(e, 'e');
    translation_map.insert(f, 'f');
    translation_map.insert(g, 'g');
    translation_map
}

fn get_unique_pattern(number: usize, patterns: &[&str]) -> String {
    let pattern_length = match number {
        1 => 2,
        7 => 3,
        4 => 4,
        8 => 7,
        _ => panic!("Pattern for number {} is not unique", number),
    };
    patterns
        .iter()
        .find(|&p| p.chars().count() == pattern_length)
        .unwrap()
        .to_string()
}

#[rustfmt::skip]
fn translate_pattern(pattern: &str, translation_map: &HashMap<char, char>) -> i32 {
    let mut translated_pattern: Vec<char> = Vec::new();
    for c in pattern.chars() {
        translated_pattern.push(*translation_map.get(&c).unwrap());
    }
    translated_pattern.sort_unstable();
    
    let pattern_string = translated_pattern.into_iter().collect::<String>();
    match pattern_string.as_str() {
        "abcefg"    => 0_i32,
        "cf"        => 1_i32,
        "acdeg"     => 2_i32,
        "acdfg"     => 3_i32,
        "bcdf"      => 4_i32,
        "abdfg"     => 5_i32,
        "abdefg"    => 6_i32,
        "acf"       => 7_i32,
        "abcdefg"   => 8_i32,
        "abcdfg"    => 9_i32,
        _ => panic!("Invalid pattern: {}", pattern_string),
    }
}

fn find_diff_char(a: &str, b: &str) -> char {
    for c1 in a.chars() {
        if !b.chars().any(|c| c == c1) {
            return c1;
        }
    }
    panic!("No diff char found");
}

fn find_nine_and_g(four: &str, a: &char, six_digit_patterns: &[&str]) -> (String, char) {
    let mut pattern = four.chars().collect::<Vec<char>>();
    pattern.push(*a);
    for six_digit_pattern in six_digit_patterns {
        let mut difference_counter: u32 = 0;
        let mut nine_pattern: &str = "";
        let mut diff_char: char = 'Z';
        for c in six_digit_pattern.chars() {
            if !pattern.contains(&c) {
                diff_char = c;
                difference_counter += 1;
                nine_pattern = six_digit_pattern;
            }
        }
        if difference_counter == 1 {
            return (nine_pattern.to_string(), diff_char);
        }
    }
    panic!("No five digit pattern with exactly one differing char found");
}

fn find_five_and_six(
    five_digit_patterns: &[&str],
    six_digit_patterns: &[&str],
    e: &char,
) -> (String, String) {
    for five_digit_pattern in five_digit_patterns {
        let mut match_five_pattern = five_digit_pattern.chars().collect::<Vec<char>>();
        match_five_pattern.push(*e);
        match_five_pattern.sort_unstable();
        for six_digit_pattern in six_digit_patterns {
            let mut match_six_pattern = six_digit_pattern.chars().collect::<Vec<char>>();
            match_six_pattern.sort_unstable();
            if match_five_pattern == match_six_pattern {
                return (
                    five_digit_pattern.to_string(),
                    six_digit_pattern.to_string(),
                );
            }
        }
    }
    panic!("No five and six digit pattern found which differ exactly by 'e'");
}

fn find_zero(six_digit_patterns: &[&str], six: &&str, nine: &&str) -> String {
    for pattern in six_digit_patterns {
        if pattern == six || pattern == nine {
            continue;
        }
        return pattern.to_string();
    }
    panic!(
        "Zero pattern not found (six pattern: {}, nine pattern: {})",
        six, nine
    );
}
