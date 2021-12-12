use std::str::FromStr;
use aoc_2021;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Ok(selection) = usize::from_str(&args[1]) {
        aoc_2021::get_days()[selection]();
    } else {
        for call in aoc_2021::get_days() {
            call();
        }
    }
}
