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
    println!("Running day {}:\n\tPart1 {}\n\tPart2 {}", DAY, part1(&input), part2(&input));
}

fn compute_score(error: char) -> u32 {
    match error {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn part1(input: &Vec<InputType>) -> u32{
    let mut high_score = 0;
    for line in input {
        let mut stack = Vec::new();
        let mut round_score = 0;
        for c in line.chars() {
            match c {
                '(' => stack.push(c),
                ')' => {
                    if let Some(poped) = stack.pop() {
                        if poped != '(' {
                            round_score = compute_score(c);
                            break;
                        }
                    }
                },
                '[' => stack.push(c),
                ']' => {
                    if let Some(poped) = stack.pop() {
                        if poped != '[' {
                            round_score = compute_score(c);
                            break;
                        }
                    }
                },
                '{' => stack.push(c),
                '}' => {
                    if let Some(poped) = stack.pop() {
                        if poped != '{' {
                            round_score = compute_score(c);
                            break;
                        }
                    }
                },
                '<' => stack.push(c),
                '>' => {
                    if let Some(poped) = stack.pop() {
                        if poped != '<' {
                            round_score = compute_score(c);
                            break;
                        }
                    };
                },
                _ => (),
            }
        }
        if !stack.is_empty() {
            high_score += round_score;
        }
    }

    high_score
}

fn part2(input: &Vec<InputType>) -> u64 {
    let mut incomplete = input.clone();
    for line in input {
        let mut stack = Vec::new();
        for c in line.chars() {
            match c {
                '(' => stack.push(c),
                ')' => {
                    if let Some(poped) = stack.pop() {
                        if poped != '(' {
                            let idx = incomplete.iter().position(|l| l == line).unwrap();
                            incomplete.remove(idx);
                            break;
                        }
                    }
                },
                '[' => stack.push(c),
                ']' => {
                    if let Some(poped) = stack.pop() {
                        if poped != '[' {
                            let idx = incomplete.iter().position(|l| l == line).unwrap();
                            incomplete.remove(idx);
                            break;
                        }
                    }
                },
                '{' => stack.push(c),
                '}' => {
                    if let Some(poped) = stack.pop() {
                        if poped != '{' {
                            let idx = incomplete.iter().position(|l| l == line).unwrap();
                            incomplete.remove(idx);
                            break;
                        }
                    }
                },
                '<' => stack.push(c),
                '>' => {
                    if let Some(poped) = stack.pop() {
                        if poped != '<' {
                            let idx = incomplete.iter().position(|l| l == line).unwrap();
                            incomplete.remove(idx);
                            break;
                        }
                    };
                },
                _ => (),
            }
        }
    }

    let mut line_scores = Vec::new();
    for line in incomplete {
        let mut stack = Vec::new();
        for c in line.chars() {
            match c {
                '(' => stack.push(c),
                ')' => {
                    if let Some(poped) = stack.pop() {
                        if poped != '(' {
                            stack.push(poped);
                        }
                    }
                },
                '[' => stack.push(c),
                ']' => {
                    if let Some(poped) = stack.pop() {
                        if poped != '[' {
                            stack.push(poped);
                        }
                    }
                },
                '{' => stack.push(c),
                '}' => {
                    if let Some(poped) = stack.pop() {
                        if poped != '{' {
                            stack.push(poped);
                        }
                    }
                },
                '<' => stack.push(c),
                '>' => {
                    if let Some(poped) = stack.pop() {
                        if poped != '<' {
                            stack.push(poped);
                        }
                    };
                },
                _ => (),
            }
        }
        let mut line_score: u64 = 0;
        for c in stack.iter().rev() {
            print!("{}", c);
            line_score *= 5;
            line_score += match c {
                '[' => 2,
                '(' => 1,
                '{' => 3,
                '<' => 4,
                _ => 0,
            }
        }
        println!("{}", line_score);
        line_scores.push(line_score);
    } 
    let len = line_scores.len();
    line_scores.sort();
    line_scores[len / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day10_part1_output() {
        let input = parse_input(&get_input());
        assert_eq!(425, part1(&input));
    }

    #[test]
    fn day10_part2_output() {
        let input = parse_input(&get_input());
        assert_eq!(1135260, part2(&input));
    }
}
