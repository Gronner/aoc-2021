#![feature(int_roundings)]
#[macro_use]
extern crate lazy_static;

mod day_template;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
pub mod utils;

pub fn get_days() -> Vec<fn()> {
    vec![
        day_template::run_day,
        day1::run_day,
        day2::run_day,
        day3::run_day,
        day4::run_day,
        day5::run_day,
        day6::run_day,
        day7::run_day,
        day8::run_day,
        day9::run_day,
        day10::run_day,
        day11::run_day,
        day12::run_day,
        day13::run_day,
        day14::run_day,
        day15::run_day,
        day16::run_day,
        day17::run_day,
        day18::run_day,
    ]
}
