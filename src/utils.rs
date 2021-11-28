use const_format::formatcp;

pub const fn get_input(day: u32) -> Vec<String> {
    include_str!(formatcp!("input/input{}.txt", day))
        .split("\n")
        .collect::<Vec<_>>()
}
