#[macro_use]
extern crate lazy_static;

mod day_template;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod utils;

pub fn get_days() -> Vec<fn()> {
    vec![
        day_template::run_day,
        day1::run_day,
        day2::run_day,
        day3::run_day,
        day4::run_day,
        day5::run_day,
        day6::run_day,
    ]
}
