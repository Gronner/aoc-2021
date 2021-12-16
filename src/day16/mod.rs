use aoc_downloader::download_day;

const DAY: u32 = 16;
type InputType = Vec<char>;

#[derive(Debug)]
struct Literal {
    packet_version: u64, // first three
    type_id: u64, // next three
    value: u64, // 5 - 15 bits, 1st bit is end indicator and gets discarded
    // 3 0s
}

#[derive(Debug)]
enum LengthValue {
    TotalLength(usize),
    SubPackets(usize),
}

impl LengthValue {
    fn decrease(&self) -> Self {
        match self {
            Self::TotalLength(i) => Self::TotalLength(i-1),
            Self::SubPackets(i) => Self::SubPackets(i-1),
        }
    }
}

#[derive(Debug)]
struct Operator {
    packet_version: u64, // first three
    type_id: u64, // next three
    length_type: LengthValue, // next_one + 15 or 11 bits to get length
    subpackets: Vec<Packet>,
}

#[derive(Debug)]
enum Packet{
    Lit(Literal),
    Op(Operator),
}


fn get_input() -> String {
    download_day((DAY) as u32, "input").unwrap();
    std::fs::read_to_string(format!("input/input{}.txt", DAY)).unwrap()
}

fn parse_input(input: &str) -> InputType {
    input.chars()
        .filter(|&c| c != '\n')
        .map(|c| c.to_digit(16).unwrap() as u64)
        .map(|number| format!("{:04b}", number))
        .collect::<String>()
        .chars()
        .rev()
        .collect()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(&input);
    println!("Running day {}:\n\tPart 1 {}\n\tPart 2: {}", DAY, part1(&input), part2(&input));
}


fn get_version(input: &mut InputType) -> u64 {
    let mut buffer = vec![];
    buffer.push(input.pop().unwrap());
    buffer.push(input.pop().unwrap());
    buffer.push(input.pop().unwrap());
    u64::from_str_radix(&buffer.iter().collect::<String>(), 2).unwrap()
}

fn get_type_id(input: &mut InputType) -> u64 {
    let mut buffer = vec![];
    buffer.push(input.pop().unwrap());
    buffer.push(input.pop().unwrap());
    buffer.push(input.pop().unwrap());
    u64::from_str_radix(&buffer.iter().collect::<String>(), 2).unwrap()
}

fn get_value(input: &mut InputType) -> (u64, usize) {
    let mut parsed = 0;
    let mut buffer = vec![];
    loop {
        let end = input.pop().unwrap();
        buffer.push(input.pop().unwrap());
        buffer.push(input.pop().unwrap());
        buffer.push(input.pop().unwrap());
        buffer.push(input.pop().unwrap());
        parsed += 5;
        if end == '0' {
            break;
        }
    }
    (u64::from_str_radix(&buffer.iter().collect::<String>(), 2).unwrap(), parsed)
}

fn get_length_type(input: &mut InputType) -> (LengthValue, usize) {
    let mut buffer = vec![];
    if input.pop().unwrap() == '0' {
        for _ in 0..15 {
            buffer.push(input.pop().unwrap());
        }
        (LengthValue::TotalLength(usize::from_str_radix(&buffer.iter().collect::<String>(), 2).unwrap()), 16)
    } else {
        for _ in 0..11 {
            buffer.push(input.pop().unwrap());
        }
        (LengthValue::SubPackets(usize::from_str_radix(&buffer.iter().collect::<String>(), 2).unwrap()), 12)
    }
}

fn parse_by_length(input: &mut InputType, len: usize) -> (Vec<Packet>, usize) {
    let mut subpackets = Vec::new();
    let mut total_parsed = 0;
    while len != total_parsed {
        let (packet, parsed) = parse_packet(input);
        total_parsed += parsed;
        subpackets.push(packet);
    }

    (subpackets, total_parsed)
}

fn parse_by_number(input: &mut InputType, mut len: usize) -> (Vec<Packet>, usize) {
    let mut subpackets = Vec::new();
    let mut total_parsed = 0;
    while len != 0 {
        let (packet, parsed) = parse_packet(input);
        len -= 1;
        total_parsed += parsed;
        subpackets.push(packet);
    }

    (subpackets, total_parsed)
}

fn discard(input: &mut InputType) {
    input.pop();
    input.pop();
    input.pop();
}

fn parse_packet(input: &mut InputType) -> (Packet, usize) {
    let mut parsed = 0;
    let packet_version = get_version(input);
    parsed += 3;
    let type_id = get_type_id(input);
    parsed += 3;

    if type_id == 4 {
        let (value, size) = get_value(input);
        parsed += size;
        (Packet::Lit(Literal { packet_version, type_id, value }), parsed)
    } else {
        let (length_type, size) = get_length_type(input);
        parsed += size;
        let output;
        match length_type {
            LengthValue::TotalLength(len) => output = parse_by_length(input, len),
            LengthValue::SubPackets(len) => output = parse_by_number(input, len),
        }
        let (subpackets, parsed_) = output;
        parsed += parsed_;
        (Packet::Op(Operator { packet_version, type_id, length_type, subpackets}), parsed)
    }
}

fn get_version_number(packet: &Packet) -> u64 {
    match packet {
        Packet::Lit(lit) => lit.packet_version,
        Packet::Op(op) => {
            let mut total_packet_version = op.packet_version;
            for package in &op.subpackets {
                total_packet_version += get_version_number(&package)
            }
            total_packet_version
        }
    }
}

fn calculate(packet: &Packet) -> u64 {
    match packet {
        Packet::Lit(lit) => lit.value,
        Packet::Op(op) => {
            match op.type_id {
                0 => {
                    let mut sum = 0;
                    for package in &op.subpackets {
                        sum += calculate(package);
                    }
                    sum
                },
                1 => {
                    let mut prod = 1;
                    for package in &op.subpackets {
                        prod *= calculate(package);
                    }
                    prod 
                },
                2 => {
                    let mut min = u64::MAX;
                    for package in &op.subpackets {
                        min = std::cmp::min(min, calculate(package));
                    }
                    min
                },
                3 => {
                    let mut max = 0;
                    for package in &op.subpackets {
                        max = std::cmp::max(max, calculate(package));
                    }
                    max
                },
                5 => {
                    if calculate(&op.subpackets[0]) > calculate(&op.subpackets[1]) { 1 } else { 0 }
                },
                6 => {
                    if calculate(&op.subpackets[0]) < calculate(&op.subpackets[1]) { 1 } else { 0 }
                },
                7 => {
                    if calculate(&op.subpackets[0]) == calculate(&op.subpackets[1]) { 1 } else { 0 }
                },
                _ => panic!(),
            }
        }
    }
}

fn part1(input: &InputType) -> u64 {
    let mut input = input.clone();
    let (packet, parsed) = parse_packet(&mut input);
    get_version_number(&packet)
}

fn part2(input: &InputType) -> u64 {
    let mut input = input.clone();
    let (packet, parsed) = parse_packet(&mut input);
    calculate(&packet)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day16_part1_output() {
        let input = parse_input(&get_input());
        assert_eq!(3406, part1(&input));
    }

    #[test]
    fn day16_part2_output() {
        let input = parse_input(&get_input());
        assert_eq!(3941782230241, part2(&input));
    }
}
