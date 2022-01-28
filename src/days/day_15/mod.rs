use super::get_contents;
use std::fmt::{Display, Error, Formatter};

pub fn main() {
    println!("Day-15 part 1: {}", part_one());
    println!("Day-15 part 2: {}", part_two());
}

fn part_one() -> u64 {
    let contents = get_contents("src/days/day_15/input.txt");
    let mut map = Map::from_string(contents, false);
    map.calculate_path();
    map.get(map.rows() - 1, map.cols() - 1).path
}

fn part_two() -> u64 {
    let contents = get_contents("src/days/day_15/input.txt");
    let mut map = Map::from_string(contents, true);
    map.calculate_path();
    map.get(map.rows() - 1, map.cols() - 1).path
}

#[derive(Debug, Clone)]
struct Node {
    cost: u32,
    path: u64,
    visited: bool,
}

#[derive(Debug)]
struct Map {
    nodes: Vec<Node>,
    cols: usize,
    cur_row: usize,
    cur_col: usize,
}

impl Map {
    fn new(nodes: Vec<Node>, cols: usize, curr: (usize, usize)) -> Map {
        Map {
            nodes,
            cols,
            cur_row: curr.0,
            cur_col: curr.1,
        }
    }

    fn from_string(input: String, repeats: bool) -> Map {
        let mut nodes = Vec::<Node>::new();
        let mut rows = 0;
        let loops: usize = if repeats { 5 } else { 1 };
        for a in 0..loops {
            for (l, line) in input.lines().enumerate() {
                for b in 0..loops {
                    for (c, char) in line.chars().enumerate() {
                        let path = if a == 0 && l == 0 && b == 0 && c == 0 {
                            0
                        } else {
                            u64::MAX
                        };
                        let mut cost = char.to_digit(10).unwrap() + (a + b) as u32;
                        if cost > 9 {
                            cost -= 9;
                        };
                        nodes.push(Node {
                            cost,
                            path,
                            visited: false,
                        });
                    }
                }
                rows += 1;
            }
        }
        let cols = nodes.len() / rows;
        Map::new(nodes, cols, (0, 0))
    }

    fn rows(&self) -> usize {
        self.nodes.len() / self.cols
    }

    fn cols(&self) -> usize {
        self.cols
    }

    fn get(&self, row: usize, col: usize) -> &Node {
        &self.nodes[row * self.cols + col]
    }

    fn get_mut(&mut self, row: usize, col: usize) -> &mut Node {
        &mut self.nodes[row * self.cols + col]
    }

    fn calculate_path(&mut self) {
        loop {
            let mut paths = Vec::new();

            // update possible paths
            if self.cur_row > 0 {
                paths.push((self.update_path((-1, 0)), (-1, 0)));
            }
            if self.cur_row < self.nodes.len() / self.cols - 1 {
                paths.push((self.update_path((1, 0)), (1, 0)));
            }

            if self.cur_col > 0 {
                paths.push((self.update_path((0, -1)), (0, -1)));
            }

            if self.cur_col < self.cols - 1 {
                paths.push((self.update_path((0, 1)), (0, 1)));
            }

            // mark the current node as visited
            self.get_mut(self.cur_row, self.cur_col).visited = true;

            // if the end node is visited, we're done
            if self.get(self.rows() - 1, self.cols() - 1).visited {
                break;
            }

            // continue with the node with the lowest tentative total cost
            let min_node = self
                .nodes
                .iter()
                .enumerate()
                .filter(|(_, n)| !n.visited)
                .min_by(|(_, a), (_, b)| a.path.cmp(&b.path))
                .unwrap();
            if min_node.1.cost == u32::MAX {
                println!("No path to be found...");
                break;
            }
            self.cur_row = (min_node.0 / self.rows()) as usize;
            self.cur_col = (min_node.0 % self.rows()) as usize;
        }
    }

    fn update_path(&mut self, offset: (isize, isize)) -> u64 {
        let current = self.get(self.cur_row, self.cur_col).clone();
        let mut node = self.get_mut(
            (self.cur_row as isize + offset.0) as usize,
            (self.cur_col as isize + offset.1) as usize,
        );
        if !node.visited {
            let path = current.path + node.cost as u64;
            if path < node.path {
                node.path = path;
            }
            node.path
        } else {
            u64::MAX
        }
    }
}

#[rustfmt::skip]
impl Display for Map {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for row in 0..self.rows() {
            for col in 0..self.cols() {
                let node = self.get(row, col);
                let path = if node.path == u64::MAX {
                    format!("{:>width$}", "inf".to_string(), width = 4)
                } else {
                    format!("{:>width$}", node.path, width = 4)
                };
                write!(f, "{}", path)?;
            }
            writeln!(f)?;
        }
        writeln!(f)?;
        for row in 0..self.rows() {
            for col in 0..self.cols() {
                let node = self.get(row, col);
                let cost = format!("{:>width$}", node.cost, width = 3);
                write!(f, "{}", cost)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
