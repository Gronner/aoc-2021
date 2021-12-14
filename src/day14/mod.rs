use aoc_downloader::download_day;
use regex::Regex;
use std::{collections::HashMap, hash::Hash};

const DAY: u32 = 14;
type InputType = (Vec<char>, HashMap<String, char>);

fn get_input() -> String {
    download_day((DAY) as u32, "input").unwrap();
    std::fs::read_to_string(format!("input/input{}.txt", DAY)).unwrap()
}

fn parse_input(input: &str) -> InputType {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(.+) -> (.)").unwrap();
    }
    let input: Vec<String> = input.lines()
        .filter(|line| *line != "")
        .map(|line| line.to_string())
        .collect();
    let template: Vec<char> = input[0].chars().collect();
    let mut translations: HashMap<String, char> = HashMap::new();
    for line in input[1..].into_iter() {
        RE.captures(line).and_then(|captured| {
            translations.insert(
                captured[1].to_string(),
                captured[2].chars().nth(0).unwrap());
            Some(0)
        }).unwrap();
    }

    (template, translations)
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(&input);
    println!("Running day {}:\n\tPart 1 {}\n\tPart 2: {}", DAY, part1(input.clone()), part2(input.clone()));
}

fn part1(input: InputType) -> u64 {
    let (mut template, translations) = input;
    for _ in 0..10 {
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
        add_or_insert(element, 1, &mut occurences);
    }
    let mut max = 0;
    let mut min = u64::MAX;
    for (_, count) in occurences {
        max = std::cmp::max(max, count);
        min = std::cmp::min(min, count);
    }
    max - min
}

fn part2(input: InputType) -> u64 {
    let (template, translations) = input;

    let mut occurences: HashMap<String, u64> = HashMap::new();
    for pair in template.windows(2) {
        add_or_insert(pair.iter().collect::<String>(), 1, &mut occurences);
    }

    for _ in 0..40 {
        let mut next_occurences: HashMap<String, u64> = HashMap::new();
        for (pair, pair_count) in &occurences {

            let first_element = pair.chars().nth(0).unwrap();
            let second_element = pair.chars().nth(1).unwrap();
            let new_element = translations[pair];

            add_or_insert(vec![first_element, new_element].iter().collect::<String>(),
                *pair_count, &mut next_occurences);

            add_or_insert(vec![new_element, second_element].iter().collect::<String>(),
                *pair_count, &mut next_occurences);
        }
        occurences = next_occurences.clone();
    }

    let mut element_occurences: HashMap<char, u64> = HashMap::new();
    for (pair, pair_count) in &occurences {
        let element = pair.chars().nth(0).unwrap();
        add_or_insert(element, *pair_count, &mut element_occurences);
    }
    element_occurences.entry(*template.iter().last().unwrap())
        .and_modify(|element_count| *element_count += 1)
        .or_insert(1);


    let mut max = 0;
    let mut min = u64::MAX;
    for (_, count) in element_occurences {
        max = std::cmp::max(max, count);
        min = std::cmp::min(min, count);
    }
    max - min
}

fn add_or_insert<K: Eq + Hash, V: Copy + std::ops::AddAssign>(key: K, value: V, map: &mut HashMap<K, V>) {
    map.entry(key)
        .and_modify(|v| *v += value)
        .or_insert(value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day14_part1_output() {
        let input = parse_input(&get_input());
        assert_eq!(3406, part1(input));
    }

    #[test]
    fn day14_part2_output() {
        let input = parse_input(&get_input());
        assert_eq!(3941782230241, part2(input));
    }
}
