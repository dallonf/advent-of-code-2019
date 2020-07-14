use std::fs;

pub fn lines_for_day(day: &str) -> Vec<String> {
  string_for_day(day)
    .lines()
    .map(|line| String::from(line))
    .collect()
}

pub fn string_for_day(day: &str) -> String {
  fs::read_to_string(format!("src/days/day{}-input.txt", day))
    .unwrap()
    .trim()
    .into()
}
