use crate::get_contents;

//use std::fmt::{Display, Error, Formatter};

pub fn main() {
    println!("Day-16 part 1: {}", part_one());
    println!("Day-16 part 2: {}", part_two());
}

fn part_one() -> u32 {
    let contents = get_contents("src/day_16/example_01.txt");
    let _packets = parse_packets::<u16>(contents);
    0
}

fn part_two() -> u32 {
    //let contents = get_contents("src/day_16/example_01.txt");
    0
}

struct Packet<T: Default> {
    _version: u8,
    _type_id: u8,
    _number_of_subpackets: T,
    _subpackets: Vec<Packet<T>>,
}

fn parse_packets<T: Default>(_contents: String) -> Vec<Packet<T>> {
    Vec::<Packet<T>>::new()
}

fn hex_to_dec(hex: String) -> u32 {
    let mut out = 0_u32;
    for (i, c) in hex.chars().enumerate() {
        let dec = c.to_digit(16).unwrap();
        out += 16_u32.pow((hex.len() - (i + 1)) as u32) * dec;
    }
    out
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_hex_to_dec() {
        let result = hex_to_dec(String::from("D2FE28"));
        assert_eq!(result, 13827624_u32);
        assert_eq!(
            format!("{:b}", result),
            String::from("110100101111111000101000")
        );
    }
}
