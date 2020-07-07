// Day 5: Sunny with a Chance of Asteroids

use crate::intcode;
use crate::prelude::*;

lazy_static! {
  static ref PUZZLE_INPUT: String = puzzle_input::string_for_day("05");
}

#[cfg(test)]
mod part_one {
  use super::*;

  #[test]
  fn input_output() {
    let program = "3,0,4,0,99";
    let mut code = intcode::parse(program);
    let result = intcode::compute(&mut code, Some(42));
    assert_eq!(result, Some(42));
  }

  #[test]
  fn parameter_modes() {
    let mut code = intcode::parse("1002,4,3,4,33");
    intcode::compute(&mut code, None);
    assert_eq!(code, vec![1002, 4, 3, 4, 99]);
  }

  #[test]
  fn test_cases() {}
  // #[test]
  // fn answer() {}
}

// #[cfg(test)]
// mod part_two {
//   use super::*;
//   #[test]
//   fn test_cases() {}
//   #[test]
//   fn answer() {}
// }
