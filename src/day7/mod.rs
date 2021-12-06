use aoc_downloader::download_day;
use par_map::ParMap;

const DAY: u32 = 7;
type InputType = String;

fn get_input() -> String {
    download_day((DAY) as u32, "input").unwrap();
    std::fs::read_to_string(format!("input/input{}.txt", DAY)).unwrap()
}

fn parse_input(input: &str) -> Vec<InputType> {
    let len = input.len();
    input.iter()
        .map(|s| s.to_owned())
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

fn part2(input: &Vec<InputType>) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day7_part1_output() {
        let input = parse_input(&get_input());
        assert_eq!(359999, part1(&input));
    }

    #[test]
    fn day7_part2_output() {
        let input = parse_input(&get_input());
        assert_eq!(1631647919273, part2(&input));
    }
}
