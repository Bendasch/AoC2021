use super::get_contents;
use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};

pub fn main() {
    println!("Day-12 part 1: {}", part_one());
    println!("Day-12 part 2: {}", part_two());
}

fn part_one() -> usize {
    let contents = get_contents("src/days/day_12/input.txt");
    let connections = collect_connections(contents);
    let paths: Vec<Path> = find_paths(&connections, false);
    paths.len()
}

fn part_two() -> usize {
    let contents = get_contents("src/days/day_12/input.txt");
    let connections = collect_connections(contents);
    let paths: Vec<Path> = find_paths(&connections, true);
    paths.len()
}

fn find_paths(connections: &HashMap<Cave, Vec<Cave>>, twice: bool) -> Vec<Path> {
    let start = Cave::new("start".to_string());
    let mut paths: Vec<Path> = Vec::new();
    let paths_from_start = connections.get(&start).unwrap();
    for cave in paths_from_start.iter() {
        let path = vec![start.clone(), cave.clone()];
        paths.append(&mut continue_path_from_cave(cave, path, connections, twice));
    }
    paths
}

fn continue_path_from_cave(
    cave: &Cave,
    path: Vec<Cave>,
    connections: &HashMap<Cave, Vec<Cave>>,
    twice: bool,
) -> Vec<Path> {
    let mut paths: Vec<Path> = Vec::new();
    let next_caves = connections.get(cave).unwrap();
    for next_cave in next_caves.iter() {
        if next_cave.size == Size::Small
            && path.contains(next_cave)
            && (!twice || next_cave.id == "start" || Cave::path_has_duplicate(&path))
        {
            continue;
        }
        let mut new_path = path.clone().to_vec();
        new_path.push(next_cave.clone());
        if next_cave.id == "end" {
            paths.push(Path::new(new_path));
        } else {
            paths.append(&mut continue_path_from_cave(
                next_cave,
                new_path,
                connections,
                twice,
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

#[derive(Debug, Hash, PartialEq, Clone)]
struct Cave {
    id: String,
    size: Size,
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

    fn path_has_duplicate(path: &[Cave]) -> bool {
        let mut path_ids: Vec<String> = Vec::new();
        for cave in path.iter().filter(|c| c.size == Size::Small) {
            if path_ids.contains(&cave.id) {
                return true;
            }
            path_ids.push(cave.id.clone());
        }
        false
    }
}

fn collect_connections(contents: String) -> HashMap<Cave, Vec<Cave>> {
    let mut connections = HashMap::<Cave, Vec<Cave>>::new();
    for line in contents.lines() {
        let mut parts = line.split('-');
        let a = Cave::new(parts.next().unwrap().to_string());
        let b = Cave::new(parts.next().unwrap().to_string());
        insert_connection(&a, &b, &mut connections);
        insert_connection(&b, &a, &mut connections);
    }
    connections
}

fn insert_connection(key: &Cave, value: &Cave, connections: &mut HashMap<Cave, Vec<Cave>>) {
    let mut values: Vec<Cave> = match connections.get(key) {
        Some(vector) => vector.to_vec(),
        None => Vec::<Cave>::new(),
    };
    values.push(value.clone());
    connections.insert(key.clone(), values);
}

struct Path {
    _path: String,
}

impl Path {
    fn new(caves: Vec<Cave>) -> Path {
        let mut path = String::new();
        for (k, cave) in caves.iter().enumerate() {
            if k > 0 {
                path.push(',');
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
        write!(f, "{}", self._path)?;
        Ok(())
    }
}
