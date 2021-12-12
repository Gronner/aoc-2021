use aoc_downloader::download_day;

const DAY: u32 = 10;
type InputType = String;

fn get_input() -> String {
    download_day((DAY) as u32, "input").unwrap();
    std::fs::read_to_string(format!("input/input{}.txt", DAY)).unwrap()
}

fn parse_input(input: &str) -> Vec<InputType> {
    input.lines()
        .map(|line| line.to_owned())
        .collect::<Vec<_>>()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(&input);
    println!("Running day {}:\n\tPart 1 {}\n\tPart 2 {}", DAY, part1(&input), part2(&input));
}

fn compute_error_score(error: char) -> u64 {
    match error {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn compute_correction_score(correction: char) -> u64 {
    match correction {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    }
}

fn is_matching(current: char, last: char) -> bool {
    match (last, current) {
        ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => true,
        (_, _) => false,
    }
}

enum ParsingError {
    UnclosedParenthesis(char),
    UnexpectedToken(char),
}

fn parse_line(line: &String) -> Result<Vec<char>, ParsingError> {
    let mut stack = Vec::new();
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => {
                if let Some(popped) = stack.pop() {
                    if !is_matching(c, popped) {
                        return Err(ParsingError::UnclosedParenthesis(c));
                    }
                }
            },
            _ => return Err(ParsingError::UnexpectedToken(c)),
        }
    }
    Ok(stack)
}

fn parse_lines(input: &Vec<InputType>) -> (u64, Vec<Vec<char>>) {
    let mut high_score: u64 = 0;
    let mut stacks = Vec::new();
    for line in input {
        match parse_line(line) {
            Err(ParsingError::UnclosedParenthesis(c)) => high_score += compute_error_score(c),
            Err(ParsingError::UnexpectedToken(c)) => panic!("Unexpected Token encountered: {}", c),
            Ok(stack) => stacks.push(stack),
        }
    }

    (high_score, stacks)
}

fn part1(input: &Vec<InputType>) -> u64 {
    let (high_score, _) = parse_lines(input);
    high_score
}

fn part2(input: &Vec<InputType>) -> u64 {
    let (_, stacks) = parse_lines(input);
    let mut line_scores: Vec<u64> = stacks.iter()
        .map(|stack| { stack.iter().rev()
            .fold(0, |line_score , &c|  { line_score * 5 + compute_correction_score(c) })})
        .collect();
    line_scores.sort();
    let len = line_scores.len();
    line_scores[len / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day10_part1_output() {
        let input = parse_input(&get_input());
        assert_eq!(392043, part1(&input));
    }

    #[test]
    fn day10_part2_output() {
        let input = parse_input(&get_input());
        assert_eq!(1605968119, part2(&input));
    }
}
