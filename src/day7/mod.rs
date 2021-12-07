use aoc_downloader::download_day;

const DAY: u32 = 7;
type InputType = i32;

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
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(&input);
    println!("Running day {}:\n\tPart1 {}\n\tPart2 {}", DAY, part1(&input), part2(&input));
}

fn part1(input: &Vec<InputType>) -> i32{
    let len = input.len();
    let mut median = input.clone();
    median.sort();
    let median = median[len / 2];

    input.iter().map(|&crab: &i32| (median - crab).abs()).collect::<Vec<i32>>().iter().sum::<i32>()
}

fn part2(input: &Vec<InputType>) -> i32 {
    let len = input.len();
    let max_pos = input.iter().max().unwrap();
    let min_pos = input.iter().min().unwrap();

    let mut moveses: Vec<Vec<i32>> = vec![vec![0; (max_pos - min_pos + 1) as usize]; len];

    for (idx, crab) in input.iter().enumerate() {
        for distance in *min_pos..=*max_pos {
            let offset = (crab - distance).abs();
            let mut fuel = 0;
            for i in 1..=offset {
                fuel += i;
            }
            moveses[idx][(distance - min_pos) as usize] = fuel;
        }
    }
    let mut fewest_fuel = i32::MAX;
    for distance in *min_pos..=*max_pos {
        let mut total_fuel = 0;
        for (idx, _) in input.iter().enumerate() {
            total_fuel += moveses[idx][(distance - min_pos) as usize]
        }
        if total_fuel < fewest_fuel {
            fewest_fuel = total_fuel;
        }
    }
    fewest_fuel
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
