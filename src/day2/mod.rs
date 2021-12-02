use aoc_downloader::download_day;
use regex::Regex;

const DAY: u32 = 2;

enum Operation {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl From<&String> for Operation {
    fn from(input: &String) -> Self {
        let re = Regex::new(r"(.+) (\d)").unwrap();
        re.captures(input).and_then(|captured| {
            let value = captured[2].parse::<i32>().unwrap();
            match &captured[1] {
                "forward" => Some(Operation::Forward(value)),
                "down" => Some(Operation::Down(value)),
                "up" => Some(Operation::Up(value)),
                _ => None,
            }
        }).unwrap()
    }
}

struct Submarine {
    position: i32,
    depth: i32,
    aim: i32,
}

impl Submarine {
    pub fn dive(&mut self, operation: &Operation) {
        match operation {
            Operation::Forward(movement) => self.position += movement,
            Operation::Down(movement) => self.depth += movement,
            Operation::Up(movement) => self.depth -= movement,
        };
    }

    pub fn dive2(&mut self, operation: &Operation) {
        match operation {
            Operation::Forward(movement) => {
                self.position += movement;
                self.depth += self.aim * movement;
            },
            Operation::Down(change) => self.aim += change,
            Operation::Up(change) => self.aim -= change,
        };
    }

    pub fn get_traveled_distance(self) -> i32 {
        self.position * self.depth
    }
}

impl Default for Submarine {
    fn default() -> Self {
        Submarine {
            position: 0,
            depth: 0,
            aim: 0,
        }
    }
}

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day((DAY) as u32, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{}.txt", DAY)).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

fn parse_input(input: Vec<String>) -> Vec<Operation> {
    input.iter()
        .map(|inp| Operation::from(inp))
        .collect::<Vec<_>>()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(input);
    println!("Running day {}:\n\tPart1 {}\n\tPart2 {}", DAY, part1(&input), part2(&input));
}

fn part1(input: &Vec<Operation>) -> i32{
    let mut uboot = Submarine::default();
    for operation in input {
        uboot.dive(operation);
    }
    uboot.get_traveled_distance()
}

fn part2(input: &Vec<Operation>) -> i32 {
    let mut uboot = Submarine::default();
    for operation in input {
        uboot.dive2(operation);
    }
    uboot.get_traveled_distance()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2_part1_output() {
        let input = parse_input(get_input());
        assert_eq!(1694130, part1(&input));
    }

    #[test]
    fn day2_part1_testcase1() {
        let input = vec![
            String::from("forward 5"),
            String::from("down 5"),
            String::from("forward 8"),
            String::from("up 3"),
            String::from("down 8"),
            String::from("forward 2"),
        ];
        let input = parse_input(input);
        assert_eq!(150, part1(&input));
    }

    #[test]
    fn day2_part2_output() {
        let input = parse_input(get_input());
        assert_eq!(1698850445, part2(&input));
    }

    #[test]
    fn day2_part2_testcase1() {
        let input = vec![
            String::from("forward 5"),
            String::from("down 5"),
            String::from("forward 8"),
            String::from("up 3"),
            String::from("down 8"),
            String::from("forward 2"),
        ];
        let input = parse_input(input);
        assert_eq!(900, part2(&input));
    }
}
