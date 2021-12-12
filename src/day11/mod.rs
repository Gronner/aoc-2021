use aoc_downloader::download_day;

const DAY: u32 = 11;
type InputType = Vec<u32>;
type Coordinate = (usize, usize);

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
    println!("Running day {}:\n\tPart 1 {}\n\tPart 2 {}", DAY, part1(&input), part2(&input));
}

fn get_neighbours(current_pos: Coordinate, input: &Vec<InputType>) -> Vec<Coordinate> {
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

fn find_initial_flashers(input: &mut Vec<InputType>) -> Vec<Coordinate> {
    let mut flashers = Vec::new();
    for (y, row) in input.clone().iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            input[y][x] += 1;
            if input[y][x] > 9 {
                flashers.push((y, x));
            }
        }
    }
    flashers
}

fn get_all_flashers_this_cycle(input: &mut Vec<InputType>, flashers: &mut Vec<Coordinate>) -> Vec<Coordinate> {
    let mut flashed = Vec::new();
    while let Some(flasher) = flashers.pop() {
        let neighbours = get_neighbours(flasher, &input);
        for neighbour in neighbours {
            input[neighbour.0][neighbour.1] += 1;
            if input[neighbour.0][neighbour.1] == 10 {
                flashers.push(neighbour);
            }
        }
        flashed.push(flasher);
    }
    flashed
}

fn cycle_of_life(input: &mut Vec<InputType>) -> usize {
    let mut flashers = find_initial_flashers(input);
    let flashed = get_all_flashers_this_cycle(input, &mut flashers);

    for flasher in &flashed {
        input[flasher.0][flasher.1] = 0;
    }
    flashed.len()
}

fn part1(input: &Vec<InputType>) -> usize {
    let mut input = input.clone();
    let mut flashes = 0;
    let rounds = 100;
    for _ in 0..rounds {
        flashes += cycle_of_life(&mut input);
    }
    flashes
}

fn part2(input: &Vec<InputType>) -> u64 {
    let mut input = input.clone();
    let octopuses_max = input.len() * input[0].len();
    let mut rounds = 0;
    loop {
        let flashed_this_cycle = cycle_of_life(&mut input);
        rounds += 1;
        if octopuses_max == flashed_this_cycle {
            break;
        }
    }
    rounds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day11_part1_output() {
        let input = parse_input(&get_input());
        assert_eq!(1739, part1(&input));
    }

    #[test]
    fn day11_part2_output() {
        let input = parse_input(&get_input());
        assert_eq!(324, part2(&input));
    }
}
