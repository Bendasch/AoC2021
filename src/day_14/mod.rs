use crate::get_contents;

use std::collections::HashMap;

pub fn main() {
    println!("Day-14 part 1: {}", part_one());
    println!("Day-14 part 2: {}", part_two());
}

fn part_one() -> u64 {
    let contents = get_contents("src/day_14/input.txt");
    let mut polymer = Polymer::new(contents);
    polymer.iterate(10);
    let max_count = polymer.counts.iter().max().unwrap();
    let min_count = polymer.counts.iter().filter(|&v| *v > 0).min().unwrap();
    max_count - min_count
}

fn part_two() -> u64 {
    let contents = get_contents("src/day_14/input.txt");
    let mut polymer = Polymer::new(contents);
    polymer.iterate(40);
    let max_count = polymer.counts.iter().max().unwrap();
    let min_count = polymer.counts.iter().min().unwrap();
    max_count - min_count
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Key(char, char);

#[derive(Clone, Debug, PartialEq, Eq)]
struct Rules(HashMap<Key, char>);

#[derive(Debug)]
struct Polymer {
    template: Vec<char>,
    rules: Rules,
    counts: Vec<u64>,
}

impl Polymer {
    fn new(contents: String) -> Polymer {
        let mut lines = contents.lines();
        let template = lines.next().unwrap().chars().collect::<Vec<char>>();
        lines.next(); // skip empty line separting template and rules

        let mut rules: Rules = Rules(HashMap::new());
        for line in lines {
            let mut rule_parts = line.split("->");
            let mut location = rule_parts.next().unwrap().trim().chars();
            let key = Key(location.next().unwrap(), location.next().unwrap());
            let value = rule_parts.next().unwrap().trim().chars().next().unwrap();
            rules.0.insert(key, value);
        }

        let mut counts = vec![0; 10];
        for ch in template.iter() {
            add_count(&mut counts, *ch);
        }

        Polymer {
            template,
            rules,
            counts,
        }
    }

    fn iterate(&mut self, iter: usize) {
        let mut parents = Key(' ', ' ');
        for k in 1..self.template.len() {
            parents.0 = self.template[k - 1];
            parents.1 = self.template[k];
            println!("Calculating {}{}...", parents.0, parents.1);
            self.calc_offspring(&parents, &iter);
        }
    }

    fn calc_offspring(&mut self, parents: &Key, iter: &usize) {
        match self.add_child(parents) {
            Some(child) if *iter > 1 => self.calc_offspring_recursive(&child, parents, &(iter - 1)),
            _ => {}
        }
    }

    fn calc_offspring_recursive(&mut self, child: &char, parents: &Key, iter: &usize) {
        let left_parents = Key(parents.0, *child);
        match self.add_child(&left_parents) {
            Some(left_child) if *iter > 1 => {
                self.calc_offspring_recursive(&left_child, &left_parents, &(iter - 1))
            }
            _ => {}
        }

        let right_parents = Key(*child, parents.1);
        match self.add_child(&right_parents) {
            Some(right_child) if *iter > 1 => {
                self.calc_offspring_recursive(&right_child, &right_parents, &(iter - 1))
            }
            _ => {}
        }
    }

    fn add_child(&mut self, parents: &Key) -> Option<char> {
        let rule = self.rules.0.get(parents);
        if rule.is_none() {
            return None;
        }
        let child = rule.unwrap();
        add_count(&mut self.counts, *child);
        return Some(*child);
    }
}

fn add_count(counts: &mut Vec<u64>, ch: char) {
    match ch {
        'B' => counts[0] += 1,
        'C' => counts[1] += 1,
        'F' => counts[2] += 1,
        'H' => counts[3] += 1,
        'K' => counts[4] += 1,
        'N' => counts[5] += 1,
        'O' => counts[6] += 1,
        'P' => counts[7] += 1,
        'S' => counts[8] += 1,
        'V' => counts[9] += 1,
        _ => panic!("Unknown char {}", ch),
    }
}
