// Day 7: Amplification Circuit

use crate::logic::intcode;
use crate::logic::intcode::IntcodeSequenceUtils;
use crate::prelude::*;
use std::cell::Cell;

pub type PhaseSettingSequence = [u8; 5];

pub fn compute_thruster_signal(
  sequence: &intcode::IntcodeSequence,
  phase_settings: &PhaseSettingSequence,
) -> isize {
  let mut signal = 0;
  for phase_setting in phase_settings.iter().cloned() {
    let mut copy_memory = sequence.clone();
    let computer = intcode::IntcodeComputer::new(&mut copy_memory).start();
    let computer = computer
      .as_input()
      .expect("Expected computer to take phase setting input")
      .execute(phase_setting.into());
    let computer = computer
      .as_input()
      .expect("Expected computer to take signal input")
      .execute(signal);

    let computer = computer
      .as_output()
      .expect("Expected computer to give output");

    signal = computer.output;

    computer
      .execute()
      .as_halt()
      .expect("Expected computer to halt");
  }

  signal
}

fn get_all_phase_setting_combinations(
  options: &[u8],
) -> impl std::iter::Iterator<Item = PhaseSettingSequence> + '_ {
  options.iter().flat_map(move |i1| {
    let remaining = options.iter().filter(move |x| *x != i1);
    remaining.clone().into_iter().flat_map(move |i2| {
      let remaining = remaining.clone().into_iter().filter(move |x| *x != i2);
      remaining.clone().into_iter().flat_map(move |i3| {
        let remaining = remaining.clone().into_iter().filter(move |x| *x != i3);
        remaining.clone().into_iter().flat_map(move |i4| {
          let remaining = remaining.clone().into_iter().filter(move |x| *x != i4);
          remaining.map(move |i5| [*i1, *i2, *i3, *i4, *i5])
        })
      })
    })
  })
}

pub fn get_highest_phase_settings(
  sequence: &intcode::IntcodeSequence,
  phase_settings_options: &[u8],
) -> isize {
  get_all_phase_setting_combinations(phase_settings_options)
    .par_bridge()
    .map(|phase_settings| compute_thruster_signal(&sequence, &phase_settings))
    .max()
    .unwrap()
}

pub fn compute_thruster_signal_feedback(
  sequence: &intcode::IntcodeSequence,
  phase_settings: &PhaseSettingSequence,
) -> isize {
  let mut computer_memory: Vec<_> = phase_settings
    .iter()
    .map(|x| (sequence.clone(), x))
    .collect();
  let computers: Vec<_> = computer_memory
    .iter_mut()
    .map(|(memory, phase_setting)| {
      let computer = memory
        .start()
        .as_input()
        .expect("Expected computer to take phase setting input")
        .execute(isize::from(**phase_setting));
      Cell::new(Some(computer))
    })
    .collect();

  let mut signal = 0;
  loop {
    for computer_cell in computers.iter() {
      let computer = computer_cell.take().expect("Expected computer to exist!");
      match computer {
        intcode::IntcodeComputer::Input(state) => {
          let new_computer = state.execute(signal);
          let new_computer = new_computer
            .as_output()
            .expect("Expected computer to give output");
          signal = new_computer.output;
          computer_cell.set(Some(new_computer.execute()));
        }
        intcode::IntcodeComputer::Halt(_) => {
          return signal;
        }
        state => panic!(
          "Unexpected computer state (expected to take input or halt): {:?}",
          state
        ),
      }
    }
  }
}

pub fn get_highest_feedback_phase_settings(
  sequence: &intcode::IntcodeSequence,
  phase_settings_options: &[u8],
) -> isize {
  get_all_phase_setting_combinations(phase_settings_options)
    .par_bridge()
    .map(|phase_settings| compute_thruster_signal_feedback(&sequence, &phase_settings))
    .max()
    .unwrap()
}

lazy_static! {
  static ref PUZZLE_INPUT: String = puzzle_input::string_for_day("07");
}

#[cfg(test)]
mod part_one {
  use super::*;
  use crate::logic::intcode::IntcodeSequenceUtils;

  #[test]
  fn test_cases() {
    assert_eq!(
      compute_thruster_signal(
        &intcode::IntcodeSequence::parse("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"),
        &[4, 3, 2, 1, 0],
      ),
      43210
    );
    assert_eq!(
      compute_thruster_signal(
        &intcode::IntcodeSequence::parse(
          "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"
        ),
        &[0, 1, 2, 3, 4],
      ),
      54321
    );
    assert_eq!(
      compute_thruster_signal(
        &intcode::IntcodeSequence::parse(
          "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"
        ),
        &[1, 0, 4, 3, 2],
      ),
      65210
    );
  }

  #[test]
  fn combinations() {
    let combinations = get_all_phase_setting_combinations(&(0..5).collect::<Vec<_>>()).count();
    assert_eq!(combinations, 120);
  }

  #[test]
  fn answer() {
    let sequence = intcode::IntcodeSequence::parse(&PUZZLE_INPUT);
    let result = get_highest_phase_settings(&sequence, &(0..5).collect::<Vec<_>>());
    assert_eq!(result, 77500);
  }
}

#[cfg(test)]
mod part_two {
  use super::*;
  use crate::logic::intcode::IntcodeSequenceUtils;

  #[test]
  fn test_cases() {
    assert_eq!(
      compute_thruster_signal_feedback(
        &intcode::IntcodeSequence::parse(
          "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
        ),
        &[9, 8, 7, 6, 5],
      ),
      139629729
    );
    assert_eq!(
      compute_thruster_signal_feedback(
        &intcode::IntcodeSequence::parse(
          "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"
        ),
        &[9,7,8,5,6],
      ),
      18216
    );
  }
  #[test]
  fn answer() {
    let sequence = intcode::IntcodeSequence::parse(&PUZZLE_INPUT);
    let result = get_highest_feedback_phase_settings(&sequence, &(5..10).collect::<Vec<_>>());
    assert_eq!(result, 22476942);
  }
}
