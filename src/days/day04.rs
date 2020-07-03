// Day 4: Secure Container

use crate::prelude::*;

pub fn is_valid_password(input: u32) -> bool {
  let input_as_str = format!("{}", input);
  let digits: Vec<u32> = input_as_str
    .chars()
    .map(|x| x.to_digit(10).unwrap())
    .collect();

  if digits.len() != 6 {
    return false;
  }

  let mut double_rule = false;
  let mut last_digit: Option<u32> = None;
  for digit in digits {
    if let Some(last_digit) = last_digit {
      if digit < last_digit {
        return false;
      }
      if digit == last_digit {
        double_rule = true;
      }
    }
    last_digit = Some(digit)
  }
  double_rule
}

pub fn count_valid_passwords(passwords: Vec<u32>) -> usize {
  passwords
    .into_par_iter()
    .filter(|x| is_valid_password(*x))
    .count()
}

pub fn is_valid_password_mk2(input: u32) -> bool {
  let input_as_str = format!("{}", input);
  let digits: Vec<u32> = input_as_str
    .chars()
    .map(|x| x.to_digit(10).unwrap())
    .collect();

  if digits.len() != 6 {
    return false;
  }

  let mut double_rule = false;
  let mut repeated = 1;
  let mut last_digit: Option<u32> = None;
  for digit in digits {
    if let Some(last_digit) = last_digit {
      if digit < last_digit {
        return false;
      }
      if digit == last_digit {
        repeated += 1
      } else {
        if repeated == 2 {
          double_rule = true
        }
        repeated = 1
      }
    }
    last_digit = Some(digit)
  }
  repeated == 2 || double_rule
}

pub fn count_valid_passwords_mk2(passwords: Vec<u32>) -> usize {
  passwords
    .into_par_iter()
    .filter(|x| is_valid_password_mk2(*x))
    .count()
}

pub const PUZZLE_INPUT: (u32, u32) = (367479, 893698);

#[cfg(test)]
mod part_one {
  use super::*;

  #[test]
  fn test_cases() {
    assert!(is_valid_password(122345));
    assert!(is_valid_password(111123));
    assert!(is_valid_password(111111));
    assert!(!is_valid_password(223450));
    assert!(!is_valid_password(123789));
  }

  #[test]
  fn answer() {
    let result = count_valid_passwords(
      (PUZZLE_INPUT.0..PUZZLE_INPUT.1)
        .into_iter()
        .collect::<Vec<u32>>(),
    );
    assert_eq!(result, 495);
  }
}

#[cfg(test)]
mod part_two {
  use super::*;
  #[test]
  fn test_cases() {
    assert!(is_valid_password_mk2(112233));
    assert!(!is_valid_password_mk2(123444));
    assert!(is_valid_password_mk2(111122));
  }
  #[test]
  fn answer() {
    let result = count_valid_passwords_mk2(
      (PUZZLE_INPUT.0..PUZZLE_INPUT.1)
        .into_iter()
        .collect::<Vec<u32>>(),
    );
    assert_eq!(result, 305);
  }
}
