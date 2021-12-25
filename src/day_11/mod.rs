use crate::get_contents;

use std::fmt::{Display, Error, Formatter};

pub fn main() {
    println!("Day-11 part 1: {}", part_one());
    println!("Day-11 part 2: {}", part_two());
}

fn part_one() -> u32 {
    let contents = get_contents("src/day_11/input.txt");
    let mut matrix = OctoMatrix::new(contents);
    matrix.make_n_steps(100);
    //println!("{}", matrix);
    matrix.flashes
}

fn part_two() -> u32 {
    let contents = get_contents("src/day_11/input.txt");
    let mut matrix = OctoMatrix::new(contents);
    matrix.make_steps_until_all_flash()
}

struct OctoMatrix {
    energy: Vec<(i32, bool)>,
    flashes: u32,
}

impl OctoMatrix {
    fn new(contents: String) -> OctoMatrix {
        let mut energy = Vec::new();
        for line in contents.lines() {
            for char in line.chars() {
                energy.push((char.to_digit(10).unwrap() as i32, false));
            }
        }
        OctoMatrix { energy, flashes: 0 }
    }

    fn make_n_steps(&mut self, n: u32) {
        for _ in 0..n {
            self.make_step();
        }
    }

    fn make_steps_until_all_flash(&mut self) -> u32 {
        let mut iteration = 0;
        loop {
            if self.all_flash() {
                return iteration;
            }
            self.reset_flashes();
            self.make_step();
            iteration += 1;
        }
    }

    fn all_flash(&self) -> bool {
        for &(_, flash) in self.energy.iter() {
            if !flash {
                return false;
            }
        }
        true
    }

    fn make_step(&mut self) {
        self.reset_flashes();
        for i in 0..10 {
            for j in 0..10 {
                self.increment(i, j);
            }
        }
    }

    fn reset_flashes(&mut self) {
        for field in self.energy.iter_mut() {
            field.1 = false;
        }
    }

    fn increment(&mut self, r: usize, c: usize) {
        let mut field = &mut self.energy[r * 10 + c];
        if field.1 {
            return;
        }
        field.0 += 1;
        if field.0 > 9 {
            self.flash(r, c);
        }
    }

    fn flash(&mut self, r: usize, c: usize) {
        self.energy[r * 10 + c].0 = 0; // set back to zero
        self.energy[r * 10 + c].1 = true; // mark as flashed
        self.flashes += 1;
        for i in 0..=2 {
            for j in 0..=2 {
                if r == 0 && i == 0 || r == 9 && i == 2 || c == 0 && j == 0 || c == 9 && j == 2 {
                    continue; // mind the bounds!
                }
                if self.energy[(r + i - 1) * 10 + (c + j - 1)].1 {
                    continue; // ignore fields which have already flashed...
                }
                self.increment(r + i - 1, c + j - 1);
            }
        }
    }
}

impl Display for OctoMatrix {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for i in 0..10 {
            for j in 0..10 {
                write!(f, "{}", self.energy[i * 10 + j].0)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
