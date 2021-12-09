use aoc_downloader::download_day;
use std::collections::HashSet;
use crate::utils::table::Table;

const DAY: u32 = 9;
type InputType = Vec<u32>;

fn get_input() -> String {
    download_day((DAY) as u32, "input").unwrap();
    std::fs::read_to_string(format!("input/input{}.txt", DAY)).unwrap()
}

fn parse_input(input: &str) -> Vec<InputType> {
    input.lines()
        .map(|line| line.chars().
            map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>())
        .collect::<Vec<_>>()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(&input);
    println!("Running day {}:\n\tPart1 {}\n\tPart2 {}", DAY, part1(&input), part2(&input));
}

fn get_neighbours(current_pos: (usize, usize), input: &Vec<InputType>) -> Vec<(usize, usize)> {
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
        if new_pos.0 < 0 || new_pos.0 >= max_x || new_pos.1 < 0 || new_pos.1 >= max_y {
            continue;
        }
        neighbours.push((new_pos.0 as usize, new_pos.1 as usize));
    }

    neighbours
}

fn part1(input: &Vec<InputType>) -> u32{
    let mut risk_level = 0;
    for (y, row) in input.iter().enumerate() {
        for (x, element) in row.iter().enumerate() {
            let neighbours = get_neighbours((x, y), &input);
            if neighbours.iter().all(|v| input[v.1][v.0] > *element) {
                risk_level += element + 1;
            }
        }
    }
    risk_level
}

fn depth_search(minimum: (usize, usize), map: &Vec<InputType>) -> u32 {
    let mut queue = vec![minimum];
    let mut visited = HashSet::new();
    let mut part = 0;

    while queue.len() > 0 {
        if let Some(current) = queue.pop() {
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current);

            println!("{:?}", current);
            let value = map[current.1][current.0];
            if value != 9 {
                part += 1;
            } else {
                continue;
            }

            for neighbour in get_neighbours(current, map) {
                queue.push(neighbour);
            }
        }
    }
    part
}

fn part2(input: &Vec<InputType>) -> u32 {
    let mut minima = Vec::new();

    println!("{:?}", input);

    for (y, row) in input.iter().enumerate() {
        for (x, element) in row.iter().enumerate() {
            let neighbours = get_neighbours((x, y), &input);
            if neighbours.iter().all(|v| input[v.1][v.0] > *element) {
                minima.push((x, y));
            }
        }
    }

    let mut sizes = Vec::new();
    println!("{:?}", minima);
    for minimum in minima {
        sizes.push(depth_search(minimum, &input));
    }
    
    sizes.sort();
    sizes.reverse();

    sizes[0] * sizes[1] * sizes[2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day9_part1_output() {
        let input = parse_input(&get_input());
        assert_eq!(342, part1(&input));
    }

    #[test]
    fn day9_part2_output() {
        let input = parse_input(&get_input());
        assert_eq!(1068933, part2(&input));
    }
}
