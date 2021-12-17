use aoc2021::*;
use std::env;

fn main() {
    let mut args = env::args().take(2);
    args.next();
    match args.next() {
        Some(day) if day == "01" => day_01::main(),
        Some(day) if day == "02" => day_02::main(),
        Some(day) if day == "03" => day_03::main(),
        Some(day) if day == "04" => day_04::main(),
        Some(day) if day == "05" => day_05::main(),
        Some(day) if day == "06" => day_06::main(),
        Some(day) if day == "07" => day_07::main(),
        Some(day) if day == "08" => day_08::main(),
        Some(day) if day == "09" => day_09::main(),
        Some(_) => println!("Not implemented."),
        None => println!("Please specify a day"),
    }
}
