// Day 5: Sunny with a Chance of Asteroids

use crate::intcode;
use crate::prelude::*;

lazy_static! {
  static ref PUZZLE_INPUT: String = puzzle_input::string_for_day("05");
}

pub fn get_diagnostic_code(input: isize) -> isize {
  intcode::parse_and_compute(&PUZZLE_INPUT, Some(input)).unwrap()
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
  fn negative_integers() {
    let mut code = intcode::parse("1101,100,-1,4,0");
    intcode::compute(&mut code, None);
    assert_eq!(code, vec![1101, 100, -1, 4, 99]);
  }

  #[test]
  fn answer() {
    assert_eq!(get_diagnostic_code(1), 13787043);
  }
}

#[cfg(test)]
mod part_two {
  use super::*;
  #[test]
  fn test_comparison() {
    // Position mode, output = input == 8
    assert_eq!(
      intcode::parse_and_compute("3,9,8,9,10,9,4,9,99,-1,8", Some(8)),
      Some(1)
    );
    assert_eq!(
      intcode::parse_and_compute("3,9,8,9,10,9,4,9,99,-1,8", Some(5)),
      Some(0)
    );

    // Position mode, output = input < 8
    assert_eq!(
      intcode::parse_and_compute("3,9,7,9,10,9,4,9,99,-1,8", Some(2)),
      Some(1)
    );
    assert_eq!(
      intcode::parse_and_compute("3,9,7,9,10,9,4,9,99,-1,8", Some(10)),
      Some(0)
    );

    // Immediate mode, output = input == 8
    assert_eq!(
      intcode::parse_and_compute("3,3,1108,-1,8,3,4,3,99", Some(8)),
      Some(1)
    );
    assert_eq!(
      intcode::parse_and_compute("3,3,1108,-1,8,3,4,3,99", Some(5)),
      Some(0)
    );

    // Immediate mode, output = input < 8
    assert_eq!(
      intcode::parse_and_compute("3,3,1107,-1,8,3,4,3,99", Some(2)),
      Some(1)
    );
    assert_eq!(
      intcode::parse_and_compute("3,3,1107,-1,8,3,4,3,99", Some(10)),
      Some(0)
    );
  }

  // #[test]
  // fn answer() {}
}
