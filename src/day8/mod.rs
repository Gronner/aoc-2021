use aoc_downloader::download_day;
use std::collections::HashMap;

const DAY: u32 = 8;
type InputType = Input;

struct Input {
    input: Vec<String>,
    output: Vec<String>,
}

fn get_input() -> String {
    download_day((DAY) as u32, "input").unwrap();
    std::fs::read_to_string(format!("input/input{}.txt", DAY)).unwrap()
}

fn parse_input(input: &str) -> Vec<InputType> {
    input.lines()
        .map(|line| {
            let inp: Vec<String> = line.split('|').map(|inp| inp.to_owned()).collect();
            Input {
                input: inp[0].split(' ').map(|inp2| inp2.to_owned()).collect::<Vec<String>>(),
                output: inp[1].split(' ').map(|inp2| inp2.to_owned()).collect::<Vec<String>>(),
            }})
        .collect()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(&input);
    println!("Running day {}:\n\tPart1 {}\n\tPart2 {}", DAY, part1(&input), part2(&input));
}

fn part1(input: &Vec<InputType>) -> u32{
    let mut count = 0;
    for inp in input {
        for inp2 in &inp.output {
            match inp2.len() {
                2 | 3 | 4 | 7 => count += 1,
                _ => (),
            }
        }
    }
    count
}

fn match_index(input: char) -> usize {
    match input {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        _ => panic!(),
    }
}

fn segements_to_digit(input: &str, mapping: &HashMap<char, char>) -> u32 {
    lazy_static! {
        static ref NUMBERS: Vec<Vec<bool>> = vec![
            vec![true,  true, true, false, true, true, true], // 0
            vec![false, false, true, false, false, true, false], // 1
            vec![true,  false, true, true, true, false, true], // 2
            vec![true,  false, true, true, false, true, true], // 3
            vec![false, true, true, true, false, true, false], // 4
            vec![true,  true, false, true, false, true, true], // 5
            vec![true,  true, false, true, true, true, true], // 6
            vec![true,  false, true, false, false, true, false], // 7
            vec![true,  true, true, true, true, true, true], // 8
            vec![true, true, true, true, false, true, true], // 9
        ];
    }
    let mut segments = vec![false; 7];
    for segment in input.chars() {
        segments[match_index(mapping[&segment])] = true;
    }
    NUMBERS.iter().position(|number| *number == segments).unwrap() as u32
}

fn get_c_and_f(current: &String, inputs: &Vec<String>, mapping: &mut HashMap<char, char>) {
    let segments: Vec<char> = current.chars().collect();
    let mut count_c = 0;
    for number in inputs {
        if number.chars().any(|c| c == segments[0]) {
            count_c += 1;
        }
    }
    if count_c == 8 {
        mapping.insert(segments[0], 'c');
        mapping.insert(segments[1], 'f');
    } else {
        mapping.insert(segments[0], 'f');
        mapping.insert(segments[1], 'c');
    }
}

fn get_a(current: &String, mapping: &mut HashMap<char, char>) {
    let segments: Vec<char> = current.chars().collect();
    let a = segments.iter().find(|segment| !mapping.contains_key(segment)).unwrap();
    mapping.insert(*a, 'a');
}

fn get_b_and_d(current: &String, inputs: &Vec<String>, mapping: &mut HashMap<char, char>) {
    let segments: Vec<char> = current.chars().collect();
    let possible_b = segments.iter()
        .find(|segment| !mapping.contains_key(segment))
        .unwrap();
    let possible_d = segments.iter()
        .find(|segment| !mapping.contains_key(segment) && *segment != possible_b)
        .unwrap();
    let mut count_b = 0;
    for number in inputs {
        if number.chars().any(|c| c == *possible_b) {
            count_b += 1; 
        }
    }
    if count_b == 6 {
        mapping.insert(*possible_b, 'b');
        mapping.insert(*possible_d, 'd');
    } else {
        mapping.insert(*possible_b, 'd');
        mapping.insert(*possible_d, 'b');
    }
}

fn get_e_and_g(current: &String, inputs: &Vec<String>, mapping: &mut HashMap<char, char>) {
    let segments: Vec<char> = current.chars().collect();
    let possible_e = segments.iter()
        .find(|segment| !mapping.contains_key(segment))
        .unwrap();
    let possible_g = segments.iter()
        .find(|segment| !mapping.contains_key(segment) && *segment != possible_e)
        .unwrap();
    let mut count_b = 0;
    for number in inputs {
        if number.chars().any(|c| c == *possible_e) {
            count_b += 1; 
        }
    }
    if count_b == 4 {
        mapping.insert(*possible_e, 'e');
        mapping.insert(*possible_g, 'g');
    } else {
        mapping.insert(*possible_e, 'g');
        mapping.insert(*possible_g, 'e');
    }
}

fn part2(input: &Vec<InputType>) -> u32 {
    let mut total_sum = 0;
    for inp in input {
        let mut matches = HashMap::<char, char>::new();
        let one = inp.input.iter().find(|number| 2 == number.len()).unwrap();
        get_c_and_f(&one, &inp.input, &mut matches);
        let seven = inp.input.iter().find(|number| 3 == number.len()).unwrap();
        get_a(&seven, &mut matches);
        let four = inp.input.iter().find(|number| 4 == number.len()).unwrap();
        get_b_and_d(&four, &inp.input, &mut matches);
        let eight = inp.input.iter().find(|number| 7 == number.len()).unwrap();
        get_e_and_g(&eight, &inp.input, &mut matches);

        let mut line_sum = 0;
        for out in &inp.output {
            if out == "" {
                continue;
            }
            line_sum = line_sum * 10 + segements_to_digit(&out, &matches);
        }
        total_sum += line_sum;
    }

    total_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day8_part1_output() {
        let input = parse_input(&get_input());
        assert_eq!(342, part1(&input));
    }

    #[test]
    fn day8_part2_output() {
        let input = parse_input(&get_input());
        assert_eq!(1068933, part2(&input));
    }
}
