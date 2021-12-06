use aoc_downloader::download_day;
use par_map::ParMap;

const DAY: u32 = 6;
type InputType = u32;

fn get_input() -> String {
    download_day((DAY) as u32, "input").unwrap();
    std::fs::read_to_string(format!("input/input{}.txt", DAY)).unwrap()
}

fn parse_input(input: &str) -> Vec<InputType> {
    let len = input.len();
    let mut input = input.to_owned();
    input.truncate(len - 1);
    input
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(&input);
    println!("Running day {}:\n\tPart1 {}\n\tPart2 {}", DAY, part1(&input), part2(&input));
}

fn part1(input: &Vec<InputType>) -> u32{
    let mut lifetime = vec![0; 9];
    for fish in input {
        lifetime[*fish as usize] += 1;
    }
    for i in 0..80{
        lifetime.rotate_left(1);
        lifetime[6] += lifetime[8];
    }
    lifetime.iter().sum()
}

fn part2(input: &Vec<InputType>) -> u64 {
    let mut lifetime:Vec<u64> = vec![0; 9];
    for fish in input {
        lifetime[*fish as usize] += 1;
    }
    for i in 0..256 {
        lifetime.rotate_left(1);
        lifetime[6] += lifetime[8];
    }
    lifetime.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day6_part1_output() {
        let input = parse_input(&get_input());
        assert_eq!(359999, part1(&input));
    }

    #[test]
    fn day6_part2_output() {
        let input = parse_input(&get_input());
        assert_eq!(1631647919273, part2(&input));
    }
}
