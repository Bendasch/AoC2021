use crate::get_contents;

use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};

pub fn main() {
    println!("Day-12 part 1: {}", part_one());
    println!("Day-12 part 2: {}", part_two());
}

fn part_one() -> usize {
    let contents = get_contents("src/day_12/input.txt");
    let connections = collect_connections(contents);
    let paths: Vec<Path> = find_paths(&connections);
    for path in paths.iter() {
        println!("{}", path);
    }
    paths.len()
}

fn part_two() -> u32 {
    //let contents = get_contents("src/day_12/input.txt");
    0
}

fn find_paths(connections: &HashMap<Cave, Vec<Cave>>) -> Vec<Path> {
    let start = Cave::new("start".to_string());
    let path = vec![start.clone()];
    let mut paths: Vec<Path> = Vec::new();
    let paths_from_start = connections.get(&start).unwrap();
    for cave in paths_from_start.iter() {
        let mut new_path = path.clone();
        new_path.push(cave.clone());
        paths.append(&mut continue_path_from_cave(cave, &new_path, connections));
    }
    paths
}

fn continue_path_from_cave(
    cave: &Cave,
    path: &Vec<Cave>,
    connections: &HashMap<Cave, Vec<Cave>>,
) -> Vec<Path> {
    let mut paths: Vec<Path> = Vec::new();
    let next_caves = connections.get(cave).unwrap();
    for next_cave in next_caves.iter() {
        if next_cave.size == Size::Small && path.contains(next_cave) {
            continue;
        }
        let mut new_path = path.clone();
        new_path.push(next_cave.clone());
        if next_cave.id == "end" {
            paths.push(Path::new(new_path));
        } else {
            paths.append(&mut continue_path_from_cave(
                next_cave,
                &new_path,
                connections,
            ));
        }
    }
    paths
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Size {
    Small,
    Big,
}

#[derive(Debug, Hash, Clone)]
struct Cave {
    id: String,
    size: Size,
}

impl PartialEq for Cave {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.size == other.size
    }
}

impl Eq for Cave {}

impl Cave {
    fn new(id: String) -> Cave {
        let size = if id.chars().next().unwrap().is_uppercase() {
            Size::Big
        } else {
            Size::Small
        };
        Cave {
            id: id.to_lowercase(),
            size,
        }
    }
}

fn collect_connections(contents: String) -> HashMap<Cave, Vec<Cave>> {
    let mut connections = HashMap::<Cave, Vec<Cave>>::new();
    for line in contents.lines() {
        let mut parts = line.split('-');
        let from = Cave::new(parts.next().unwrap().to_string());
        let to = Cave::new(parts.next().unwrap().to_string());
        insert_connection(&from, &to, &mut connections);
        insert_connection(&to, &from, &mut connections);
    }
    connections
}

fn insert_connection(from: &Cave, to: &Cave, connections: &mut HashMap<Cave, Vec<Cave>>) {
    let mut vec: Vec<Cave> = match connections.get(&from) {
        Some(vector) => vector.to_vec(),
        None => Vec::<Cave>::new(),
    };
    vec.push(to.clone());
    connections.insert(from.clone(), vec);
}

struct Path {
    _path: String,
}

impl Path {
    fn new(caves: Vec<Cave>) -> Path {
        let mut path = String::new();
        for (k, cave) in caves.iter().enumerate() {
            if k > 0 {
                path.push_str(",");
            }
            match cave.size {
                Size::Big => path.push_str(&cave.id.to_uppercase()),
                Size::Small => path.push_str(&cave.id),
            }
        }
        Path { _path: path }
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let caves: Vec<&str> = self._path.split(',').collect();
        for (k, cave) in caves.iter().enumerate() {
            if k > 0 {
                write!(f, "->")?;
            }
            write!(f, "{}", cave)?;
        }
        Ok(())
    }
}
