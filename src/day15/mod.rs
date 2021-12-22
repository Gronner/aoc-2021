use aoc_downloader::download_day;
use pathfinding::prelude::dijkstra;

const DAY: u32 = 15;
type InputType = Vec<Vec<u64>>;

fn get_input() -> String {
    download_day((DAY) as u32, "input").unwrap();
    std::fs::read_to_string(format!("input/input{}.txt", DAY)).unwrap()
}

fn parse_input(input: &str) -> InputType {
    input.lines()
        .filter(|line| *line != "")
        .map(|line| line.chars().map(|c| {
                c.to_digit(10).unwrap() as u64
            }).collect::<Vec<u64>>())
        .collect()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(&input);
    println!("Running day {}:\n\tPart 1 {}\n\tPart 2: {}", DAY, part1(input.clone()), part2(input.clone()));
}

fn get_neighbours(current_pos: (usize, usize), input: &Vec<Vec<u64>>) -> Vec<((usize, usize), usize)> {
    lazy_static!{
        static ref OFFSETS: Vec<(isize, isize)> = vec![
            (0, 1), (1, 0), (0, -1), (-1, 0),
        ];
    }
    let max_x = input[0].len() as isize;
    let max_y = input.len() as isize;
    let mut neighbours = vec![];
    for offset in OFFSETS.iter() {
        let new_pos = (current_pos.0 as isize + offset.0,
            current_pos.1 as isize + offset.1);
        if new_pos.0 < 0 || new_pos.0 >= max_y || new_pos.1 < 0 || new_pos.1 >= max_x {
            continue;
        }
        neighbours.push(((new_pos.0 as usize, new_pos.1 as usize),
            input[new_pos.0 as usize][new_pos.1 as usize] as usize));
    }

    neighbours
}

fn part1(input: InputType) -> u64 {
    let start = (0, 0);
    let end = (input.len() - 1, input[0].len() - 1);

    let result = dijkstra(&start, |&p| get_neighbours(p, &input), |&p| p == end).unwrap();

    result.1 as u64
}

fn part2(input: InputType) -> u64 {
    let input = expand(&input);

    let start = (0, 0);
    let end = (input.len() - 1, input[0].len() - 1);

    let result = dijkstra(&start, |&p| get_neighbours(p, &input), |&p| p == end).unwrap();

    result.1 as u64
}

fn expand(input: &Vec<Vec<u64>>) -> InputType {
    let mut expanded_input = input.clone();

    for x in 1..5 {
        input.iter()
            .enumerate()
            .for_each(|(y, row)| expanded_input[y].append(&mut row.iter()
                .map(|&value| update_weight(value, x))
                .collect()));
    }

    let tmp_input = expanded_input.clone();
    for y in 1..5 {
        expanded_input.append(&mut tmp_input.iter()
            .map(|row| row.iter().map(|&value| update_weight(value, y))
                .collect())
            .collect());
    }

    expanded_input
}

fn update_weight(mut old: u64, iteration: u64) -> u64 {
    for _ in 0..iteration{
        old += 1;
        if old == 10 {
            old = 1;
        }
    }
    old
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day15_part1_output() {
        let input = parse_input(&get_input());
        assert_eq!(472, part1(input));
    }

    #[test]
    fn day15_part2_output() {
        let input = parse_input(&get_input());
        assert_eq!(2851, part2(input));
    }
}
