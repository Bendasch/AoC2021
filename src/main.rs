mod days;

use days::*;
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
        Some(day) if day == "10" => day_10::main(),
        Some(day) if day == "11" => day_11::main(),
        Some(day) if day == "12" => day_12::main(),
        Some(day) if day == "13" => day_13::main(),
        Some(day) if day == "14" => day_14::main(),
        Some(day) if day == "15" => day_15::main(),
        Some(day) if day == "16" => day_16::main(),
        Some(day) if day == "17" => day_17::main(),
        Some(_) => println!("Not implemented."),
        None => println!("Please specify a day"),
    }
}
