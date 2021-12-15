use aoc_downloader::download_day;
use std::{collections::{HashMap, HashSet}, cmp::Ordering};

const DAY: u32 = 15;
type InputType = Vec<Vec<Node>>;

#[derive(Clone, Copy, Debug, Hash)]
struct Node {
    pub weight: u64,
    pub accumulated_risk: u64,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.accumulated_risk == other.accumulated_risk
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.accumulated_risk.cmp(&other.accumulated_risk)
    }
}

fn get_input() -> String {
    download_day((DAY) as u32, "input").unwrap();
    std::fs::read_to_string(format!("input/input{}.txt", DAY)).unwrap()
}

fn parse_input(input: &str) -> InputType {
    input.lines()
        .filter(|line| *line != "")
        .map(|line| line.chars().map(|c| {
            Node {
                weight: c.to_digit(10).unwrap() as u64,
                accumulated_risk: u64::MAX,
            }}).collect::<Vec<Node>>())
        .collect()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(&input);
    println!("Running day {}:\n\tPart 1 {}\n\tPart 2: {}", DAY, part1(input.clone()), part2(input.clone()));
}

fn get_neighbours(current_pos: (usize, usize), input: &InputType) -> Vec<(usize, usize)> {
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
        neighbours.push((new_pos.0 as usize, new_pos.1 as usize));
    }

    neighbours
}

fn part1(mut input: InputType) -> u64 {
    let end = (input.len() - 1, input[0].len() - 1);
    let mut graph: HashMap<(usize, usize), Node> = HashMap::new();

    input[0][0].accumulated_risk = 0;

    for (y, row) in input.iter().enumerate() {
        for (x, spot) in row.iter().enumerate() {
            graph.insert((y, x), *spot);
        }
    }

    while !graph.is_empty() {
        let mut current_cords = (0, 0);
        let mut min_risk = u64::MAX;
        for (y, row) in input.iter().enumerate() {
            for (x, spot) in row.iter().enumerate() {
                if !graph.contains_key(&(y, x)) {
                    continue;
                }
                if spot.accumulated_risk < min_risk {
                    min_risk = spot.accumulated_risk;
                    current_cords = (y, x);
                }
            }
        }
        graph.remove(&current_cords);

        for neighbour in get_neighbours(current_cords, &input) {
            let new_distance = input[current_cords.0][current_cords.1].accumulated_risk + input[neighbour.0][neighbour.1].weight;
            if new_distance < input[neighbour.0][neighbour.1].accumulated_risk {
                input[neighbour.0][neighbour.1].accumulated_risk = new_distance;
            }
        }
    }

    let mut current = end;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    while current != (0, 0) {
        visited.insert(current);
        let mut min_risk = u64::MAX;
        for neighbour in get_neighbours(current, &input) {
            if visited.contains(&neighbour) {
                continue;
            }
            if input[neighbour.0][neighbour.1].accumulated_risk < min_risk {
                current = neighbour;
                min_risk = input[neighbour.0][neighbour.1].accumulated_risk;
            }
        }
    }
    input[end.0][end.1].accumulated_risk
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

fn part2(input: InputType) -> u64 {
    let mut new_input = input.clone();
    for x_ in 1..5 {
        for (y, row) in input.iter().enumerate() {
            new_input[y].append(&mut row.iter()
                .map(|v| { Node { weight: update_weight(v.weight, x_), accumulated_risk: u64::MAX}} )
                .collect())
        }
    }
    let new_new_input = new_input.clone();

    for y_ in 1..5 {
        new_input.append(&mut new_new_input.iter()
            .map(|row| row.iter().map(|v| { Node { weight: update_weight(v.weight, y_), accumulated_risk: u64::MAX}})
            .collect()).collect());
    }

    let mut input = new_input.clone();

    let end = (input.len() - 1, input[0].len() - 1);
    let mut graph: HashMap<(usize, usize), Node> = HashMap::new();

    input[0][0].accumulated_risk = 0;

    for (y, row) in input.iter().enumerate() {
        for (x, spot) in row.iter().enumerate() {
            graph.insert((y, x), *spot);
        }
    }

    while !graph.is_empty() {
        let mut current_cords = (0, 0);
        let mut min_risk = u64::MAX;
        for (y, row) in input.iter().enumerate() {
            for (x, spot) in row.iter().enumerate() {
                if !graph.contains_key(&(y, x)) {
                    continue;
                }
                if spot.accumulated_risk < min_risk {
                    min_risk = spot.accumulated_risk;
                    current_cords = (y, x);
                }
            }
        }
        graph.remove(&current_cords);

        for neighbour in get_neighbours(current_cords, &input) {
            let new_distance = input[current_cords.0][current_cords.1].accumulated_risk + input[neighbour.0][neighbour.1].weight;
            if new_distance < input[neighbour.0][neighbour.1].accumulated_risk {
                input[neighbour.0][neighbour.1].accumulated_risk = new_distance;
            }
        }
        if current_cords == end {
            break;
        }
    }

    let mut current = end;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    while current != (0, 0) {
        visited.insert(current);
        println!("{:?} - {:?}", current, input[current.0][current.1]);
        let mut min_risk = u64::MAX;
        for neighbour in get_neighbours(current, &input) {
            if visited.contains(&neighbour) {
                continue;
            }
            if input[neighbour.0][neighbour.1].accumulated_risk < min_risk {
                current = neighbour;
                min_risk = input[neighbour.0][neighbour.1].accumulated_risk;
            }
        }
    }
    input[end.0][end.1].accumulated_risk
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
