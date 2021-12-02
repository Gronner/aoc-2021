mod day_template;
mod day1;
mod day2;
mod day3;

pub fn get_days() -> Vec<fn()> {
    vec![
        day_template::run_day,
        day1::run_day,
        day2::run_day,
        day3::run_day,
    ]
}
