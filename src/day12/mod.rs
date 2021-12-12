use aoc_downloader::download_day;
use regex::Regex;
use std::{str::FromStr, collections::HashMap};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Node {
    Upper(u32),
    Lower(u32),
    Start,
    End,
}

impl FromStr for Node {
    type Err = std::io::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let node = match input {
            "start" => Node::Start,
            "end" => Node::End,
            _ => if input.chars().nth(0).unwrap().is_lowercase() {
                    Node::Lower(input.chars().map(|c| c as u32).sum())
                } else {
                    Node::Upper(input.chars().map(|c| c as u32).sum())
                },
        };
        Ok(node)
    }
}

#[derive(Debug)]
struct Edge {
    start: Node,
    end: Node,
}

impl FromStr for Edge {
    type Err = std::io::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(.+)-(.+)").unwrap();
        }

        Ok(RE.captures(input).and_then(|captured| {
            Some(Edge {
                start: Node::from_str(&captured[1]).unwrap(),
                end: Node::from_str(&captured[2]).unwrap(),
            })
        }).unwrap())
    }
}

const DAY: u32 = 12;
type InputType = Edge;

fn get_input() -> String {
    download_day((DAY) as u32, "input").unwrap();
    std::fs::read_to_string(format!("input/input{}.txt", DAY)).unwrap()
}

fn parse_input(input: &str) -> Vec<InputType> {
    let mut input = input.lines()
        .map(|line| Edge::from_str(line).unwrap())
        .collect::<Vec<_>>();
    input.append(&mut input.iter()
        .map(|edge| Edge { start: edge.end, end: edge.start})
        .collect::<Vec<_>>());
    input
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(&input);
    println!("Running day {}:\n\tPart 1 {}\n\tPart 2 {}", DAY, part1(&input), part2(&input));
}

fn visit_cave_only(how_often: usize, visited: &HashMap<Node, usize>) -> bool {
    visited.iter()
        .filter(|(node, count)| if let Node::Lower(_) = node {
            **count == how_often
        } else {
            false
        })
        .count() != 0
}

fn go_in_direction(direction: Node, edge: &Vec<Edge>, current_path: &Vec<Node>,
    visited: &HashMap<Node, usize>, how_often: usize) -> Option<Vec<Vec<Node>>> {

    if let Node::Lower(_) = direction {
        if visited.contains_key(&direction) {
            if visit_cave_only(how_often, &visited) {
                return None;
            }
        }
    }

    if direction == Node::Start {
        return None;
    }
    
    Some(modified_dfs(direction, edge, &current_path, &visited, how_often))
}

fn modified_dfs(start: Node, edge: &Vec<Edge>, current_path: &Vec<Node>,
    visited: &HashMap<Node, usize>, how_often: usize) -> Vec<Vec<Node>> {
    let mut visited = visited.clone();
    let mut current_path = current_path.clone();
    let mut paths = Vec::new();
    visited.entry(start).and_modify(|visits| *visits += 1).or_insert(1);
    current_path.push(start);

    if start == Node::End {
        return vec![current_path];
    }

    let next_steps: Vec<&Edge> = edge.iter().filter(|edge| edge.start == start).collect();
    for next_step in next_steps {
        if let Some(mut next_path) = go_in_direction(next_step.end, edge, &current_path, &visited, how_often) {
            paths.append(&mut next_path);
        }
    }

    paths
}

fn part1(input: &Vec<InputType>) -> usize {
    modified_dfs(Node::Start, input, &vec![], &HashMap::new(), 1).len()
}

fn part2(input: &Vec<InputType>) -> usize {
    modified_dfs(Node::Start, input, &vec![], &HashMap::new(), 2).len()
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
