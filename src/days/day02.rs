// Day 2: 1202 Program Alarm

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

fn compute_instruction(sequence: &mut IntcodeSequence, instruction_pointer: usize) -> ProgramState {
  let instruction = sequence[instruction_pointer];
  match instruction {
    1 => {
      // Add
      let a_addr = sequence[instruction_pointer + 1];
      let b_addr = sequence[instruction_pointer + 2];
      let result_addr = sequence[instruction_pointer + 3];

      let a = sequence[a_addr];
      let b = sequence[b_addr];
      let result = &mut sequence[result_addr];
      *result = a + b;

      ProgramState::Continue(instruction_pointer + 4)
    }
    2 => {
      // Multiply
      let a_addr = sequence[instruction_pointer + 1];
      let b_addr = sequence[instruction_pointer + 2];
      let result_addr = sequence[instruction_pointer + 3];

      let a = sequence[a_addr];
      let b = sequence[b_addr];
      let result = &mut sequence[result_addr];
      *result = a * b;

      ProgramState::Continue(instruction_pointer + 4)
    }
    99 => ProgramState::Halt,
    _ => {
      panic!(format!(
        "Unrecognized instruction {} at instruction pointer {}",
        instruction, instruction_pointer
      ));
    }
  }
}

pub fn compute(sequence: &mut IntcodeSequence) -> usize {
  let mut instruction_pointer = 0;
  loop {
    let result = compute_instruction(sequence, instruction_pointer);
    match result {
      ProgramState::Continue(new_position) => instruction_pointer = new_position,
      ProgramState::Halt => return sequence[0],
    }
  }
}

pub fn parse_and_compute(input: &str) -> usize {
  let mut sequence = parse(input);
  compute(&mut sequence)
}

pub fn brute_force_answer(
  sequence: &IntcodeSequence,
  noun_addr: usize,
  verb_addr: usize,
  desired_output: usize,
) -> (usize, usize) {
  let candidates: Vec<_> = (0..100)
    .flat_map(|noun_candidate| (0..100).map(move |verb_candidate| (noun_candidate, verb_candidate)))
    .collect();

  *candidates
    .par_iter()
    .find_any(move |&&(noun_candidate, verb_candidate)| {
      let mut candidate_sequence = sequence.clone();
      candidate_sequence[noun_addr] = noun_candidate;
      candidate_sequence[verb_addr] = verb_candidate;
      let result = compute(&mut candidate_sequence);
      result == desired_output
    })
    .expect("No answer found")
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

#[cfg(test)]
mod part_two {
  use super::*;
  #[test]
  fn part_two() {
    let sequence = parse(&PUZZLE_INPUT);
    let (noun, verb) = brute_force_answer(&sequence, 1, 2, 19690720);
    let result = 100 * noun + verb;
    assert_eq!(result, 4925);
  }
}
