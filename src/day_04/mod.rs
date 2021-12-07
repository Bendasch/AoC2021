use crate::get_contents;
use std::collections::LinkedList;
use std::str::{Lines, Split};

pub fn main() {
    println!("Day-04 part 1: {}", part_one());
    println!("Day-04 part 2: {}", part_two());
}

fn part_one() -> u32 {
    let contents = get_contents("src/day_04/input.txt");
    let mut lines = contents.lines();
    let numbers = lines.next().unwrap().split(',');
    let boards = create_boards(lines);
    let winning_boards = play_bingo(boards, numbers);
    let first_winner = winning_boards.front().unwrap();
    first_winner.calculate_answer()
}

fn part_two() -> u32 {
    let contents = get_contents("src/day_04/input.txt");
    let mut lines = contents.lines();
    let numbers = lines.next().unwrap().split(',');
    let boards = create_boards(lines);
    let winning_boards = play_bingo(boards, numbers);
    let last_winner = winning_boards.back().unwrap();
    last_winner.calculate_answer()
}

#[derive(Clone)]
struct BingoBoard {
    values: Vec<String>,
    marked: Vec<bool>,
    has_bingo: bool,
    winning_number: u32,
}

impl BingoBoard {
    fn new(lines: &mut Lines) -> Option<BingoBoard> {
        let mut values = Vec::new();
        lines.next(); // skip empty line between boards
        for _ in 0..5 {
            match lines.next() {
                Some(line) if !line.is_empty() => {
                    line.split(' ').for_each(|value| {
                        if !value.is_empty() {
                            values.push(String::from(value));
                        }
                    });
                }
                _ => return None,
            }
        }
        let marked = vec![false; 25];
        Some(BingoBoard {
            values,
            marked,
            has_bingo: false,
            winning_number: 0,
        })
    }

    fn process_number(&mut self, value: &str) {
        if let Some(index) = self.values.iter().position(|v| v.as_str() == value) {
            self.marked[index] = true;
        }
    }

    fn check_bingo(&mut self) -> bool {
        for i in 0..5 {
            if self.has_bingo_on_row(i) || self.has_bingo_on_column(i) {
                self.has_bingo = true;
                return true;
            }
        }
        false
    }

    fn has_bingo_on_row(&self, row: usize) -> bool {
        for i in 0..5 {
            if !self.marked[i + row * 5] {
                return false;
            }
        }
        true
    }

    fn has_bingo_on_column(&self, column: usize) -> bool {
        for i in 0..5 {
            if !self.marked[i * 5 + column] {
                return false;
            }
        }
        true
    }

    fn calculate_answer(&self) -> u32 {
        let mut answer = 0;
        for (i, marked) in self.marked.iter().enumerate() {
            if !marked {
                answer += self.values[i].parse::<u32>().unwrap();
            }
        }
        answer *= self.winning_number;
        answer
    }
}

fn create_boards(mut lines: Lines) -> Vec<BingoBoard> {
    let mut boards = Vec::new();
    loop {
        let board = BingoBoard::new(&mut lines);
        if board.is_none() {
            break;
        }
        boards.push(board.unwrap());
    }
    boards
}

fn play_bingo(mut boards: Vec<BingoBoard>, numbers: Split<char>) -> LinkedList<BingoBoard> {
    let mut winning_boards = LinkedList::new();
    for number in numbers {
        let mut new_winners = process_number(&mut boards, number);
        winning_boards.append(&mut new_winners);
    }
    winning_boards
}

fn process_number(boards: &mut [BingoBoard], number: &str) -> LinkedList<BingoBoard> {
    let mut winning_boards = LinkedList::new();
    for board in boards.iter_mut().filter(|b| !b.has_bingo) {
        board.process_number(number);
        if board.check_bingo() {
            board.winning_number = number.parse::<u32>().unwrap();
            winning_boards.push_back(board.clone());
        };
    }
    winning_boards
}
