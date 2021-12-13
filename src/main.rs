use aoc_2021;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if 2 == args.len() {
        aoc_2021::get_days()[args[1].parse::<usize>().unwrap()]();
    } else {
        for call in aoc_2021::get_days() {
            call();
        }
    }
}
