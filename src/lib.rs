mod day_template;

pub fn get_days() -> Vec<fn()> {
    vec![
        day_template::run_day,
    ]
}
