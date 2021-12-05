use std::str::FromStr;
use aoc_downloader::download_day;
use num::signum;
use regex::Regex;

use crate::utils::table::Table;

const DAY: u32 = 5;
type InputType = Lines;

#[derive(Debug)]
struct Lines {
    start: (usize, usize),
    end: (usize, usize),
}

impl FromStr for Lines {
    type Err = std::num::ParseIntError;
    
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
        Ok(re.captures(input).and_then(|captured| {
            Some(Lines {
                start: (captured[1].parse::<usize>().unwrap(), captured[2].parse::<usize>().unwrap()),
                end: (captured[3].parse::<usize>().unwrap(), captured[4].parse::<usize>().unwrap()),
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
        let mut current_pos = line.start;
        let vector = compute_normal_vector(line.start, line.end);
        if !include_diagonals(vector) {
            continue;
        }
        seafloor[current_pos] += 1;
        while line.end != current_pos {
            current_pos = next_position(current_pos, vector);
            seafloor[current_pos] += 1;
        }
    }
    seafloor.get_vector().iter().filter(|&n| 1 < *n).count() as u32
}

fn include_diagonals(vector: (isize, isize)) -> bool {
    !((vector.0 != 0) && (vector.1 != 0))
}

fn part2(input: &Vec<InputType>) -> u32 {
    let mut seafloor = Table::from_vecvec(vec![vec![0; 1000]; 1000]);
    for line in input {
        let mut current_pos = line.start;
        let vector = compute_normal_vector(line.start, line.end);
        seafloor[current_pos] += 1;
        while line.end != current_pos {
            current_pos = next_position(current_pos, vector);
            seafloor[current_pos] += 1;
        }
    }
    seafloor.get_vector().iter().filter(|&n| 1 < *n).count() as u32
}

fn compute_normal_vector(start: (usize, usize), end: (usize, usize)) -> (isize, isize) {
    (num::signum(end.0 as isize - start.0 as isize),
    num::signum(end.1 as isize - start.1 as isize))
}

fn next_position(position: (usize, usize), vector: (isize, isize)) -> (usize, usize) {
    ((position.0 as isize + vector.0) as usize,
    (position.1 as isize + vector.1) as usize)
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
