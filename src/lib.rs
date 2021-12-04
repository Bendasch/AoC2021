pub mod day_01;
pub mod day_02;
pub mod day_03;

use std::fs::File;
use std::io::prelude::*;

pub fn get_contents(filename: &str) -> String {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
