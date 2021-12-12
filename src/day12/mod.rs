use aoc_downloader::download_day;
use regex::Regex;
use std::collections::{VecDeque, HashMap, HashSet};

const DAY: u32 = 12;
type InputType = HashMap<String, Vec<String>>;

fn get_input() -> String {
    download_day((DAY) as u32, "input").unwrap();
    std::fs::read_to_string(format!("input/input{}.txt", DAY)).unwrap()
}

fn parse_input(input: &str) -> InputType {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(.+)-(.+)").unwrap();
    }
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines() {
        let (start, end) = RE.captures(line)
            .and_then(|captured| { Some((captured[1].to_string(), captured[2].to_string())) }).unwrap();
        map.entry(start.clone()).and_modify(|next| next.push(end.clone())).or_insert(vec![end.clone()]);
        map.entry(end.clone()).and_modify(|next| next.push(start.clone())).or_insert(vec![start]);
    }
    map
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(&input);
    println!("Running day {}:\n\tPart 1 {}\n\tPart 2 {}", DAY, part1(&input), part2(&input));
}

fn solve(input: &InputType, part2: bool) -> usize {
    let mut count = 0;
    let mut everything: VecDeque<(String, HashSet<String>, Option<String>)> = VecDeque::new();
    everything.push_back(("start".to_string(), HashSet::from(["start".to_string()]), None));

    while let Some((position, small_visited, twice_visited)) = everything.pop_front() {
        if position == "end" {
            count += 1;
            continue;
        }
        for next in &input[&position] {
            if !small_visited.contains(next) {
                let mut new_small_visited = small_visited.clone();
                if next.chars().nth(0).unwrap().is_lowercase() {
                    new_small_visited.insert(next.clone());
                } 
                everything.push_back((next.clone(), new_small_visited, twice_visited.clone()));
            } else if small_visited.contains(next) && twice_visited.is_none() &&
                next != "start" && next != "end" && part2 {
                everything.push_back((next.clone(), small_visited.clone(), Some(next.clone())));
            }
        }
    }
    count
}

fn part1(input: &InputType) -> usize {
    solve(input, false)
}

fn part2(input: &InputType) -> usize {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day12_part1_output() {
        let input = parse_input(&get_input());
        assert_eq!(4885, part1(&input));
    }

    #[test]
    fn day12_part2_output() {
        let input = parse_input(&get_input());
        assert_eq!(117095, part2(&input));
    }
}
