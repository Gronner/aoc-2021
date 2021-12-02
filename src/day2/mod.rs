use aoc_downloader::download_day;
use regex::Regex;

const DAY: u32 = 2;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day((DAY) as u32, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{}.txt", DAY)).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

fn parse_input(input: Vec<String>) -> Vec<Vec<String>> {
    let re = Regex::new(r" ").unwrap();
    input.iter()
        .map(|i| re.split(i)
            .map(|f| f.to_owned())
            .collect())
        .collect::<Vec<_>>()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(input);
    println!("Running day {}:\n\tPart1 {}\n\tPart2 {}", DAY, part1(&input), part2(&input));
}

fn part1(input: &Vec<Vec<String>>) -> i32{
    let mut pos_x: i32 = 0;
    let mut pos_y: i32 = 0;
    for inp in input {
        match (inp[0].as_ref(), inp[1].parse::<i32>().unwrap()) {
            ("forward", movement) => pos_x += movement,
            ("down", movement) => pos_y += movement,
            ("up", movement) => pos_y -= movement,
            (_, _) => panic!(),
        }
    }
    pos_x * pos_y
}

fn part2(input: &Vec<Vec<String>>) -> i32 {
    let mut aim = 0;
    let mut pos_x: i32 = 0;
    let mut pos_y: i32 = 0;
    for command in input {
        match (command[0].as_ref(), command[1].parse::<i32>().unwrap()) {
            ("forward", movement) => {
                pos_x += movement;
                pos_y += aim * movement;
            }
            ("down", change) => aim += change,
            ("up", change) => aim -= change,
            (_, _) => panic!(),
        }
    }
    pos_x * pos_y
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
