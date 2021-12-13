use aoc_downloader::download_day;
use regex::Regex;

const DAY: u32 = 13;
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
    println!("Running day {}:\n\tPart 1 {}\n\tPart 2 {}", DAY, part1(&input), part2(&input));
}

fn part1(input: &Vec<InputType>) -> u64 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"fold along (.)=(\d+)").unwrap();
    }

    //let mut paper = vec![vec![0; 15]; 15];
    let mut paper = vec![vec![0; 1500]; 1500];
    let mut folds = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for line in input {
        if !line.starts_with("fold along") {
            let coords: Vec<usize> = line.split(",").map(|number| number.parse::<usize>().unwrap()).collect();
            if max_y < coords[1] {
                max_y = coords[1];
            }
            if max_x < coords[0] {
                max_x = coords[0];
            }
            paper[coords[1]][coords[0]] = 1;
        } else {
            RE.captures(line).and_then(|captured| {
                let value = captured[2].parse::<usize>().unwrap();
                let idx = if captured[1] == *"y" { 
                    0
                } else {
                    1
                };
                folds.push(vec![idx, value]);
            Some(true)
            }).unwrap();
        }
    }
    paper.drain(max_y+1..);
    for row in &mut paper {
        row.drain(max_x+1..);
    }
    fold_it(folds[0].clone(), &mut paper);
    let mut count = 0;
    for row in paper {
        for spot in row {
            if spot > 0 {
                count += 1;
            }
        }
    }
    count
}

fn fold_it(fold: Vec<usize>, paper: &mut Vec<Vec<usize>>) {
    if fold[0] == 0 {
        paper.remove(fold[1]);
        let mut lower_half: Vec<Vec<usize>> = paper.drain(fold[1]..).collect();
        lower_half.reverse();
        for (y, row) in lower_half.iter().enumerate() {
            for (x, spot) in row.iter().enumerate() {
                paper[y][x] += spot;
            }
        }
    } else {
        let mut right_half = vec![];
        for row in paper.into_iter() {
            row.remove(fold[1]);
            let right_row: Vec<usize> = row.drain(fold[1]..).collect();
            right_half.push(right_row);
        }
        for row in &mut right_half {
            row.reverse();
        }
        for (y, row) in right_half.iter().enumerate() {
            for (x, spot) in row.iter().enumerate() {
                paper[y][x] += spot;
            }
        }
    }
}

fn part2(input: &Vec<InputType>) -> u64 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"fold along (.)=(\d+)").unwrap();
    }

    //let mut paper = vec![vec![0; 15]; 15];
    let mut paper = vec![vec![0; 1500]; 1500];
    let mut folds = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for line in input {
        if !line.starts_with("fold along") {
            let coords: Vec<usize> = line.split(",").map(|number| number.parse::<usize>().unwrap()).collect();
            if max_y < coords[1] {
                max_y = coords[1];
            }
            if max_x < coords[0] {
                max_x = coords[0];
            }
            paper[coords[1]][coords[0]] = 1;
        } else {
            RE.captures(line).and_then(|captured| {
                let value = captured[2].parse::<usize>().unwrap();
                let idx = if captured[1] == *"y" { 
                    0
                } else {
                    1
                };
                folds.push(vec![idx, value]);
            Some(true)
            }).unwrap();
        }
    }
    paper.drain(max_y+1..);
    for row in &mut paper {
        row.drain(max_x+1..);
    }
    for fold in folds {
        fold_it(fold, &mut paper);
    }
    for row in paper {
        for dot in row {
            if dot > 2 {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day13_part1_output() {
        let input = parse_input(&get_input());
        assert_eq!(4885, part1(&input));
    }

    #[test]
    fn day13_part2_output() {
        let input = parse_input(&get_input());
        assert_eq!(117095, part2(&input));
    }
}
