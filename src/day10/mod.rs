use aoc_downloader::download_day;
use std::collections::HashSet;

const DAY: u32 = 10;
type InputType = String;

fn get_input() -> String {
    download_day((DAY) as u32, "input").unwrap();
    std::fs::read_to_string(format!("input/input{}.txt", DAY)).unwrap()
}

fn parse_input(input: &str) -> Vec<InputType> {
    input.lines()
        .map(|line| line.to_owned())
        .collect::<Vec<_>>()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(&input);
    println!("Running day {}:\n\tPart1 {}\n\tPart2 {}", DAY, part1(&input), part2(&input));
}

fn part1(input: &Vec<InputType>) -> u32{
    0
}

fn part2(input: &Vec<InputType>) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day10_part1_output() {
        let input = parse_input(&get_input());
        assert_eq!(425, part1(&input));
    }

    #[test]
    fn day10_part2_output() {
        let input = parse_input(&get_input());
        assert_eq!(1135260, part2(&input));
    }
}