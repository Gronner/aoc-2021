use std::fmt;
use num::Integer;
use itertools::Itertools;

use aoc_downloader::download_day;

const DAY: u32 = 18;
type InputType = Vec<Pair>;

#[derive(Clone, PartialEq)]
enum Pair {
    Number(u64),
    Pair(Box<Pair>, Box<Pair>),
}

impl std::ops::Add for Pair {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::Pair(Box::new(self), Box::new(other))
    }
}

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(num) => write!(f, "{}", num),
            Self::Pair(left, right) => write!(f, "[{}, {}]", left, right),
        }
    }
}

impl fmt::Debug for Pair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(num) => write!(f, "{}", num),
            Self::Pair(left, right) => write!(f, "[{}, {}]", left, right),
        }
    }
}

impl Pair {
    pub fn from_str(input: &mut String) -> Pair {
        let left_start = input.pop().unwrap();
        let left;
        if '[' == left_start {
            left = Self::from_str(input);
        } else {
            left = Self::Number(left_start.to_digit(10).unwrap() as u64);
        }
        input.pop().unwrap(); // Discard delimiter
        let right_start = input.pop().unwrap();
        let right;
        if '[' == right_start {
            right = Self::from_str(input);
        } else {
            right = Self::Number(right_start.to_digit(10).unwrap() as u64);
        }
        input.pop().unwrap(); // Discard closing
        Self::Pair(Box::new(left), Box::new(right))
    }

    pub fn get_magnitude(&self) -> u64 {
        match self {
            Self::Number(num) => *num,
            Self::Pair(left, right) => 3 * (*left).get_magnitude() + 2 * (*right).get_magnitude(),
        }
    }

    fn add_left(&self, value: Option<u64>) -> Self {
        match value {
            None => self.clone(),
            Some(val) => match self.clone() {
                Pair::Number(num) => Pair::Number(num + val),
                Pair::Pair(left, right) => left.clone().add_left(value) + *right,
            },
        }
    }

    fn add_right(&self, value: Option<u64>) -> Self {
        match value {
            None => self.clone(),
            Some(val) => match self.clone() {
                Pair::Number(num) => Pair::Number(num + val),
                Pair::Pair(left, right) => *left + right.clone().add_right(value),
            }
        }

    }

    fn explode(&mut self, depth: u64) -> (Pair, Option<u64>, Option<u64>, bool) {
        match self {
            Pair::Number(_) => (self.clone(), None, None, false),
            Pair::Pair(left, right) => {
                if depth >= 4 {
                    match (*left.clone(), *right.clone()) {
                        (Pair::Number(left_val), Pair::Number(right_val)) => {
                            return (Pair::Number(0), Some(left_val), Some(right_val), true);
                        }
                        _ => panic!("Unexpected values for exploding Pair"),
                    }
                }
                let (next_pair_left, left_explosion, right_explosion, exploded) = left.explode(depth + 1);
                if exploded {
                    return (next_pair_left + right.clone().add_left(right_explosion),
                        left_explosion,
                        None,
                        true,
                    );
                } else {
                    let (next_pair_right, left_explosion, right_explosion, exploded) = right.explode(depth + 1);
                    return (next_pair_left.add_right(left_explosion) + next_pair_right,
                        None,
                        right_explosion,
                        exploded,
                    );
                }
            }
        }
    }

    fn split(&mut self) -> (Pair, bool) {
        match self {
            Pair::Number(num) => {
                if *num >= 10 {
                    return (
                            Pair::Number(num.div_floor(&2)) +
                            Pair::Number(num.div_ceil(&2))
                        ,
                        true,
                    )
                } else {
                    (self.clone(), false)
                }
            },
            Pair::Pair(left, right) => {
                let (new_left, splitted) = left.split();
                if splitted {
                    return (new_left + *right.clone(), true);
                } else {
                    let (new_right, splitted) = right.split();
                    return (new_left + new_right, splitted)
                }
            },
        }
    }

    fn reduce(&mut self) -> Self {
        loop {
            let (next_pair, _, _, exploded) = self.explode(0);
            *self = next_pair;
            if exploded {
                continue;
            }

            let (next_pair, splitted) = self.split();
            *self = next_pair;
            if splitted {
                continue;
            }
            break;
        }
        self.clone()
    }
}

fn get_input() -> String {
    download_day((DAY) as u32, "input").unwrap();
    std::fs::read_to_string(format!("input/input{}.txt", DAY)).unwrap()
}

fn parse_input(input: &str) -> InputType {
    input.lines()
        .filter(|&line| line != "")
        .map(|line| {
            let input = line[1..].to_string();
            Pair::from_str(&mut input.chars().rev().collect())
        })
        .collect()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(&input);
    println!("Running day {}:\n\tPart 1 {}\n\tPart 2: {}", DAY, part1(&input), part2(&input));
}

fn part1(input: &InputType) -> u64 {
    let input = input.clone();

    input.into_iter()
        .reduce(|current, next| {
            (current + next).reduce()
        })
        .unwrap()
        .get_magnitude()
}

