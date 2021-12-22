use aoc_downloader::download_day;
use regex::Regex;

const DAY: u32 = 22;
type InputType = Vec<Cube>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Cube {
    x_start: isize,
    x_end: isize,
    y_start: isize,
    y_end: isize,
    z_start: isize,
    z_end: isize,
    state: bool,
}

impl Cube { 
    fn intersect(&self, other: &Self) -> bool {
        other.x_end > self.x_start && other.x_start < self.x_end &&
        other.y_end > self.y_start && other.y_start < self.y_end &&
        other.z_end > self.z_start && other.z_start < self.z_end
    }

    fn split_cube(&mut self, other: &Self) -> Vec<Self> {
        let mut new_cubes = Vec::new();

        if self.x_start < other.x_start {
            new_cubes.push(Cube {
                x_start: self.x_start,
                x_end: other.x_start,
                y_start: self.y_start,
                y_end: self.y_end,
                z_start: self.z_start,
                z_end: self.z_end,
                state: self.state,
            });
            self.x_start = other.x_start;
        }
        if self.x_end > other.x_end {
            new_cubes.push(Cube {
                x_start: other.x_end,
                x_end: self.x_end,
                y_start: self.y_start,
                y_end: self.y_end,
                z_start: self.z_start,
                z_end: self.z_end,
                state: self.state,
            });
            self.x_end = other.x_end;
        }
        if self.y_start < other.y_start {
            new_cubes.push(Cube {
                x_start: self.x_start,
                x_end: self.x_end,
                y_start: self.y_start,
                y_end: other.y_start,
                z_start: self.z_start,
                z_end: self.z_end,
                state: self.state,
            });
            self.y_start= other.y_start;
        }
        if self.y_end > other.y_end{
            new_cubes.push(Cube {
                x_start: self.x_start,
                x_end: self.x_end,
                y_start: other.y_end,
                y_end: self.y_end,
                z_start: self.z_start,
                z_end: self.z_end,
                state: self.state,
            });
            self.y_end = other.y_end;
        }
        if self.z_start < other.z_start {
            new_cubes.push(Cube {
                x_start: self.x_start,
                x_end: self.x_end,
                y_start: self.y_start,
                y_end: self.y_end,
                z_start: self.z_start,
                z_end: other.z_start,
                state: self.state,
            });
            self.z_start= other.z_start;
        }
        if self.z_end > other.z_end{
            new_cubes.push(Cube {
                x_start: self.x_start,
                x_end: self.x_end,
                y_start: self.y_start,
                y_end: self.y_end,
                z_start: other.z_end,
                z_end: self.z_end,
                state: self.state,
            });
            self.z_end = other.z_end;
        }

        new_cubes
    }

    fn get_size(&self) -> isize {
        (self.x_end - self.x_start) * (self.y_end - self.y_start) * (self.z_end - self.z_start)
    }
}

fn get_input() -> String {
    download_day((DAY) as u32, "input").unwrap();
    std::fs::read_to_string(format!("input/input{}.txt", DAY)).unwrap()
}

fn parse_input(input: &str) -> InputType {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)").unwrap();
    }

    input.lines()
        .filter(|line| *line != "")
        .map(|line| RE.captures(line).and_then(|captured| {
                let state = if captured[1] == *"on" { true } else { false };
                let x = (captured[2].parse::<isize>().unwrap(), captured[3].parse::<isize>().unwrap() + 1);
                let y = (captured[4].parse::<isize>().unwrap(), captured[5].parse::<isize>().unwrap() + 1);
                let z = (captured[6].parse::<isize>().unwrap(), captured[7].parse::<isize>().unwrap() + 1);
                Some(Cube {
                    x_start: x.0,
                    x_end: x.1,
                    y_start: y.0,
                    y_end: y.1,
                    z_start: z.0,
                    z_end: z.1,
                    state
                })
        }).unwrap())
        .collect()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(&input);
    println!("Running day {}:\n\tPart 1: {}\n\tPart 2: {}", DAY, part1(&input), part2(&input));
}

fn part1(input: &InputType) -> isize {
    let mut cubes: Vec<Cube> = Vec::new();

    for in_cube in input[..20].iter() {
        let mut new_cubes: Vec<Cube> = Vec::new();
        for cube in &mut cubes {
            if !cube.intersect(in_cube) {
                new_cubes.push(*cube);
            } else {
                new_cubes.append(&mut cube.split_cube(in_cube));
            }
        }
        new_cubes.push(*in_cube);
        cubes = new_cubes.clone();
    }

    let mut turned_on = 0;
    for cube in cubes {
        if cube.state {
            turned_on += cube.get_size();
        }
    }
    turned_on
}

fn part2(input: &InputType) -> isize {
    let mut cubes: Vec<Cube> = Vec::new();

    for in_cube in input {
        let mut new_cubes: Vec<Cube> = Vec::new();
        for cube in &mut cubes {
            if !cube.intersect(in_cube) {
                new_cubes.push(*cube);
            } else {
                new_cubes.append(&mut cube.split_cube(in_cube));
            }
        }
        new_cubes.push(*in_cube);
        cubes = new_cubes.clone();
    }

    let mut turned_on = 0;
    for cube in cubes {
        if cube.state {
            turned_on += cube.get_size();
        }
    }
    turned_on
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day22_part1_output() {
        let input = parse_input(&get_input());
        assert_eq!(602574, part1(&input));
    }

    #[test]
    fn day22_part2_output() {
        let input = parse_input(&get_input());
        assert_eq!(1288707160324706, part2(&input));
    }
}
