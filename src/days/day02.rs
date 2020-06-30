// Day NN: Description

use crate::prelude::*;

pub type IntcodeSequence = Vec<usize>;

#[derive(Debug, PartialEq)]
enum ProgramState {
  Continue(usize),
  Halt,
}

pub fn parse(input: &str) -> IntcodeSequence {
  input.split(",").map(|num| num.parse().unwrap()).collect()
}

fn compute_instruction(sequence: &mut IntcodeSequence, position: usize) -> ProgramState {
  let instruction = sequence[position];
  match instruction {
    1 => {
      // Add
      let a_addr = sequence[position + 1];
      let b_addr = sequence[position + 2];
      let result_addr = sequence[position + 3];

      let a = sequence[a_addr];
      let b = sequence[b_addr];
      let result = &mut sequence[result_addr];
      *result = a + b;

      ProgramState::Continue(position + 4)
    }
    2 => {
      // Multiply
      let a_addr = sequence[position + 1];
      let b_addr = sequence[position + 2];
      let result_addr = sequence[position + 3];

      let a = sequence[a_addr];
      let b = sequence[b_addr];
      let result = &mut sequence[result_addr];
      *result = a * b;

      ProgramState::Continue(position + 4)
    }
    99 => ProgramState::Halt,
    _ => {
      panic!(format!(
        "Unrecognized instruction {} at position {}",
        instruction, position
      ));
    }
  }
}

pub fn compute(sequence: &mut IntcodeSequence) -> usize {
  let mut position = 0;
  loop {
    let result = compute_instruction(sequence, position);
    match result {
      ProgramState::Continue(new_position) => position = new_position,
      ProgramState::Halt => return sequence[0],
    }
  }
}

pub fn parse_and_compute(input: &str) -> usize {
  let mut sequence = parse(input);
  compute(&mut sequence)
}

lazy_static! {
  static ref PUZZLE_INPUT: String = puzzle_input::string_for_day("02");
}

#[cfg(test)]
mod part_one {
  use super::*;

  #[test]
  fn run_single_instruction() {
    let program = "1,9,10,3,2,3,11,0,99,30,40,50";
    let mut sequence = parse(program);
    let result = compute_instruction(&mut sequence, 0);
    assert_eq!(result, ProgramState::Continue(4));
    assert_eq!(sequence[3], 70);
  }

  #[test]
  fn detailed_test_case() {
    let program = "1,9,10,3,2,3,11,0,99,30,40,50";
    assert_eq!(parse_and_compute(program), 3500);
  }

  #[test]
  fn test_cases() {
    assert_eq!(parse_and_compute("1,0,0,0,99"), 2);
    assert_eq!(parse_and_compute("1,1,1,4,99,5,6,0,99"), 30);
  }

  #[test]
  fn answer() {
    let mut sequence = parse(&PUZZLE_INPUT);
    sequence[1] = 12;
    sequence[2] = 2;
    assert_eq!(compute(&mut sequence), 5305097);
  }
}

// #[cfg(test)]
// mod part_two {
//   use super::*;
//   #[test]
//   fn test_cases() {}
//   #[test]
//   fn part_two() {}
// }
