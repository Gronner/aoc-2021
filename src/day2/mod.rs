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
    let re = Regex::new(r"(.+) (\d)").unwrap();
    input
        .iter()
        .map(|i| {
            for cap in re.captures_iter(i) {
                return vec![cap[1].to_owned(), cap[2].to_owned()];
        }
        vec![]})
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
            ("forward", m) => pos_x = pos_x + m,
            ("down", m) => pos_y = pos_y + m,
            ("up", m) => pos_y = pos_y - m,
            (_, _) => panic!(),
        }
    }
    pos_x * pos_y
}

fn part2(input: &Vec<Vec<String>>) -> i32 {
    let mut aim = 0;
    let mut pos_x: i32 = 0;
    let mut pos_y: i32 = 0;
    for inp in input {
        match (inp[0].as_ref(), inp[1].parse::<i32>().unwrap()) {
            ("forward", m) => {
                pos_x = pos_x + m;
                pos_y = pos_y + (aim * m);
            }
            ("down", m) => aim = aim + m,
            ("up", m) => aim = aim - m,
            (_, _) => panic!(),
        }
    }
    pos_x * pos_y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day0_part1_output() {
        let input = parse_input(get_input());
        assert_eq!(1215, part1(&input));
    }

    #[test]
    fn day0_part1_testcase1() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260,263];
        assert_eq!(7, part1(&input));
    }

    #[test]
    fn day0_part2_output() {
        let input = parse_input(get_input());
        assert_eq!(1150, part2(&input));
    }
}
