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
        let vector = (-1 * num::signum(line.start.0 as isize - line.end.0 as isize),
            -1 * num::signum(line.start.1 as isize - line.end.1 as isize));
        if (vector.0 != 0) && (vector.1 != 0) {
            continue;
        }
        seafloor[current_pos] += 1;
        loop {
            current_pos = ((current_pos.0 as isize + vector.0) as usize,
                (current_pos.1 as isize + vector.1) as usize);
            seafloor[current_pos] += 1;
            if line.end == current_pos {
                break;
            }
        }
    }
    seafloor.get_vector().iter().filter(|&n| 1 < *n).count() as u32
}

fn part2(input: &Vec<InputType>) -> u32 {
    let mut seafloor = Table::from_vecvec(vec![vec![0; 1000]; 1000]);
    for line in input {
        let mut current_pos = line.start;
        let vector = (-1 * num::signum(line.start.0 as isize - line.end.0 as isize),
            -1 * num::signum(line.start.1 as isize - line.end.1 as isize));
        seafloor[current_pos] += 1;
        loop {
            current_pos = ((current_pos.0 as isize + vector.0) as usize,
                (current_pos.1 as isize + vector.1) as usize);
            seafloor[current_pos] += 1;
            if line.end == current_pos {
                break;
            }
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
        assert_eq!(41668, part1(&input));
    }

    #[test]
    fn day5_part2_output() {
        let input = parse_input(&get_input());
        assert_eq!(10478, part2(&input));
    }
}
