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

fn part1(input: &Vec<InputType>) -> u32 {
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
    let mask = 0b111111111111; // Determined by method of very sharp looking
    (gamma_val ^ mask) * gamma_val
}

fn part2(numbers: &Vec<InputType>) -> u32 {
    let more_of = get_dominant(numbers, 12, one_dominant);
    let less_of = get_dominant(numbers, 12, zero_dominant);

    more_of * less_of
}

fn one_dominant(ones: u32, zeros: u32) -> bool {
    ones >= zeros
}

fn zero_dominant(ones: u32, zeros: u32) -> bool {
    zeros > ones
}


fn get_dominant(numbers: &Vec<InputType>, num_len: usize, compare: fn(u32, u32) -> bool) -> InputType {
    let mut numbers = numbers.clone();
    let mut current_bit = num_len;
    while 1 != numbers.len() {
        current_bit -= 1;
        let mut zeros = 0;
        let mut ones = 0;
        for number in &numbers {
            if bit_is_set(*number, current_bit) {
                ones += 1;
            } else {
                zeros += 1;
            }
        }
        if compare(ones, zeros) {
            numbers = numbers.clone().into_iter()
                .filter(|n| bit_is_set(*n, current_bit))
                .collect();
        } else {
            numbers = numbers.clone().into_iter()
                .filter(|n| !bit_is_set(*n, current_bit))
                .collect();
        }
    }
    numbers[0]
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
