use std::fs;

pub fn load_input_for_day(day: &str) -> Vec<String> {
  fs::read_to_string(format!("src/days/day{}-input.txt", day))
    .unwrap()
    .lines()
    .map(|line| String::from(line))
    .collect()
}
