use std::str::FromStr;
use aoc_downloader::download_day;
use regex::Regex;

use crate::utils::table::Table;

const DAY: u32 = 4;
type InputType = String;

#[derive(Debug)]
struct Board {
    board: Table<i32>,
    winning_number: i32,
}

impl Board {
    pub fn get_score(&self) -> i32 {
        self.board.get_vector()
            .iter()
            .filter(|&entry| -1 != *entry)
            .sum()
    }

    pub fn get_winning_number(&self) -> i32 {
        self.winning_number
    }

    pub fn play_game(&mut self, number: i32) -> bool{
        self.mark_number(number);
        self.winning_number = number;
        self.has_won()
    }

    fn mark_number(&mut self, number: i32) {
        let called_number = self.board.get_vetor_mut()
            .iter_mut()
            .find(|entry| number == **entry);
        if called_number.is_some() {
            *called_number.unwrap() = -1;
        }
    }

    pub fn has_won(&self) -> bool {
        self.check_row() || self.check_column()
    }

    fn check_row(&self) -> bool {
        self.board.get_rows()
            .iter()
            .any(|row| row.iter()
                .all(|&value| -1 == value))
    }

    fn check_column(&self) -> bool {
        self.board.get_columns()
            .iter()
            .any(|column| column.iter()
                .all(|&value| -1 == value))
    }
}

impl FromStr for Board {
    type Err = std::num::ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r" ").unwrap();
            static ref RE2: Regex = Regex::new(r"\s+").unwrap();
        }
        let board = input.lines()
            .filter(|line| "" != *line)
            .map(|line| RE2.replace_all(line.trim(), " "))
            .map(|line| RE.split(&line).map(|n| n.parse::<i32>().unwrap()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Ok(Board {
            board: Table::from_vecvec(board),
            winning_number: -1,
        })
    }
}

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

fn get_numbers(input: &str) -> Vec<i32> {
    let re = Regex::new(r",").unwrap();
    re.split(&input)
        .map(|v| v.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

fn build_boards(input: &Vec<InputType>) -> Vec<Board> {
    let mut boards = Vec::new();
    for board in input[1..].chunks(5) {
        let mut board_setup = String::new();
        for board_line in board {
            board_setup.push_str(&board_line);
            board_setup.push_str("\n");
        }
        boards.push(Board::from_str(&board_setup).unwrap());
    }
    boards
}

fn part1(input: &Vec<InputType>) -> u32{
    let numbers = get_numbers(&input[0]);
    let mut boards = build_boards(input);

    for number in numbers {
        for board in boards.iter_mut() {
            if board.play_game(number) {
                return (board.get_score() * number) as u32;
            }
        }
    }
    0
}

fn part2(input: &Vec<InputType>) -> u32 {
    let numbers = get_numbers(&input[0]);
    let mut boards = build_boards(input);

    let mut winning_number = -1;
    for number in numbers {
        for board in boards.iter_mut() {
            if !board.has_won() {
                board.play_game(number);
            }
        }
        if boards.iter().all(|board| board.has_won()) {
            winning_number = number;
            break;
        }
    }
    (boards.into_iter()
        .find(|board| &winning_number == &board.get_winning_number())
        .unwrap()
        .get_score() * winning_number) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day4_part1_output() {
        let input = parse_input(&get_input());
        assert_eq!(41668, part1(&input));
    }

    #[test]
    fn day4_part2_output() {
        let input = parse_input(&get_input());
        assert_eq!(10478, part2(&input));
    }
}
