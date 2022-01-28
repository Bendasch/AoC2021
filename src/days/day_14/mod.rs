use super::get_contents;
use std::collections::HashMap;

pub fn main() {
    println!("Day-14 part 1: {}", part_one());
    println!("Day-14 part 2: {}", part_two());
}

fn part_one() -> u64 {
    let contents = get_contents("src/days/day_14/input.txt");
    let mut polymer = Polymer::new(contents);
    polymer.iterate(10);
    let counts = polymer.get_char_counts();
    counts.values().max().unwrap() - counts.values().min().unwrap()
}

fn part_two() -> u64 {
    let contents = get_contents("src/days/day_14/input.txt");
    let mut polymer = Polymer::new(contents);
    polymer.iterate(40);
    let counts = polymer.get_char_counts();
    counts.values().max().unwrap() - counts.values().min().unwrap()
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Key(char, char);

#[derive(Debug)]
struct Polymer {
    rules: HashMap<Key, char>,
    counts: HashMap<Key, u64>,
    first_char: char,
    last_char: char,
}

impl Polymer {
    fn new(contents: String) -> Polymer {
        let mut lines = contents.lines();
        let template = lines.next().unwrap().chars().collect::<Vec<char>>();
        lines.next(); // skip empty line separting template and rules

        let mut rules = HashMap::<Key, char>::new();
        for line in lines {
            let mut rule_parts = line.split("->");
            let mut location = rule_parts.next().unwrap().trim().chars();
            let key = Key(location.next().unwrap(), location.next().unwrap());
            let value = rule_parts.next().unwrap().trim().chars().next().unwrap();
            rules.insert(key, value);
        }

        let mut counts = HashMap::<Key, u64>::new();
        for k in 1..template.len() {
            let key = Key(template[k - 1], template[k]);
            if let Some(count) = counts.get_mut(&key) {
                *count += 1;
            } else {
                counts.insert(key, 1);
            }
        }

        Polymer {
            rules,
            counts,
            first_char: template[0],
            last_char: template[template.len() - 1],
        }
    }

    fn iterate(&mut self, iter: usize) {
        for _ in 0..iter {
            let keys: HashMap<_, _> = self.counts.clone();
            for (key, old_count) in keys.iter().filter(|(_, &c)| c > 0) {
                let rule = self.rules.get(key);
                if rule.is_none() {
                    continue;
                } else {
                    let count = self.counts.get_mut(key).unwrap();
                    *count -= old_count;
                }

                let left_key = Key(key.0, *rule.unwrap());
                if let Some(count) = self.counts.get_mut(&left_key) {
                    *count += old_count;
                } else {
                    self.counts.insert(left_key, *old_count);
                }

                let right_key = Key(*rule.unwrap(), key.1);
                if let Some(count) = self.counts.get_mut(&right_key) {
                    *count += old_count;
                } else {
                    self.counts.insert(right_key, *old_count);
                }
            }
        }
    }

    fn get_char_counts(&self) -> HashMap<char, u64> {
        let mut char_counts = HashMap::<char, u64>::new();
        for (key, count) in self.counts.iter() {
            let left_char = key.0;
            let right_char = key.1;

            if let Some(left_count) = char_counts.get_mut(&left_char) {
                *left_count += *count;
            } else {
                char_counts.insert(left_char, *count);
            }

            if let Some(right_count) = char_counts.get_mut(&right_char) {
                *right_count += *count;
            } else {
                char_counts.insert(right_char, *count);
            }
        }

        *char_counts.get_mut(&self.first_char).unwrap() += 1;
        *char_counts.get_mut(&self.last_char).unwrap() += 1;

        for (_, count) in char_counts.iter_mut() {
            *count /= 2;
        }

        char_counts
    }
}
