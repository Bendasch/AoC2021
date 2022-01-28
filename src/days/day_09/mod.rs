use super::get_contents;

pub fn main() {
    println!("Day-09 part 1: {}", part_one());
    println!("Day-09 part 2: {}", part_two());
}

fn part_one() -> i32 {
    let contents = get_contents("src/days/day_09/input.txt");
    let heightmap = Matrix::<i32>::new(contents);
    heightmap.calculate_risk()
}

fn part_two() -> i32 {
    let contents = get_contents("src/days/day_09/input.txt");
    let mut heightmap = Matrix::<(i32, Basin)>::new(contents);
    heightmap.assign_basins();
    heightmap.multiply_largest_basins()
}

#[derive(Debug)]
struct Matrix<T> {
    val: Vec<T>,
    ncol: usize,
}

impl<T> Matrix<T>
where
    T: std::fmt::Debug + std::clone::Clone + Heightmap,
{
    fn new(contents: String) -> Matrix<T> {
        let nrow = contents.lines().count();
        let val = contents
            .chars()
            .filter(|&c| !&c.is_ascii_whitespace())
            .map(T::from_char)
            .collect::<Vec<T>>();
        let ncol = val.len() / nrow;
        Matrix { val, ncol }
    }

    fn rows(&self) -> usize {
        self.val.len() / self.cols()
    }

    fn cols(&self) -> usize {
        self.ncol
    }

    fn val(&self, i: usize, j: usize) -> i32 {
        self.val[i * self.cols() + j].val()
    }

    fn element(&self, i: usize, j: usize) -> &T {
        &self.val[i * self.ncol + j]
    }
    fn element_mut(&mut self, i: usize, j: usize) -> &mut T {
        &mut self.val[i * self.ncol + j]
    }

    fn calculate_risk(&self) -> i32 {
        let mut risk = 0;
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                if self.is_low_point(i, j) {
                    risk += self.val(i, j).risk();
                }
            }
        }
        risk
    }

    fn is_low_point(&self, i: usize, j: usize) -> bool {
        let val = self.val(i, j);
        if (i > 0 && self.val(i - 1, j) <= val)
            || (i < self.rows() - 1 && self.val(i + 1, j) <= val)
            || (j > 0 && self.val(i, j - 1) <= val)
            || (j < self.cols() - 1 && self.val(i, j + 1) <= val)
        {
            return false;
        }
        true
    }

    fn assign_basins(&mut self) {
        let mut basin_id: usize = 0;
        let mut checked_fields: usize = 0;
        for k in 0..9 {
            for i in 0..self.rows() {
                for j in 0..self.cols() {
                    if self.element(i, j).unchecked() && self.val(i, j) == k {
                        basin_id += 1;
                        self.assign_basin(i, j, basin_id, &mut checked_fields);
                        self.assign_basins_recursive(i, j, basin_id, &mut checked_fields);
                    }
                }
            }
            if checked_fields == self.val.len() {
                return;
            }
        }
    }

    fn assign_basins_recursive(
        &mut self,
        i: usize,
        j: usize,
        id: usize,
        checked_fields: &mut usize,
    ) {
        if i > 0 && self.element(i - 1, j).unchecked() {
            self.assign_basin(i - 1, j, id, checked_fields);
        }
        if i < self.rows() - 1 && self.element(i + 1, j).unchecked() {
            self.assign_basin(i + 1, j, id, checked_fields);
        }
        if j > 0 && self.element(i, j - 1).unchecked() {
            self.assign_basin(i, j - 1, id, checked_fields);
        }
        if j < self.cols() - 1 && self.element(i, j + 1).unchecked() {
            self.assign_basin(i, j + 1, id, checked_fields);
        }
    }

    fn assign_basin(&mut self, i: usize, j: usize, id: usize, checked_fields: &mut usize) {
        *checked_fields += 1;
        if self.val(i, j) != 9 {
            self.element_mut(i, j).set_basin_number(id);
            self.assign_basins_recursive(i, j, id, checked_fields);
        } else {
            self.element_mut(i, j).no_basin();
        }
    }

    fn multiply_largest_basins(&self) -> i32 {
        let mut basins = Vec::new();
        let max_basin_number = self.val.iter().map(|e| e.basin_number()).max().unwrap();
        for k in 1..=max_basin_number {
            let mut basin_size = 0;
            for i in 0..self.rows() {
                for j in 0..self.cols() {
                    if self.element(i, j).basin_number() == k {
                        basin_size += 1;
                    }
                }
            }
            basins.push(basin_size);
        }
        basins.sort_unstable_by(|a, b| a.cmp(b).reverse());
        let basin_iter = basins.iter();
        basin_iter.take(3).product()
    }
}

#[derive(PartialEq, Debug, Clone)]
enum Basin {
    Unchecked,
    None,
    Number(usize),
}

trait Heightmap {
    fn val(&self) -> i32;
    fn risk(&self) -> i32;
    fn from_char(c: char) -> Self;
    fn set_basin_number(&mut self, _: usize) {}
    fn basin_number(&self) -> usize {
        0
    }
    fn is_basin(&self) -> bool {
        true
    }
    fn no_basin(&mut self) {}
    fn unchecked(&self) -> bool {
        true
    }
}

impl Heightmap for i32 {
    fn risk(&self) -> i32 {
        self + 1
    }
    fn from_char(c: char) -> Self {
        c.to_digit(10).unwrap() as Self
    }
    fn val(&self) -> i32 {
        *self
    }
}

impl Heightmap for (i32, Basin) {
    fn risk(&self) -> i32 {
        self.0.risk()
    }
    fn from_char(c: char) -> Self {
        (c.to_digit(10).unwrap() as i32, Basin::Unchecked)
    }
    fn val(&self) -> i32 {
        self.0
    }
    fn basin_number(&self) -> usize {
        match self.1 {
            Basin::Number(n) => n,
            _ => 0,
        }
    }
    fn set_basin_number(&mut self, basin_number: usize) {
        self.1 = Basin::Number(basin_number);
    }
    fn no_basin(&mut self) {
        self.1 = Basin::None;
    }
    fn unchecked(&self) -> bool {
        self.1 == Basin::Unchecked
    }
    fn is_basin(&self) -> bool {
        matches!(self.1, Basin::Number(_))
    }
}
