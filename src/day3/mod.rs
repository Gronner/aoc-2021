use aoc_downloader::download_day;

const DAY: u32 = 3;
type InputType = u32;

fn get_input() -> String {
    download_day((DAY) as u32, "input").unwrap();
    std::fs::read_to_string(format!("input/input{}.txt", DAY)).unwrap()
}

fn parse_input(input: &str) -> Vec<InputType> {
    input.lines()
        .filter(|line| "" != *line)
        .map(|s| u32::from_str_radix(s, 2).unwrap())
        .collect::<Vec<_>>()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(&input);
    println!("Running day {}:\n\tPart1 {}\n\tPart2 {}", DAY, part1(&input), part2(&input));
}

fn bit_is_set(num: u32, bit: usize) -> bool {
    0 != (num & (1 << bit))
}

fn part1(input: &Vec<InputType>) -> u32{
    let num_len = u32::BITS as usize;
    let mut gamma = vec![0; num_len];
    for num in input {
        for bit in 0..num_len {
            if bit_is_set(*num, bit) {
                gamma[bit] += 1;
            }
        }
    }
    let mut gamma_val = 0;
    for (bit, value) in gamma.iter().enumerate() {
        if *value > (input.len() / 2) {
            gamma_val = gamma_val | 1 << bit;
        }
    }
    let mask = 0b111111111111;
    (!gamma_val & mask) * gamma_val
}

fn part2(numbers: &Vec<InputType>) -> u32 {
    let num_len = 12;
    let mut numbers = numbers.clone();
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
        assert_eq!(1025636, part1(&input));
    }

    #[test]
    fn day3_part2_output() {
        let input = parse_input(&get_input());
        assert_eq!(793873, part2(&input));
    }
}