fn part2(input: &InputType) -> u64 {
    let input = input.clone();

    input.into_iter()
        .permutations(2)
        .map(|permutations| (permutations[0].clone() + permutations[1].clone()).reduce())
        .max_by(|a, b| a.get_magnitude().cmp(&b.get_magnitude()))
        .unwrap()
        .get_magnitude()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_works() {
        let input = "[[9,1],[1,9]]"[1..].to_string();
        let pair = Pair::from_str(&mut input.chars().rev().collect());

        let expected_pair = Pair::Pair(
            Box::new(Pair::Pair(
                Box::new(Pair::Number(9)),
                Box::new(Pair::Number(1)),
            )),
            Box::new(Pair::Pair(
                Box::new(Pair::Number(1)),
                Box::new(Pair::Number(9)),
            )),
        );

        assert_eq!(expected_pair, pair);
    }

    #[test]
    fn magnitude_example1() {
        let input = "[[9,1],[1,9]]"[1..].to_string();
        let pair = Pair::from_str(&mut input.chars().rev().collect());

        assert_eq!(129, pair.get_magnitude());
    }

    #[test]
    fn magnitude_example2() {
        let input = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"[1..].to_string();
        let pair = Pair::from_str(&mut input.chars().rev().collect());

        assert_eq!(1384, pair.get_magnitude());
    }

    #[test]
    fn magnitude_example3() {
        let input = "[[[[1,1],[2,2]],[3,3]],[4,4]]"[1..].to_string();
        let pair = Pair::from_str(&mut input.chars().rev().collect());

        assert_eq!(445, pair.get_magnitude());
    }

    #[test]
    fn example_1_works() {
        let input = "[[[[4,3],4],4],[7,[[8,4],9]]]"[1..].to_string();
        let pair_a = Pair::from_str(&mut input.chars().rev().collect());

        let input = "[1,1]"[1..].to_string();
        let pair_b = Pair::from_str(&mut input.chars().rev().collect());

        let mut pair = pair_a + pair_b;


        let input = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"[1..].to_string();
        let expected_pair = Pair::from_str(&mut input.chars().rev().collect());

        let pair = pair.reduce();
        assert_eq!(expected_pair, pair);
    }

    #[test]
    fn reduce_explode_works_0() {
        let input = "[[[[[9,8],1],2],3],4]"[1..].to_string();
        let mut pair = Pair::from_str(&mut input.chars().rev().collect());
        
        let input = "[[[[0,9],2],3],4]"[1..].to_string();
        let expected_pair = Pair::from_str(&mut input.chars().rev().collect());

        let pair = pair.reduce();
        assert_eq!(expected_pair, pair);
    }

    #[test]
    fn reduce_explode_works_1() {
        let input = "[7,[6,[5,[4,[3,2]]]]]"[1..].to_string();
        let mut pair = Pair::from_str(&mut input.chars().rev().collect());
        
        let input = "[7,[6,[5,[7,0]]]]"[1..].to_string();
        let expected_pair = Pair::from_str(&mut input.chars().rev().collect());

        let pair = pair.reduce();
        assert_eq!(expected_pair, pair);
    }

    #[test]
    fn reduce_explode_works_2() {
        let input = "[[6,[5,[4,[3,2]]]],1]"[1..].to_string();
        let mut pair = Pair::from_str(&mut input.chars().rev().collect());
        
        let input = "[[6,[5,[7,0]]],3]"[1..].to_string();
        let expected_pair = Pair::from_str(&mut input.chars().rev().collect());

        let pair = pair.reduce();
        assert_eq!(expected_pair, pair);
    }

    #[test]
    fn reduce_explode_works_3() {
        let input = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"[1..].to_string();
        let mut pair = Pair::from_str(&mut input.chars().rev().collect());
        
        let input = "[[3,[2,[8,0]]],[9,[5,[7,0]]]]"[1..].to_string();
        let expected_pair = Pair::from_str(&mut input.chars().rev().collect());

        let pair = pair.reduce();
        assert_eq!(expected_pair, pair);
    }

    #[test]
    fn reduce_explode_works_4() {
        let input = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"[1..].to_string();
        let mut pair = Pair::from_str(&mut input.chars().rev().collect());
        
        let input = "[[3,[2,[8,0]]],[9,[5,[7,0]]]]"[1..].to_string();
        let expected_pair = Pair::from_str(&mut input.chars().rev().collect());

        let pair = pair.reduce();
        assert_eq!(expected_pair, pair);
    }

    #[test]
    fn addition_works() {
        let input = "[1,1]"[1..].to_string();
        let pair_a = Pair::from_str(&mut input.chars().rev().collect());
        
        let input = "[2,2]"[1..].to_string();
        let pair_b = Pair::from_str(&mut input.chars().rev().collect());
        
        let input = "[[1,1],[2,2]]"[1..].to_string();
        let expected_pair = Pair::from_str(&mut input.chars().rev().collect());

        assert_eq!(expected_pair, pair_a + pair_b);
    }

    #[test]
    fn day18_part1_output() {
        let input = parse_input(&get_input());
        assert_eq!(4033, part1(&input));
    }

    #[test]
    fn day18_part2_output() {
        let input = parse_input(&get_input());
        assert_eq!(4864, part2(&input));
    }
}
