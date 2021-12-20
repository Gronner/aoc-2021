use aoc_downloader::download_day;
use std::collections::HashSet;

const DAY: u32 = 20;
type InputType = Vec<String>;
type Coords = (isize, isize);

fn get_input() -> String {
    download_day((DAY) as u32, "input").unwrap();
    std::fs::read_to_string(format!("input/input{}.txt", DAY)).unwrap()
}

fn parse_input(input: &str) -> InputType {
    input.lines()
        .map(|line| line.to_string())
        .collect()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(&input);
    println!("Running day {}:\n\tPart 1 {}\n\tPart 2: {}", DAY, part1(&input), part2(&input));
}

struct Image {
    pixel: HashSet<(isize, isize)>,
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    empty_space: bool,
}

impl Image {

    pub fn new(image: HashSet<Coords>, empty_space: bool) -> Self {
        let min_y = image.iter().map(|pixel| pixel.0).min().unwrap();
        let max_y = image.iter().map(|pixel| pixel.0).max().unwrap();
        let min_x = image.iter().map(|pixel| pixel.1).min().unwrap();
        let max_x = image.iter().map(|pixel| pixel.1).max().unwrap();

        Image {
            pixel: image,
            min_x,
            max_x,
            min_y,
            max_y,
            empty_space,
        }
    }

    pub fn get_pixel(&self, y: isize, x: isize) -> usize {
        if y < self.min_y || y > self.max_y || x < self.min_x || x > self.max_x {
            self.empty_space as usize
        } else {
            self.pixel.contains(&(y, x)) as usize
        }
    }
}

fn get_pointer(image: &Image, y: isize, x: isize) -> usize {
    lazy_static! {
        static ref OFFSETS: Vec<(isize, isize)> = vec![
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),  (0, 0),  (0, 1), 
            (1, -1),  (1, 0),  (1, 1),
        ];
    }
    let mut pointer = 0;
    for (idx, offset) in OFFSETS.iter().enumerate() {
        pointer |= image.get_pixel(y + offset.0, x + offset.1) << (8 - idx)
    }
    pointer as usize

}

fn enhance(mut image: Image, algorithm: &Vec<bool>, rounds: usize) -> Image {
    for _ in 0..rounds {
        let mut new_image: HashSet<Coords> = HashSet::new();

        for y in (image.min_y-1)..=(image.max_y+1) {
            for x in (image.min_x-1)..=(image.max_x+1) {
                let pointer = get_pointer(&image, y, x);

                if algorithm[pointer] {
                    new_image.insert((y, x));
                }
            }
        }

        let new_empty = if image.empty_space { algorithm[511] } else { algorithm[0] };
        image = Image::new(new_image, new_empty);
    }
    image
}


fn part1(input: &InputType) -> usize {
    let algorithm: Vec<bool> = input[0].chars()
        .map(|c| if c == '#' { true } else { false})
        .collect();

    let original_image: Vec<Vec<char>> = input.iter()
        .skip(2)
        .map(|line| line.chars()
            .collect())
        .collect();

    let mut image: HashSet<Coords> = HashSet::new();
    for (y, row) in original_image.iter().enumerate() {
        for (x, pixel) in row.iter().enumerate() {
            if *pixel == '#' {
                image.insert((y as isize, x as isize));
            }
        }
    }

    let image = Image::new(image, false);

    let image = enhance(image, &algorithm, 2);

    image.pixel.len()
}

fn part2(input: &InputType) -> usize {
    let algorithm: Vec<bool> = input[0].chars()
        .map(|c| if c == '#' { true } else { false})
        .collect();

    let original_image: Vec<Vec<char>> = input.iter()
        .skip(2)
        .map(|line| line.chars()
            .collect())
        .collect();

    let mut image: HashSet<Coords> = HashSet::new();
    for (y, row) in original_image.iter().enumerate() {
        for (x, pixel) in row.iter().enumerate() {
            if *pixel == '#' {
                image.insert((y as isize, x as isize));
            }
        }
    }

    let image = Image::new(image, false);

    let image = enhance(image, &algorithm, 50);

    image.pixel.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn day18_part1_output() {
        let input = parse_input(&get_input());
        assert_eq!(4033, part1(&input));
    }

    #[ignore]
    #[test]
    fn day18_part2_output() {
        let input = parse_input(&get_input());
        assert_eq!(4864, part2(&input));
    }
}
