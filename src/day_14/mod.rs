use crate::get_contents;

pub fn main() {
    println!("Day-14 part 1: {}", part_one());
    println!("Day-14 part 2: {}", part_two());
}

fn part_one() -> usize {
    let contents = get_contents("src/day_14/input.txt");
    let (mut polymer, rules) = parse_contents(contents);
    for _ in 0..10 {
        polymer.perform_insertions(&rules);
    }
    polymer.get_highest_count() - polymer.get_lowest_count()
}

fn part_two() -> usize {
    let contents = get_contents("src/day_14/example.txt");
    let (mut polymer, rules) = parse_contents(contents);
    for k in 0..40 {
        println!("Performing insertion: {}", k + 1);
        polymer.perform_insertions(&rules);
    }
    polymer.get_highest_count() - polymer.get_lowest_count()
}

fn parse_contents(contents: String) -> (Polymer, Vec<InsertionRule>) {
    let mut lines = contents.lines();
    let template = Polymer(lines.next().unwrap().to_string());
    lines.next(); // skip empty line separting template and rules

    let mut rules: Vec<InsertionRule> = Vec::new();
    for line in lines {
        let mut rule_parts = line.split("->");
        let mut location = rule_parts.next().unwrap().trim().chars();
        let mut insertion = rule_parts.next().unwrap().trim().chars();
        rules.push(InsertionRule(
            location.next().unwrap(),
            location.next().unwrap(),
            insertion.next().unwrap(),
        ));
    }
    rules.sort();
    (template, rules)
}

#[derive(Debug, PartialEq, Eq, Ord)]
struct InsertionRule(char, char, char);

impl PartialOrd for InsertionRule {

}

#[derive(Debug)]
struct Polymer(String);

impl Polymer {
    fn perform_insertions(&mut self, rules: &[InsertionRule]) {
        let old_polymer = self.0.clone().chars().collect::<Vec<char>>();
        let mut new_polymer: Vec<char> = Vec::new();
        new_polymer.push(old_polymer[0]);
        for (k, &char) in old_polymer.iter().enumerate() {
            if k == 0 {
                continue;
            }
            let matching_rule = rules.iter().find(|rule| {
                rule.0 == old_polymer[k-1] && rule.1 == old_polymer[k]
            });
            if matching_rule.is_none() {
                continue;
            }
            let matching_rule = matching_rule.unwrap();
            new_polymer.push(matching_rule.2);
            new_polymer.push(char);
        }
        self.0 = new_polymer.iter().collect::<String>();
    }

    fn get_highest_count(&self) -> usize {
        let mut chars = self.0.chars().collect::<Vec<char>>();
        chars.sort();
        let mut previous_char = ' ';
        let mut max_count = 0;
        let mut current_count = 0;
        for char in chars.into_iter() {
            if char == previous_char {
                current_count += 1;
                continue;
            }
            max_count = max_count.max(current_count);
            current_count = 1;
            previous_char = char;
        }
        max_count
    }

    fn get_lowest_count(&self) -> usize {
        let mut chars = self.0.chars().collect::<Vec<char>>();
        chars.sort();
        let mut previous_char = ' ';
        let mut min_count = 0;
        let mut current_count = 0;
        for char in chars.into_iter() {
            if char == previous_char {
                current_count += 1;
                continue;
            }
            if min_count == 0 {
                min_count = current_count;
            } else {
                min_count = min_count.min(current_count);
            }
            current_count = 1;
            previous_char = char;
        }
        min_count
    }
}
