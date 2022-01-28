use super::get_contents;
use std::collections::HashMap;

pub fn main() {
    println!("Day-16 part 1: {}", part_one());
    println!("Day-16 part 2: {}", part_two());
}

fn part_one() -> u64 {
    let contents = get_contents("src/days/day_16/input.txt");
    let packets = parse_packets(contents);
    packets.into_iter().map(|(_, p)| p.version as u64).sum()
}

fn part_two() -> u64 {
    let contents = get_contents("src/days/day_16/input.txt");
    let packets = parse_packets(contents);
    let mut outermost_packet = packets.get(&1).unwrap().clone();
    evaluate_packet(&mut outermost_packet);
    outermost_packet.value.unwrap()
}

#[derive(Clone, Debug)]
struct Packet {
    id: u64,
    version: u8,
    type_id: TypeId,
    length_type: LengthType,
    value: Option<u64>,
    subpackets: Vec<Packet>,
}

impl Packet {
    fn new(id: u64, version: u8, type_id: TypeId) -> Self {
        Packet {
            id,
            version,
            type_id,
            length_type: LengthType::None,
            value: None,
            subpackets: Vec::new(),
        }
    }
}

#[derive(Clone, PartialEq, PartialOrd)]
enum ParserState {
    Version,
    TypeId,
    LengthType,
    Length,
    Value(u8),
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
enum TypeId {
    Sum,         // 0
    Product,     // 1
    Minimum,     // 2
    Maximum,     // 3
    Literal,     // 4
    GreaterThan, // 5
    LessThan,    // 6
    EqualTo,     // 7
}

#[derive(Clone, PartialEq, PartialOrd, Debug)]
enum LengthType {
    Bits(u16, u16),       // 0 - length 15 bits
    Subpackets(u16, u16), // 1 - length 11 bits
    None,
}

#[rustfmt::skip]
fn parse_packets(contents: String) -> HashMap<u64, Packet> {
    use self::LengthType::*;
    use self::TypeId::*;
    
    let bin_string = hex_to_bin(&contents);
    let bits = bin_string.chars().collect::<Vec<char>>();
    let mut packets = HashMap::new();
    
    let mut stack: Vec<Packet> = Vec::new();
    let mut state = ParserState::Version;
    let mut buffer = String::new();
    let mut last_value_bit = false;

    let mut current_id = 0_u64;
    let mut current_packet: Packet = Packet::new(current_id, 0, Literal);
    
    for bit in bits {
        match state {
            ParserState::Value(0) => {},
            _ => buffer.push(bit),
        }
        
        // if we are currently parsing subpackets, 
        // we may need to increment the parents bit counter!
        for packet in stack.iter_mut() {
            match packet.length_type {
                Bits(c, t) if c < t => {
                    packet.length_type = Bits(c + 1, t);
                },
                _ => {}
            }
        }

        match state {
            
            ParserState::Version if buffer.len() >= 3 => {
                current_packet.version = bin_to_dec(&buffer) as u8;
                buffer.clear();
                state = ParserState::TypeId;
            }

            ParserState::TypeId if buffer.len() >= 3 => {
                current_id += 1;
                current_packet.id = current_id;
                match bin_to_dec(&buffer) {
                    0 => current_packet.type_id = Sum,
                    1 => current_packet.type_id = Product,
                    2 => current_packet.type_id = Minimum,
                    3 => current_packet.type_id = Maximum,
                    4 => current_packet.type_id = Literal,
                    5 => current_packet.type_id = GreaterThan,
                    6 => current_packet.type_id = LessThan,
                    7 => current_packet.type_id = EqualTo,
                    _ => panic!("Invalid type id! Invalid input or parser erro!"),
                } 
                if current_packet.type_id == Literal {
                    state = ParserState::Value(0);
                } else {
                    state = ParserState::LengthType;
                };
                buffer.clear();
            }

            ParserState::LengthType => {
                current_packet.length_type = if bin_to_dec(&buffer) == 0 {
                    Bits(0, 0)
                } else {
                    Subpackets(0, 0)
                };
                buffer.clear();
                state = ParserState::Length;
            }

            ParserState::Length => {
                match current_packet.length_type {
                    Bits(..) if buffer.len() >= 15 => {
                        current_packet.length_type = Bits(0, bin_to_dec(&buffer) as u16);
                        buffer.clear();
                        state = ParserState::Version;
                        stack.push(current_packet.clone());
                    },
                    Subpackets(..) if buffer.len() >= 11 => {
                        current_packet.length_type = Subpackets(0, bin_to_dec(&buffer) as u16);
                        buffer.clear();
                        state = ParserState::Version;
                        stack.push(current_packet.clone());
                    },
                    _ => {}
                }
            }

            ParserState::Value(k) => {                
                if k == 0 {
                    last_value_bit = bit == '0';
                    state = ParserState::Value(1);
                }  else if k < 4  {
                    state = ParserState::Value(k+1);
                } else if !last_value_bit {
                    state = ParserState::Value(0);
                } else {
                    current_packet.value = Some(bin_to_dec(&buffer));
                    buffer.clear();

                    packets.insert(current_id, current_packet.clone());

                    // the current packet is done
                    // if we are currently parsing subpackets, 
                    // we need to increment the parents counters!
                    if !stack.is_empty() {
                        let mut parent_packet = stack.pop().unwrap();
                        if let Subpackets(c, t) = parent_packet.length_type {
                            parent_packet.length_type = Subpackets(c + 1, t);
                        }
                        parent_packet.subpackets.push(current_packet.clone());
                        stack.push(parent_packet);
                    }
                    state = ParserState::Version;
                }
            },
            
            _ => {}
        }
        
        // if we are currently parsing subpackets, 
        // we need to check whether the parent packet is done!
        // NOTE:    if operator packets are nested, this will require 
        //          iteration or recursion through the stack!
        let mut may_pop = true;
        while !stack.is_empty() && may_pop {
            may_pop = match stack[stack.len() - 1].length_type {
                Subpackets(c, t) if c == t => {
                    let popped_parent = stack.pop().unwrap();
                    packets.insert(popped_parent.id, popped_parent.clone());
                    if stack.len() > 0 {
                        let mut prev_parent = stack.pop().unwrap();
                        match prev_parent.length_type {
                            Subpackets(c2, t2) => { 
                                prev_parent.length_type = Subpackets(c2 + 1, t2);
                            }
                            _ => {},
                        }
                        prev_parent.subpackets.push(popped_parent);
                        stack.push(prev_parent);
                        true
                    } else {
                        false
                    }
                },
                Bits(c, t) if c == t => {
                    let popped_parent = stack.pop().unwrap();
                    packets.insert(popped_parent.id, popped_parent.clone());
                    if !stack.is_empty() {
                        let mut prev_parent = stack.pop().unwrap();
                        if let Subpackets(c2, t2) = prev_parent.length_type {
                            prev_parent.length_type = Subpackets(c2 + 1, t2);
                        }
                        prev_parent.subpackets.push(popped_parent);
                        stack.push(prev_parent);
                        true
                    } else { 
                        false
                    }
                },
                _ => false,
            }
        }
    }

    packets
}

fn hex_to_bin(hex_string: &str) -> String {
    let mut bin_string = String::new();
    for hex_char in hex_string.chars() {
        bin_string.push_str(format!("{:04b}", hex_char.to_digit(16).unwrap()).as_str());
    }
    bin_string
}

fn bin_to_dec(bin_string: &str) -> u64 {
    let mut dec = 0_u64;
    for (k, bin_char) in bin_string.chars().enumerate() {
        dec +=
            2_u64.pow((bin_string.len() - (k + 1)) as u32) * bin_char.to_digit(2).unwrap() as u64;
    }
    dec
}

fn evaluate_packet(packet: &mut Packet) {
    // if the packet is a literal, there is nothing to evaluate
    if packet.type_id == TypeId::Literal {
        return;
    }

    // evaluate all subpackets
    for subpacket in packet.subpackets.iter_mut() {
        evaluate_packet(subpacket);
    }

    // make the operation
    packet.value = match packet.type_id {
        TypeId::Sum => Some(packet.subpackets.iter().map(|p| p.value.unwrap()).sum()),
        TypeId::Product => Some(packet.subpackets.iter().map(|p| p.value.unwrap()).product()),
        TypeId::Minimum => Some(
            packet
                .subpackets
                .iter()
                .map(|p| p.value.unwrap())
                .min()
                .unwrap(),
        ),
        TypeId::Maximum => Some(
            packet
                .subpackets
                .iter()
                .map(|p| p.value.unwrap())
                .max()
                .unwrap(),
        ),
        TypeId::GreaterThan => {
            Some((packet.subpackets[0].value.unwrap() > packet.subpackets[1].value.unwrap()) as u64)
        }
        TypeId::LessThan => {
            Some((packet.subpackets[0].value.unwrap() < packet.subpackets[1].value.unwrap()) as u64)
        }
        TypeId::EqualTo => Some(
            (packet.subpackets[0].value.unwrap() == packet.subpackets[1].value.unwrap()) as u64,
        ),
        _ => panic!("This type id should never appear here."),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_hex_to_bin() {
        assert_eq!(
            hex_to_bin("EE00D40C823060"),
            "11101110000000001101010000001100100000100011000001100000"
        );
    }

    #[test]
    fn test_bin_to_dec() {
        assert_eq!(bin_to_dec("110"), 6_u64);
        assert_eq!(bin_to_dec("10001"), 17_u64);
        assert_eq!(bin_to_dec("10011000"), 152_u64);
        assert_eq!(bin_to_dec("10110"), 22_u64);
        assert_eq!(bin_to_dec("1010100"), 84_u64);
    }

    #[test]
    fn test_example_1() {
        let contents = get_contents("src/days/day_16/example_01.txt");
        let packets = parse_packets(contents);
        assert_eq!(packets.len(), 4);
        assert!(packets.get(&1).unwrap().length_type == LengthType::Subpackets(1, 1));
        assert!(packets.get(&2).unwrap().length_type == LengthType::Subpackets(1, 1));
        assert!(packets.get(&3).unwrap().length_type == LengthType::Bits(11, 11));
        assert!(packets.get(&4).unwrap().type_id == TypeId::Literal);
        assert!(packets.get(&4).unwrap().value == Some(15));
    }

    #[test]
    fn test_example_2() {
        let contents = get_contents("src/days/day_16/example_02.txt");
        let packets = parse_packets(contents);
        assert_eq!(packets.len(), 7);
        assert!(packets.get(&1).unwrap().length_type == LengthType::Subpackets(2, 2));
        assert!(packets.get(&2).unwrap().length_type == LengthType::Bits(22, 22));
        assert!(packets.get(&3).unwrap().type_id == TypeId::Literal);
        assert!(packets.get(&4).unwrap().type_id == TypeId::Literal);
        assert!(packets.get(&5).unwrap().length_type == LengthType::Subpackets(2, 2));
        assert!(packets.get(&6).unwrap().type_id == TypeId::Literal);
        assert!(packets.get(&7).unwrap().type_id == TypeId::Literal);
    }

    #[test]
    fn test_example_3() {
        let contents = get_contents("src/days/day_16/example_03.txt");
        let packets = parse_packets(contents);
        assert_eq!(packets.len(), 7);
        assert!(packets.get(&1).unwrap().length_type == LengthType::Bits(84, 84));
        assert!(packets.get(&2).unwrap().length_type == LengthType::Bits(22, 22));
        assert!(packets.get(&3).unwrap().type_id == TypeId::Literal);
        assert!(packets.get(&4).unwrap().type_id == TypeId::Literal);
        assert!(packets.get(&5).unwrap().length_type == LengthType::Subpackets(2, 2));
        assert!(packets.get(&6).unwrap().type_id == TypeId::Literal);
        assert!(packets.get(&7).unwrap().type_id == TypeId::Literal);
    }

    #[test]
    fn test_example_4() {
        let contents = get_contents("src/days/day_16/example_04.txt");
        let packets = parse_packets(contents);
        assert!(packets.get(&1).unwrap().type_id != TypeId::Literal);
        assert!(packets.get(&2).unwrap().type_id != TypeId::Literal);
        assert!(packets.get(&3).unwrap().type_id != TypeId::Literal);
        assert!(packets.get(&4).unwrap().type_id == TypeId::Literal);
        assert!(packets.get(&5).unwrap().type_id == TypeId::Literal);
        assert!(packets.get(&6).unwrap().type_id == TypeId::Literal);
        assert!(packets.get(&7).unwrap().type_id == TypeId::Literal);
        assert!(packets.get(&8).unwrap().type_id == TypeId::Literal);
        assert_eq!(packets.len(), 8);
    }

    #[test]
    fn test_example_5() {
        let contents = String::from("C200B40A82");
        let packets = parse_packets(contents);
        let mut packet = packets.get(&1).unwrap().clone();
        evaluate_packet(&mut packet);
        assert_eq!(packet.value, Some(3));
    }

    #[test]
    fn test_example_6() {
        let contents = String::from("880086C3E88112");
        let packets = parse_packets(contents);
        let mut packet = packets.get(&1).unwrap().clone();
        evaluate_packet(&mut packet);
        assert_eq!(packet.value, Some(7));
    }
}
