use aoc_downloader::download_day;

const DAY: u32 = 1;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day((DAY) as u32, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{}.txt", DAY)).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

fn parse_input(input: Vec<String>) -> Vec<u32> {
    input
        .iter()
        .map(|v| v.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(input);
    println!("Running day {}:\n\tPart1 {}\n\tPart2 {}", DAY, part1(&input), part2(&input));
}

fn part1(input: &Vec<u32>) -> usize {
    input.windows(2)
        .filter(|w: &&[u32]| w[1] > w[0])
        .count()
}

fn part2(input: &Vec<u32>) -> usize {
    input.windows(3)
        .map(|w| w.iter().sum())
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|w: &&[u32]| w[1] > w[0])
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day0_part1_output() {
        let input = parse_input(get_input());
        assert_eq!(1215, part1(&input));
    }

    #[test]
    fn day0_part1_testcase1() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260,263];
        assert_eq!(7, part1(&input));
    }

    #[test]
    fn day0_part2_output() {
        let input = parse_input(get_input());
        assert_eq!(1150, part2(&input));
    }
}
