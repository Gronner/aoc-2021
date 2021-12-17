use aoc_downloader::download_day;
use regex::Regex;

const DAY: u32 = 17;
type InputType = TargetArea;
type Coords = (i64, i64);

struct TargetArea {
    x_start: i64,
    x_end: i64,
    y_start: i64,
    y_end: i64,
}

#[derive(Debug)]
pub struct Probe {
    position: Coords,
    velocity: Coords,
}

enum Failure {
    NotHit,
}

impl Probe {
    fn step(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;

        self.velocity.0 = std::cmp::max(self.velocity.0 - 1, 0);
        self.velocity.1 -= 1;
    }

    fn simulate(&mut self, target: &TargetArea) -> Result<i64, Failure> {
        let mut max_height = 0;
        loop  {
            self.step();
            max_height = std::cmp::max(self.position.1, max_height);
            if self.hit(&target) {
                return Ok(max_height);
            }
            if self.velocity.1 < 0 && self.position.1 < target.y_start {
                return Err(Failure::NotHit);
            }
        }
    }

    pub fn hit(&self, target: &TargetArea) -> bool {
        target.x_start <= self.position.0 && self.position.0 <= target.x_end &&
            target.y_start <= self.position.1 && self.position.1 <= target.y_end
    }
}

fn get_input() -> String {
    download_day((DAY) as u32, "input").unwrap();
    std::fs::read_to_string(format!("input/input{}.txt", DAY)).unwrap()
}

fn parse_input(input: &str) -> InputType {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)\n").unwrap();
    }
    RE.captures(input).and_then(|captured| {
        Some(TargetArea {
            x_start: captured[1].parse::<i64>().unwrap(),
            x_end: captured[2].parse::<i64>().unwrap(),
            y_start: captured[3].parse::<i64>().unwrap(),
            y_end: captured[4].parse::<i64>().unwrap(),
        })
    }).unwrap()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(&input);
    println!("Running day {}:\n\tPart 1 {}\n\tPart 2: {}", DAY, part1(&input), part2(&input));
}

fn part1(input: &InputType) -> i64 {
    let mut max_y = 0;
    for x_vel in 0..input.x_end {
        for y_vel in 0..1000 {
            let mut probe = Probe {
                position: (0, 0),
                velocity: (x_vel, y_vel),
            };
            if let Ok(y) = probe.simulate(&input) {
                max_y = std::cmp::max(max_y, y);
            }
        }
    }
    max_y
}

fn part2(input: &InputType) -> u64 {
    let mut works = 0;
    for x_vel in 0..=input.x_end {
        for y_vel in input.y_start..=1500 {
            let mut probe = Probe {
                position: (0, 0),
                velocity: (x_vel, y_vel),
            };
            if probe.simulate(input).is_ok() {
                works += 1;
            }
        }
    }
    works
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day16_part1_output() {
        let input = parse_input(&get_input());
        assert_eq!(873, part1(&input));
    }

    #[test]
    fn day16_part2_output() {
        let input = parse_input(&get_input());
        assert_eq!(402817863665, part2(&input));
    }
}
