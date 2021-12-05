use std::str::FromStr;
use aoc_downloader::download_day;
use num::signum;
use regex::Regex;

use crate::utils::{table::Table, coordinates::Vector};

const DAY: u32 = 5;
type InputType = Lines;

#[derive(Debug)]
struct Lines {
    start: Vector<usize>,
    end: Vector<usize>,
}

impl FromStr for Lines {
    type Err = std::num::ParseIntError;
    
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let re: Regex = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
        Ok(re.captures(input).and_then(|captured| {
            Some(Lines {
                start: Vector::from((captured[1].parse::<usize>().unwrap(), captured[2].parse::<usize>().unwrap())),
                end: Vector::from((captured[3].parse::<usize>().unwrap(), captured[4].parse::<usize>().unwrap())),
            })
        }).unwrap())
    }
}

fn get_input() -> String {
    download_day((DAY) as u32, "input").unwrap();
    std::fs::read_to_string(format!("input/input{}.txt", DAY)).unwrap()
}

fn parse_input(input: &str) -> Vec<InputType> {
    input.lines()
        .filter(|line| "" != *line)
        .map(|s| Lines::from_str(s).unwrap())
        .collect::<Vec<_>>()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(&input);
    println!("Running day {}:\n\tPart1 {}\n\tPart2 {}", DAY, part1(&input), part2(&input));
}

fn part1(input: &Vec<InputType>) -> u32{
    let mut seafloor = Table::from_vecvec(vec![vec![0; 1000]; 1000]);
    for line in input {
        let vector = (&line.start - &line.end).normalized();
        let mut current_pos = line.start.clone();
        if !include_diagonals(&vector) {
            continue;
        }
        seafloor[&current_pos] += 1;
        while &line.end != &current_pos {
            current_pos = &current_pos + &vector;
            seafloor[&current_pos] += 1;
        }
    }
    seafloor.get_vector().iter().filter(|&n| 1 < *n).count() as u32
}

fn include_diagonals(vector: &Vector<isize>) -> bool {
    !((vector['x'] != 0) && (vector['y'] != 0))
}

fn part2(input: &Vec<InputType>) -> u32 {
    let mut seafloor = Table::from_vecvec(vec![vec![0; 1000]; 1000]);
    for line in input {
        let mut current_pos = line.start.clone();
        let vector = (&line.start - &line.end).normalized();
        seafloor[&current_pos] += 1;
        while &line.end != &current_pos {
            current_pos = &current_pos + &vector;
            seafloor[&current_pos] += 1;
        }
    }
    seafloor.get_vector().iter().filter(|&n| 1 < *n).count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day5_part1_output() {
        let input = parse_input(&get_input());
        assert_eq!(5145, part1(&input));
    }

    #[test]
    fn day5_part2_output() {
        let input = parse_input(&get_input());
        assert_eq!(16518, part2(&input));
    }
}
