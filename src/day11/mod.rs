use aoc_downloader::download_day;

const DAY: u32 = 11;
type InputType = Vec<u32>;

fn get_input() -> String {
    download_day((DAY) as u32, "input").unwrap();
    std::fs::read_to_string(format!("input/input{}.txt", DAY)).unwrap()
}

fn parse_input(input: &str) -> Vec<InputType> {
    input.lines()
        .map(|line| line.chars().
            map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>())
        .collect::<Vec<_>>()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(&input);
    println!("Running day {}:\n\tPart2 {}\n\tPart2 {}", DAY, part1(&input), part2(&input));
}

fn get_neighbours(current_pos: (usize, usize), input: &Vec<InputType>) -> Vec<(usize, usize)> {
    lazy_static!{
        static ref OFFSETS: Vec<(isize, isize)> = vec![
            (0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1)
        ];
    }
    let max_x = input[0].len() as isize;
    let max_y = input.len() as isize;
    let mut neighbours = vec![];
    for offset in OFFSETS.iter() {
        let new_pos = (current_pos.0 as isize + offset.0,
            current_pos.1 as isize + offset.1);
        if new_pos.0 < 0 || new_pos.0 >= max_x || new_pos.1 < 0 || new_pos.1 >= max_y {
            continue;
        }
        neighbours.push((new_pos.0 as usize, new_pos.1 as usize));
    }

    neighbours
}

fn part1(input: &Vec<InputType>) -> u64 {
    let mut input = input.clone();
    let mut flashes = 0;
    let rounds = 100;
    for _ in 0..rounds {
        let mut flashers = Vec::new();
        for (y, row) in input.clone().iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                input[y][x] += 1;
                if input[y][x] > 9 {
                    flashers.push((y, x));
                    flashes += 1;
                }
            }
        }

        let mut flashed = Vec::new();
        while !flashers.is_empty() {
            let flasher = flashers.pop().unwrap();
            let neighbours = get_neighbours(flasher, &input);
            for neighbour in neighbours {
                input[neighbour.0][neighbour.1] += 1;
                if input[neighbour.0][neighbour.1] == 10 {
                    flashers.push(neighbour);
                    flashes += 1;
                }
            }
            flashed.push(flasher);
        }

        for flasher in flashed {
            input[flasher.0][flasher.1] = 0;
        }
    }
    flashes
}

fn part2(input: &Vec<InputType>) -> u64 {
    let mut input = input.clone();
    let octopuses_max = input.len() * input[0].len();
    let mut rounds = 0;
    loop {
        let mut flashers = Vec::new();
        for (y, row) in input.clone().iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                input[y][x] += 1;
                if input[y][x] > 9 {
                    flashers.push((y, x));
                }
            }
        }

        let mut flashed = Vec::new();
        while !flashers.is_empty() {
            let flasher = flashers.pop().unwrap();
            let neighbours = get_neighbours(flasher, &input);
            for neighbour in neighbours {
                input[neighbour.0][neighbour.1] += 1;
                if input[neighbour.0][neighbour.1] == 10 {
                    flashers.push(neighbour);
                }
            }
            flashed.push(flasher);
        }

        for flasher in &flashed {
            input[flasher.0][flasher.1] = 0;
        }
        if flashed.len() == octopuses_max {
            break;
        }
        rounds += 1;
    }
    rounds + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day11_part1_output() {
        let input = parse_input(&get_input());
        assert_eq!(392043, part1(&input));
    }

    #[test]
    fn day11_part2_output() {
        let input = parse_input(&get_input());
        assert_eq!(1605968119, part2(&input));
    }
}
