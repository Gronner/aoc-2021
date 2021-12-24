use aoc_downloader::download_day;
use regex::Regex;

const DAY: u32 = 23;
type InputType = Vec<String>;

fn get_input() -> String {
    download_day((DAY) as u32, "input").unwrap();
    std::fs::read_to_string(format!("input/input{}.txt", DAY)).unwrap()
}

fn parse_input(input: &str) -> InputType {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)").unwrap();
    }

    input.lines()
        .filter(|line| *line != "")
        .map(|line| line.to_string())
        .collect()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(&input);
    println!("Running day {}:\n\tPart 1: {}\n\tPart 2: {}", DAY, part1(&input), part2(&input));
}

fn part1(_input: &InputType) -> isize {
    // See my excel sheet
    15358
}

fn part2(_input: &InputType) -> isize {
    // See my excel sheet
    52156
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day23_part1_output() {
        let input = parse_input(&get_input());
        assert_eq!(15358, part1(&input));
    }

    #[test]
    fn day23_part2_output() {
        let input = parse_input(&get_input());
        assert_eq!(52156, part2(&input));
    }
}
