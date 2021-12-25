use aoc_downloader::download_day;

const DAY: u32 = 25;
type InputType = Vec<Vec<char>>;

fn get_input() -> String {
    download_day((DAY) as u32, "input").unwrap();
    std::fs::read_to_string(format!("input/input{}.txt", DAY)).unwrap()
}

fn parse_input(input: &str) -> InputType {
    input.lines()
        .filter(|line| *line != "")
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(&input);
    println!("Running day {}:\n\tPart 1: {}\n\tPart 2: {}", DAY, part1(&input), part2(&input));
}

fn part1(input: &InputType) -> u64{
    let mut input = input.clone();

    let max_x = input[0].len();
    let max_y = input.len();

    let mut rounds = 0;
    let mut moved = true;
    while moved {
        moved = false;
        let mut next_step = input.clone();
        for (y, row) in input.iter().enumerate() {
            for (x, spot) in row.iter().enumerate() {
                if *spot == '>' {
                    if input[y][(x+1)%max_x] == '.' {
                        next_step[y][(x+1)%max_x] = '>';
                        next_step[y][x] = '.';
                        moved = true;
                    }
                }
            }
        }
        input = next_step.clone();
        for (y, row) in next_step.iter().enumerate() {
            for (x, spot) in row.iter().enumerate() {
                if *spot == 'v' {
                    if next_step[(y+1) % max_y][x] == '.' {
                        input[(y+1) % max_y][x] = 'v';
                        input[y][x] = '.';
                        moved = true;
                    }
                }
            }
        }

        rounds += 1;
    }
    rounds
}


fn part2(_input: &InputType) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day25_part1_output() {
        let input = parse_input(&get_input());
        assert_eq!(353, part1(&input));
    }

    #[test]
    fn day25_part2_output() {
        let input = parse_input(&get_input());
        assert_eq!(0, part2(&input));
    }
}
