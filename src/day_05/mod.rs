use crate::get_contents;

use std::fmt::{Display, Error, Formatter};

pub fn main() {
    println!("Day-05 part 1: {}", part_one());
    println!("Day-05 part 2: {}", part_two());
}

fn part_one() -> u32 {
    let contents = get_contents("src/day_05/input.txt");
    let lines = parse_input(&contents);
    let vent_map = build_vent_map(lines, false);
    //println!("{}", vent_map);
    vent_map.val.iter().filter(|&v| *v > 1).count() as u32
}

fn part_two() -> u32 {
    let contents = get_contents("src/day_05/input.txt");
    let lines = parse_input(&contents);
    let vent_map = build_vent_map(lines, true);
    //println!("{}", vent_map);
    vent_map.val.iter().filter(|&v| *v > 1).count() as u32
}

#[derive(Debug)]
struct Line((u32, u32), (u32, u32));
fn parse_input(contents: &str) -> Vec<Line> {
    let mut lines = Vec::new();
    for line in contents.lines() {
        let mut points = line.split("->");
        let mut point1 = points
            .next()
            .unwrap()
            .trim()
            .split(',')
            .map(|x| x.parse::<u32>().unwrap());
        let mut point2 = points
            .next()
            .unwrap()
            .trim()
            .split(',')
            .map(|x| x.parse::<u32>().unwrap());
        lines.push(Line(
            (point1.next().unwrap(), point1.next().unwrap()),
            (point2.next().unwrap(), point2.next().unwrap()),
        ));
    }
    lines
}

fn build_vent_map(lines: Vec<Line>, diag: bool) -> Matrix {
    let max_coords = lines
        .iter()
        .map(|line| line.0)
        .chain(lines.iter().map(|line| line.1))
        .fold((0, 0), |acc, (x, y)| (x.max(acc.0), y.max(acc.1)));
    let mut vent_map = Matrix::new(max_coords.0 as usize + 1, max_coords.1 as usize + 1);
    for line in lines {
        vent_map.add_line(&line);
        if diag {
            vent_map.add_diag(&line);
        }
    }
    vent_map
}

#[derive(Debug)]
struct Matrix {
    val: Vec<u32>,
    ncol: usize,
}
impl Matrix {
    fn new(c: usize, r: usize) -> Matrix {
        Matrix {
            val: vec![0; r * c],
            ncol: c,
        }
    }

    fn rows(&self) -> usize {
        self.val.len() / self.cols()
    }

    fn cols(&self) -> usize {
        self.ncol
    }

    fn add_line(&mut self, line: &Line) {
        let x1 = line.0 .0.min(line.1 .0);
        let x2 = line.0 .0.max(line.1 .0);
        let y1 = line.0 .1.min(line.1 .1);
        let y2 = line.0 .1.max(line.1 .1);
        for x in x1..=x2 {
            for y in y1..=y2 {
                if x1 == x2 || y1 == y2 {
                    self.add_one(x as usize, y as usize);
                }
            }
        }
    }

    fn add_diag(&mut self, line: &Line) {
        let x1 = line.0 .0 as i32;
        let x2 = line.1 .0 as i32;
        let y1 = line.0 .1 as i32;
        let y2 = line.1 .1 as i32;
        let x_diff = x2 - x1;
        let y_diff = y2 - y1;

        if x_diff.abs() != y_diff.abs() {
            return;
        }

        for x in x1.min(x2)..=x1.max(x2) {
            for y in y1.min(y2)..=y1.max(y2) {
                if x_diff == y_diff && x - y == x1 - y1 {
                    self.add_one(x as usize, y as usize);
                }
                if x_diff != y_diff && x - x1 == y1 - y {
                    self.add_one(x as usize, y as usize);
                }
            }
        }
    }

    fn add_one(&mut self, c: usize, r: usize) {
        let rows = self.rows();
        let cols = self.cols();
        if r > rows || c > cols {
            panic!("Out of bounds");
        }
        // the matrix is stored in column major order.
        self.val[c * rows + r] += 1;
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for c in 0..self.cols() {
            for r in 0..self.rows() {
                write!(f, "{}, ", self.val[r * self.cols() + c])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
