use aoc_downloader::download_day;
use std::collections::HashSet;
use itertools::Itertools;

const DAY: u32 = 19;
type InputType = Vec<Vec<Coords>>;
type Coords = (isize, isize, isize);

fn get_input() -> String {
    download_day((DAY) as u32, "input").unwrap();
    std::fs::read_to_string(format!("input/input{}.txt", DAY)).unwrap()
}

fn parse_input(input: &str) -> InputType {
    input.split("\n\n")
        .map(|sensorlines| sensorlines.lines()
            .skip(1)
            .map(|line| {
                let coords: Vec<&str> = line.split(",").collect_vec();
                (coords[0].parse::<isize>().unwrap(),
                coords[1].parse::<isize>().unwrap(),
                coords[2].parse::<isize>().unwrap())
            })
            .collect_vec()
        )
        .collect_vec()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(&input);
    let (beacon_count, distance) = get_beacon_map(&input);
    println!("Running day {}:\n\tPart 1 {}\n\tPart 2: {}", DAY, beacon_count, distance);
}

fn rotate_z(scan: &Vec<Coords>) -> Vec<Coords> {
    scan.iter()
        .map(|coord| (coord.1, -coord.0, coord.2))
        .collect()
}

fn rotate_y(scan: &Vec<Coords>) -> Vec<Coords> {
    scan.iter()
        .map(|coord| (coord.2, coord.1, -coord.0))
        .collect()
}

fn rotate_coords(scan: &Vec<Coords>) -> Vec<Vec<Coords>> {
    let mut rotations: Vec<Vec<Coords>> = Vec::new();
    rotations.push(scan.clone());
    let mut z_idx = 0;
    let mut y_idx = 0;
    loop {
        for _ in 0..3 {
            rotations.push(rotate_z(&rotations[z_idx]));
            z_idx += 1;
        }
        z_idx += 1;
        if z_idx >= 23 {
            break;
        }
        rotations.push(rotate_y(&rotations[y_idx]));
        y_idx += 1;
    }

    assert_eq!(24, rotations.len());
    rotations
}

fn try_combine_scans(scan_a: &Vec<Coords>, scan_b: &Vec<Coords>) -> Option<(Coords, Vec<Coords>)> {
    let mut signals = scan_a.iter().copied().collect::<HashSet<_>>();
    let distances = scan_a.iter()
        .cartesian_product(scan_b)
        .map(|(a, b)| (a.0 - b.0, a.1 - b.1, a.2 - b.2));

    for (delta_x, delta_y, delta_z) in distances {
        let moved = scan_b.iter().map(|b| (b.0 + delta_x, b.1 + delta_y, b.2 + delta_z));
        if moved.clone().filter(|pos| signals.contains(pos)).count() >= 12 {
            signals.extend(moved);
            return Some(((delta_x, delta_y, delta_z), signals.into_iter().collect()));
        }
    }
    None
}

fn can_combine_scans(input: &InputType, final_scan: &Vec<Coords>) -> Option<(Coords, Vec<Coords>, usize)> {
    for (idx, scan) in input.iter().enumerate() {
        let rotations = rotate_coords(scan);
        for rotation in rotations {
            if let Some((distance, combined)) = try_combine_scans(final_scan, &rotation) {
                return Some((distance, combined, idx));
            }
        }
    }
    None
}

fn get_beacon_map(input: &InputType) -> (u64, u64) {
    let mut input = input.clone();
    let mut distances = Vec::new();
    let mut final_scan = input.remove(0);
    while let Some((distance, combined_scan, scan_idx)) = can_combine_scans(&input, &final_scan) {
        final_scan = combined_scan;
        input.remove(scan_idx);
        distances.push(distance);
    }

    let beacon_count = final_scan.len() as u64;
    let distance = distances.iter()
        .tuple_combinations()
        .map(|(a, b)| (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs())
        .max()
        .unwrap() as u64;
    (beacon_count, distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day19_part1_output() {
        let input = parse_input(&get_input());
        assert_eq!(434, get_beacon_map(&input).0);
    }

    #[test]
    fn day19_part2_output() {
        let input = parse_input(&get_input());
        assert_eq!(11906, get_beacon_map(&input).1);
    }
}
