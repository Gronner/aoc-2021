use aoc_downloader::download_day;

const DAY: u32 = 15;
type InputType = Vec<String>;

fn get_input() -> String {
    download_day((DAY) as u32, "input").unwrap();
    std::fs::read_to_string(format!("input/input{}.txt", DAY)).unwrap()
}

fn parse_input(input: &str) -> InputType {
    input.lines()
        .filter(|line| *line != "")
        .map(|line| line.to_string())
        .collect()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(&input);
    println!("Running day {}:\n\tPart 1 {}\n\tPart 2: {}", DAY, part1(input.clone()), part2(input.clone()));
}

fn part1(input: InputType) -> u64 {
    0
}

fn part2(input: InputType) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day15_part1_output() {
        let input = parse_input(&get_input());
        assert_eq!(3406, part1(input));
    }

    #[test]
    fn day15_part2_output() {
        let input = parse_input(&get_input());
        assert_eq!(3941782230241, part2(input));
    }
}
