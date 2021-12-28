use crate::get_contents;

use std::fmt::{Display, Error, Formatter};

pub fn main() {
    println!("Day-13 part 1: {}", part_one());
    println!("Day-13 part 2:");
    part_two();
}

fn part_one() -> usize {
    let contents = get_contents("src/day_13/input.txt");
    let (dots, folds) = parse_input(contents);
    let mut paper = Paper::new(&dots);
    let first_fold = folds.get(0).unwrap();
    paper.fold(&first_fold.0, first_fold.1);
    paper.values.iter().fold(0, |mut acc, &x| {
        if x {
            acc += 1;
        }
        acc
    })
}

fn part_two() {
    let contents = get_contents("src/day_13/input.txt");
    let (dots, folds) = parse_input(contents);
    let mut paper = Paper::new(&dots);
    for fold in folds.into_iter() {
        paper.fold(&fold.0, fold.1);
    }
    println!("{}", paper);
}

struct Dot(usize, usize);
struct Fold(FoldDirection, usize);

fn parse_input(contents: String) -> (Vec<Dot>, Vec<Fold>) {
    let mut dots: Vec<Dot> = Vec::new();
    let mut folds: Vec<Fold> = Vec::new();

    for line in contents.lines().into_iter() {
        let mut line_split = line.split(',');
        let first_word = line_split.next().unwrap().trim();
        if first_word.is_empty() {
            continue;
        }

        if !first_word.starts_with("fold") {
            dots.push(Dot(
                first_word.parse::<usize>().unwrap(),
                line_split.next().unwrap().parse::<usize>().unwrap(),
            ));
            continue;
        }

        let mut line_split = line.split_whitespace();
        if line_split.next().unwrap() == "fold" {
            line_split.next(); // skip the text parts
            let mut fold_split = line_split.next().unwrap().split('=');
            let direction = match fold_split.next().unwrap() {
                "x" => FoldDirection::Vertical,
                "y" => FoldDirection::Horizontal,
                _ => panic!("Unknown direction"),
            };
            let location = fold_split.next().unwrap();
            folds.push(Fold(direction, location.parse::<usize>().unwrap()));
        }
    }

    (dots, folds)
}
struct Paper {
    values: Vec<bool>,
    ncols: usize,
}

impl Paper {
    fn new(dots: &[Dot]) -> Paper {
        let nrows = dots.iter().map(|Dot(_, y)| *y).max().unwrap() + 1;
        let ncols = dots.iter().map(|Dot(x, _)| *x).max().unwrap() + 1;
        let mut values = vec![false; nrows * ncols];
        for dot in dots.iter() {
            values[dot.1 * ncols + dot.0] = true;
        }
        Paper { values, ncols }
    }

    fn cols(&self) -> usize {
        self.ncols
    }

    fn rows(&self) -> usize {
        self.values.len() / self.ncols
    }

    fn fold(&mut self, direction: &FoldDirection, location: usize) {
        let (old_nrows, old_ncols) = (self.rows(), self.cols());
        let (nrows, ncols) = match *direction {
            FoldDirection::Vertical => (old_nrows, location),
            FoldDirection::Horizontal => (location, old_ncols),
        };

        let mut new_values = vec![false; nrows * ncols];

        for r in 0..nrows {
            for c in 0..ncols {
                new_values[r * ncols + c] = self.values[r * old_ncols + c];
                match *direction {
                    FoldDirection::Vertical if old_ncols > 2 * location - c => {
                        new_values[r * ncols + c] |= self.values[r * old_ncols + 2 * location - c];
                    }
                    FoldDirection::Horizontal if old_nrows > 2 * location - r => {
                        new_values[r * ncols + c] |=
                            self.values[(2 * location - r) * old_ncols + c];
                    }
                    _ => {}
                }
            }
        }

        self.ncols = ncols;
        self.values = new_values;
    }
}

impl Display for Paper {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for r in 0..self.rows() {
            for c in 0..self.cols() {
                if self.values[r * self.cols() + c] {
                    write!(f, "# ")?;
                } else {
                    write!(f, ". ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
enum FoldDirection {
    Vertical,
    Horizontal,
}
