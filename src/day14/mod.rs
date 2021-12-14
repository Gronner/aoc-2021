use aoc_downloader::download_day;
use regex::Regex;
use std::collections::HashMap;

const DAY: u32 = 14;
type InputType = String;

fn get_input() -> String {
    download_day((DAY) as u32, "input").unwrap();
    std::fs::read_to_string(format!("input/input{}.txt", DAY)).unwrap()
}

fn parse_input(input: &str) -> Vec<InputType> {
    input.lines()
        .filter(|line| *line != "")
        .map(|line| line.to_string())
        .collect::<Vec<_>>()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(&input);
    println!("Running day {}:\n\tPart 1 {}\n\tPart 2: {}", DAY, part1(&input), part2(&&input));
}

fn part1(input: &Vec<InputType>) -> u64 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(.+) -> (.)").unwrap();
    }
    let mut template: Vec<char> = input[0].chars().collect();
    let mut translations: HashMap<String, char> = HashMap::new();
    for line in input[1..].into_iter() {
        RE.captures(line).and_then(|captured| {
            translations.insert(
                captured[1].to_string(),
                captured[2].chars().nth(0).unwrap());
            Some(0)
        }).unwrap();
    }
    for round in 0..40 {
        println!("Round: {}", round); 
        let mut next_template = Vec::new();
        next_template.push(template[0]);
        for pair in template.windows(2) {
            let current: String = pair.into_iter().collect();
            let new_char = translations[&current];
            next_template.push(new_char);
            next_template.push(pair[1]);
        }
        template = next_template.clone();
    }
    let mut occurences = HashMap::new();
    for element in template {
        occurences.entry(element).and_modify(|count| *count += 1).or_insert(1);
    }
    let mut max = 0;
    let mut min = u64::MAX;
    for (element, count) in occurences {
        println!("{}, {}", element, count);
        max = std::cmp::max(max, count);
        min = std::cmp::min(min, count);
    }
    max - min
}

fn part2(input: &Vec<InputType>) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day14_part1_output() {
        let input = parse_input(&get_input());
        assert_eq!(4885, part1(&input));
    }

    #[test]
    fn day14_part2_output() {
        let input = parse_input(&get_input());
        assert_eq!(117095, part2(&input));
    }
}
