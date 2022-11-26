use std::{ops::{IndexMut, Index}, str::FromStr};
use aoc_downloader::download_day;
use cached::proc_macro::cached;
use regex::Regex;

const DAY: u32 = 24;
type InputType = Vec<u64>;

#[derive(Debug, Clone)]
enum Command {
    Input((char, i64)),
    Add((char, String)),
    Mul((char, String)),
    Div((char, String)),
    Mod((char, String)),
    Eql((char, String))
}

#[derive(Debug, Clone, Copy)]
struct Alu {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

impl Default for Alu {
    fn default() -> Self {
        Alu {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        }
    }
}

impl Alu {
    pub fn execute(&mut self, command: &Command) {
        use Command::*;

        match command {
            Input((register, operant)) => self[register] = *operant,
            Add((register, operant)) => self[register] += self.get_value(&operant),
            Mul((register, operant)) => self[register] *= self.get_value(&operant),
            Div((register, operant)) => self[register] /= self.get_value(&operant),
            Mod((register, operant)) => self[register] %= self.get_value(&operant),
            Eql((register, operant)) => self[register] = if self[register] == self.get_value(&operant) { 1 } else { 0 },
        }
    }

    fn get_value(&self, operant: &str) -> i64 {
        match operant {
            "w" => self.w,
            "x" => self.x,
            "y" => self.y,
            "z" => self.z,
            number => number.parse::<i64>().unwrap(),
        }
    }
}

impl Index<&char> for Alu {
    type Output = i64;

    fn index(&self, index: &char) -> &Self::Output {
        match index {
            'w' => &self.w,
            'x' => &self.x,
            'y' => &self.y,
            'z' => &self.z,
            _ => unreachable!("Unkown register encountered"),
        }
    }
}

impl IndexMut<&char> for Alu {
    fn index_mut(&mut self, index: &char) -> &mut Self::Output {
        match index {
            'w' => &mut self.w,
            'x' => &mut self.x,
            'y' => &mut self.y,
            'z' => &mut self.z,
            _ => unreachable!("Unkown register encountered"),
        }
    }
}

impl FromStr for Command {
    type Err = std::io::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([a-zA-Z]{3}) ([a-zA-Z])( (.+))?").unwrap();
        }
        Ok(RE.captures(input).and_then(|captured| {
            match &captured[1] {
                "inp" => Some(Command::Input((captured[2].chars().nth(0).unwrap(), 0))),
                "add" => Some(Command::Add((captured[2].chars().nth(0).unwrap(), captured[4].to_string()))),
                "mul" => Some(Command::Mul((captured[2].chars().nth(0).unwrap(), captured[4].to_string()))),
                "div" => Some(Command::Div((captured[2].chars().nth(0).unwrap(), captured[4].to_string()))),
                "mod" => Some(Command::Mod((captured[2].chars().nth(0).unwrap(), captured[4].to_string()))),
                "eql" => Some(Command::Eql((captured[2].chars().nth(0).unwrap(), captured[4].to_string()))),
                _ => unreachable!("Unexpedted Command encountered while parsing"),
            }
        }).unwrap())
    }
}


fn get_input() -> String {
    download_day((DAY) as u32, "input").unwrap();
    std::fs::read_to_string(format!("input/input{}.txt", DAY)).unwrap()
}

fn parse_input(input: &str) -> InputType {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)").unwrap();
    }

    let _: Vec<Command> = input.lines()
        .filter(|line| *line != "")
        .map(|line| Command::from_str(line).unwrap())
        .collect();
    if let Some(numbers) = get_valid_values(0, 0) {
        return numbers.iter()
            .map(|number| number.parse::<u64>().unwrap())
            .collect::<Vec<_>>()
    }
    vec![0]
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(&input);
    println!("Running day {}:\n\tPart 1: {}\n\tPart 2: {}", DAY, part1(&input), part2(&input));
}

// This could also be read in by finding the indexes and then stepping through the program +18
const DIV_WITH_Z: [i64; 14] = [1, 1, 1, 1, 26, 26, 1, 26, 1, 26, 1, 26, 26, 26];
const ADD_TO_X: [i64; 14] = [13, 13, 10, 15, -8, -10, 11, -3, 14, -4, 14, -5, -8, -11];
const ADD_TO_Y: [i64; 14] = [15, 16, 4, 14, 1, 5, 1, 3, 3, 7, 5, 13, 3, 10];

