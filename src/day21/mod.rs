use aoc_downloader::download_day;
use cached::proc_macro::cached;
use regex::Regex;

const DAY: u32 = 21;
type InputType = Vec<u64>;

fn get_input() -> String {
    download_day((DAY) as u32, "input").unwrap();
    std::fs::read_to_string(format!("input/input{}.txt", DAY)).unwrap()
}

fn parse_input(input: &str) -> InputType {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"Player \d starting position: (\d+)").unwrap();
    }

    input.lines()
        .filter(|line| *line != "")
        .map(|line| RE.captures(line).and_then(|captured|
                Some(captured[1].parse::<u64>().unwrap())).unwrap())
        .collect()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(&input);
    println!("Running day {}:\n\tPart 1 {}\n\tPart 2: {}", DAY, part1(&input), part2(&input));
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Player {
    pos: u64,
    score: u64,
}

#[derive(Debug)]
struct Dice {
    rolled: u64,
    last_roll: u64,
}

struct QuantumDice {
}

impl QuantumDice {
    pub fn roll() -> [u64; 3] {
        [1, 2, 3]
    }
}

impl Dice {
    pub fn roll(&mut self) -> u64 {
        self.rolled += 1;
        self.last_roll += 1;
        self.last_roll
    }
}

impl Player {
    pub fn has_won(&self) -> bool {
        self.score >= 1000
    }

    /*
    pub fn has_quantum_won(&self) -> bool {
        self.score >= 21
    }
    */

    pub fn take_move(&mut self, eyes: u64) {
        self.pos = (self.pos + eyes) % 10;
        self.score += self.pos + 1;
    }
}

fn part1(input: &InputType) -> u64 {
    let mut players = vec![
        Player {
            pos: input[0] - 1,
            score: 0,
        },
        Player {
            pos: input[1] - 1,
            score: 0,
        }
    ];

    let mut dice = Dice { rolled: 0, last_roll: 0 };

    let mut current_turn = 0;

    while !players[0].has_won() && !players[1].has_won() {
        let eyes = dice.roll() + dice.roll() + dice.roll();
        players[current_turn].take_move(eyes);
        current_turn = (current_turn + 1) % 2;
    }

    if players[0].has_won() {
        players[1].score * dice.rolled
    } else {
        players[0].score * dice.rolled
    }
}

fn has_quantum_won(score: u64) -> bool {
    score >= 21
}

pub fn take_move(pos: u64, eyes: u64, score:u64 ) -> (u64, u64) {
    let new_pos = (pos + eyes) % 10;
    (new_pos, score + new_pos + 1)
}

//fn play_round(player_a: Player, player_b: Player) -> (u64, u64) { This should work, but doesn't
#[cached]
fn play_round(pos_a: u64, score_a: u64, pos_b: u64, score_b: u64) -> (u64, u64) {
    if has_quantum_won(score_a) {
        return (1, 0);
    } 
    if has_quantum_won(score_b) {
        return(0, 1);
    } 
    let mut win_count = (0_u64, 0_u64);
    for dice_roll_1 in QuantumDice::roll() {
        for dice_roll_2 in QuantumDice::roll() {
            for dice_roll_3 in QuantumDice::roll() {
                let (new_pos_a, new_score_a) = take_move(pos_a, dice_roll_1 + dice_roll_2 + dice_roll_3, score_a);

                let (player_a_wins, player_b_wins) = play_round(pos_b, score_b, new_pos_a, new_score_a);

                win_count = (win_count.0 + player_b_wins, win_count.1 + player_a_wins);
            }
        }
    }
    win_count
}

fn part2(input: &InputType) -> u64 {
    let (player_a_wins, player_b_wins) = play_round(input[0]-1, 0, input[1]-1, 0);
    std::cmp::max(player_a_wins, player_b_wins)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day21_part1_output() {
        let input = parse_input(&get_input());
        assert_eq!(503478, part1(&input));
    }

    #[test]
    fn day21_part2_output() {
        let input = parse_input(&get_input());
        assert_eq!(716241959649754, part2(&input));
    }
}
