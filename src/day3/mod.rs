use aoc_downloader::download_day;
use regex::Regex;

const DAY: u32 = 3;
type InputType = String;

fn get_input() -> String {
    download_day((DAY) as u32, "input").unwrap();
    std::fs::read_to_string(format!("input/input{}.txt", DAY)).unwrap()
}

fn parse_input(input: &str) -> Vec<InputType> {
    input.lines()
        .filter(|line| "" != *line)
        .map(|s| s.to_owned())
        .collect::<Vec<_>>()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(&input);
    println!("Running day {}:\n\tPart1 {}\n\tPart2 {}", DAY, part1(&input), part2(&input));
}

fn part1(input: &Vec<InputType>) -> u32{
    let len_input_num = input[0].len();
    let mut gamma = Vec::new();
    for i in 0..len_input_num {
        let mut zeros = 0;
        let mut ones = 0;
        for inp in input {
            let bit = inp.chars().nth(i).unwrap();
            if '0' == bit {
                zeros += 1;
            } else {
                ones += 1;
            }
        }
        if zeros > ones {
            gamma.push(0);
        } else {
            gamma.push(1);
        }
    }
    let mut gamma_val = 0;
    for val in gamma {
        gamma_val = gamma_val << 1 | val
    }
    let mask = 0b111111111111;
    (!gamma_val & mask) * gamma_val
}

fn part2(input: &Vec<InputType>) -> u32 {
    let o2_gen = 0;
    let co_scrub = 0;
    let mut numbers = Vec::new();
    let num_len = input[0].len();
    for inp in input {
        numbers.push(u32::from_str_radix(inp, 2).unwrap());
    }
    let mut numbers2 = numbers.clone();

    let mut current_bit = num_len;
    while 1 != numbers.len() {
        current_bit -= 1;
        println!("{}", numbers.len());
        let mut zeros = 0;
        let mut ones = 0;
        for number in &numbers {
            if 0 != number & (1 << current_bit) {
                ones += 1;
            } else {
                zeros += 1;
            }
        }
        println!("{:?}", numbers);
        println!("O : {}", zeros);
        println!("1 : {}", ones);
        if ones >= zeros {
            numbers = numbers.clone().into_iter()
                .filter(|n| 0 != (*n & (1 << current_bit))).collect();
        } else {
            numbers = numbers.clone().into_iter()
                .filter(|n| 0 == (*n & (1 << current_bit))).collect();
        }
        println!("{:?}", numbers);
    }

    let mut current_bit = num_len;
    while 1 != numbers2.len() {
        current_bit -= 1;
        println!("{}", numbers2.len());
        let mut zeros = 0;
        let mut ones = 0;
        for number in &numbers2 {
            if 0 != number & (1 << current_bit) {
                ones += 1;
            } else {
                zeros += 1;
            }
        }
        println!("{:?}", numbers2);
        println!("O : {}", zeros);
        println!("1 : {}", ones);
        if zeros > ones {
            numbers2 = numbers2.clone().into_iter()
                .filter(|n| 0 != (*n & (1 << current_bit))).collect();
        } else {
            numbers2 = numbers2.clone().into_iter()
                .filter(|n| 0 == (*n & (1 << current_bit))).collect();
        }
        println!("{:?}", numbers2);
    }

    numbers[0] * numbers2[0]    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3_part1_output() {
        let input = parse_input(&get_input());
        assert_eq!(1694130, part1(&input));
    }

    #[test]
    fn day3_part1_testcase1() {
        let input = "";
        let input = parse_input(&input);
        assert_eq!(150, part1(&input));
    }

    #[test]
    fn day3_part2_output() {
        let input = parse_input(&get_input());
        assert_eq!(1698850445, part2(&input));
    }

    #[test]
    fn day3_part2_testcase1() {
        let input = "";
        let input = parse_input(input);
        assert_eq!(900, part2(&input));
    }
}
