// Day 2: 1202 Program Alarm

use crate::intcode;
use crate::prelude::*;

pub fn brute_force_answer(
  sequence: &intcode::IntcodeSequence,
  noun_addr: usize,
  verb_addr: usize,
  desired_output: usize,
) -> (usize, usize) {
  let candidates: Vec<_> = (0..100)
    .flat_map(|noun_candidate| (0..100).map(move |verb_candidate| (noun_candidate, verb_candidate)))
    .collect();

  *candidates
    .iter()
    .find(move |&&(noun_candidate, verb_candidate)| {
      let mut candidate_sequence = sequence.clone();
      candidate_sequence[noun_addr] = noun_candidate;
      candidate_sequence[verb_addr] = verb_candidate;
      let result = intcode::compat::compute_v02(&mut candidate_sequence);
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
    let mut sequence = intcode::parse(program);
    let result = intcode::compat::compute_instruction_v02(&mut sequence, 0);
    assert_eq!(result, intcode::ProgramState::Continue(4));
    assert_eq!(sequence[3], 70);
  }

  #[test]
  fn detailed_test_case() {
    let program = "1,9,10,3,2,3,11,0,99,30,40,50";
    assert_eq!(intcode::compat::parse_and_compute_v02(program), 3500);
  }

  #[test]
  fn test_cases() {
    assert_eq!(intcode::compat::parse_and_compute_v02("1,0,0,0,99"), 2);
    assert_eq!(
      intcode::compat::parse_and_compute_v02("1,1,1,4,99,5,6,0,99"),
      30
    );
  }

  #[test]
  fn answer() {
    let mut sequence = intcode::parse(&PUZZLE_INPUT);
    sequence[1] = 12;
    sequence[2] = 2;
    assert_eq!(intcode::compat::compute_v02(&mut sequence), 5305097);
  }
}

#[cfg(test)]
mod part_two {
  use super::*;
  use std::time;
  #[test]
  fn part_two() {
    let start = time::Instant::now();
    let sequence = intcode::parse(&PUZZLE_INPUT);
    let (noun, verb) = brute_force_answer(&sequence, 1, 2, 19690720);
    let result = 100 * noun + verb;
    assert_eq!(result, 4925);
    println!("Time: {}", start.elapsed().as_micros());
  }
}