fn execute_for_input(round: usize, z: i64, input: i64) -> i64 {
    let mut z = z;
    let x = ADD_TO_X[round] + (z % 26);
    z = z / DIV_WITH_Z[round];
    if x != input {
        z *= 26;
        z += input + ADD_TO_Y[round];
    }
    z
}

#[cached]
fn get_valid_values(number_len: usize, z: i64) -> Option<Vec<String>> {
    if number_len == 14 {
        if z == 0 {
            return Some(vec!["".to_string()]);
        } else {
            return None;
        }
    }
    // The first few operations limit the next_digit to something in the range 1 - 9, this
    // optimizes runtime (by a 1/4!)
    let next_digit = (z % 26) +  ADD_TO_X[number_len];
    let next_digits;
    if 1 <= next_digit && next_digit <= 9 {
        next_digits = vec![next_digit];
    } else {
        next_digits = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    }
    let mut numbers = Vec::new();
    for digit in next_digits {
        let new_z = execute_for_input(number_len, z, digit);
        let next_numbers = get_valid_values(number_len + 1, new_z);
        if let Some(next_numbers) = next_numbers {
            for number in next_numbers {
                numbers.push(digit.to_string() + &number);
            }
        }
    }
    Some(numbers)
}

fn part1(input: &InputType) -> u64{
    *input.iter().max().unwrap()
}

fn part2(input: &InputType) -> u64 {
    *input.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    #[test]
    fn alu_example1() {
        let input = "
inp x
mul x -1";
        let mut commands = parse_input(input);
        let mut alu = Alu::default();

        for (i, command) in commands.iter().enumerate() {
            if let Command::Input((register, _)) = command {
                commands[i] = Command::Input((*register, 100));
                break;
            }
        }

        for command in &commands {
            alu.execute(command);
        }
        assert_eq!(-100, alu.x);
    }

    #[test]
    fn alu_example2() {
        let input = "
inp z
inp x
mul z 3
eql z x";
        let inputs = vec![100, 300];
        let commands = parse_input(input);
        let mut input_counter = 0;
        let mut new_commands = commands.clone();
        for (i, command) in commands.iter().enumerate() {
            if let Command::Input((register, _)) = command {
                new_commands[i] = Command::Input((*register, inputs[input_counter]));
                input_counter += 1;
                if input_counter == 2 {
                    break;
                }
            }
        }

        let mut alu = Alu::default();
        for command in &new_commands {
            alu.execute(command);
        }
        assert_eq!(1, alu.z);

        let input = vec![
            Command::Input(('z', 200)),
            Command::Input(('x', 300)),
            Command::Mul(('z', "-1".to_string())),
            Command::Eql(('z', "x".to_string())),
        ];
        let mut alu = Alu::default();
        for command in &input {
            alu.execute(command);
        }
        assert_eq!(0, alu.z);
    }

    #[test]
    fn alu_example3() {
        let input = vec![
            Command::Input(('w', 10)),
            Command::Add(('z', "2".to_string())),
            Command::Mod(('z', "2".to_string())),
            Command::Div(('w', "2".to_string())),
            Command::Add(('y', "w".to_string())),
            Command::Mod(('y', "2".to_string())),
            Command::Div(('w', "2".to_string())),
            Command::Add(('x', "w".to_string())),
            Command::Mod(('x', "2".to_string())),
            Command::Div(('w', "2".to_string())),
            Command::Mod(('w', "2".to_string())),
        ];
        let mut alu = Alu::default();
        for command in &input {
            alu.execute(command);
        }
        assert_eq!(0, alu.z);
        assert_eq!(1, alu.y);
        assert_eq!(0, alu.x);
        assert_eq!(1, alu.w);
    }
    */

    #[test]
    fn day24_part1_output() {
        let input = parse_input(&get_input());
        assert_eq!(51939397989999, part1(&input));
    }

    #[test]
    fn day24_part2_output() {
        let input = parse_input(&get_input());
        assert_eq!(11717131211195, part2(&input));
    }
}
