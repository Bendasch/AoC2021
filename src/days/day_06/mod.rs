use super::get_contents;
use std::collections::HashMap;
use std::thread;

pub fn main() {
    println!("Day-06 part 1: {}", part_one());
    println!("Day-06 part 2: {}", part_two());
}

// iterative brute force solution for 80 generations
fn part_one() -> usize {
    let contents = get_contents("src/days/day_06/input.txt");
    let numbers = contents.split(',');
    let mut start_count: usize = 0;
    let mut fishes = vec![9_u8; u32::MAX as usize];
    for (idx, number) in numbers.enumerate() {
        fishes[idx] = number.parse::<u8>().unwrap();
        start_count += 1;
    }
    iterate(80, &mut fishes, start_count)
}

// For 256 generations, the brute force solution falls apart!
// Instead, calculate the solution for each seed recursively in parallel,
// then look-up the answer for each input.
fn part_two() -> usize {
    let mut threads: Vec<thread::JoinHandle<usize>> = vec![];
    for k in 1..=5 {
        let thread = thread::spawn(move || get_children(256, k) + 1);
        threads.push(thread);
    }

    let mut solutions: HashMap<String, usize> = HashMap::with_capacity(5);
    for (k, thread) in threads.into_iter().enumerate() {
        let key = String::from(char::from_digit((k + 1) as u32, 10).unwrap());
        solutions.insert(key, thread.join().unwrap());
    }

    let mut total_count: usize = 0;
    let content = get_contents("src/days/day_06/input.txt");
    for seed in content.split(',') {
        total_count += solutions.get(seed).unwrap();
    }

    total_count
}

fn iterate(days: usize, fishes: &mut [u8], start_count: usize) -> usize {
    let mut count_total = start_count;
    let mut count_added;
    let mut fish: u8;
    for _ in 1..=days {
        count_added = 0;
        for i in 0..count_total {
            fish = fishes[i];
            if fish == 0 {
                fishes[i] = 6;
                fishes[count_total + count_added] = 8;
                count_added += 1;
            } else {
                fishes[i] -= 1;
            };
        }
        count_total += count_added;
    }
    count_total
}

fn get_children(days: usize, seed: usize) -> usize {
    if days < seed {
        return 0;
    }

    let children = ((days - seed) as f32 / 7.).ceil() as usize;
    let mut childrens_children = 0;
    for child in 0..children {
        childrens_children += get_children(days - (seed + 1) - child * 7, 8)
    }
    children + childrens_children
}
