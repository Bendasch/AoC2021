use super::get_contents;
use std::collections::HashMap;

pub fn main() {
    println!("Day-07 part 1: {}", part_one());
    println!("Day-07 part 2: {}", part_two());
}

fn part_one() -> i32 {
    let contents = get_contents("src/days/day_07/input.txt");
    let positions: Vec<i32> = contents
        .split(',')
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();
    let max = *positions.iter().max().unwrap() as usize;
    let min = *positions.iter().min().unwrap() as usize;
    let mut min_distance: i32 = i32::max_value();
    for i in min..=max {
        let cur_distance = positions.iter().map(|&x| (x - i as i32).abs()).sum::<i32>();
        if cur_distance < min_distance {
            min_distance = cur_distance;
        }
    }
    min_distance
}

fn part_two() -> i32 {
    let contents = get_contents("src/days/day_07/input.txt");
    let positions: Vec<i32> = contents
        .split(',')
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();
    let max = *positions.iter().max().unwrap() as usize;
    let min = *positions.iter().min().unwrap() as usize;
    let mut min_distance: i32 = i32::max_value();
    let cost_map = get_cost_map(max - min);
    for i in min..=max {
        let cur_distance = positions
            .iter()
            .map(|&x| cost_map.get(&(x - i as i32).abs()).unwrap())
            .sum::<i32>();
        if cur_distance < min_distance {
            min_distance = cur_distance;
        }
    }
    min_distance
}

fn get_cost_map(max_diff: usize) -> HashMap<i32, i32> {
    let mut cost_map: HashMap<i32, i32> = HashMap::new();
    let mut cost: i32 = 0;
    for i in 0..=max_diff {
        cost += i as i32;
        cost_map.insert(i as i32, cost);
    }
    cost_map
}
