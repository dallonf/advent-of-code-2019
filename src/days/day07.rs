// Day 7: Amplification Circuit

use crate::logic::intcode;

use crate::prelude::*;

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

pub fn get_highest_phase_settings(
  sequence: &intcode::IntcodeSequence,
  phase_settings_options: &[u8],
) -> isize {
  phase_settings_options
    .par_iter()
    .cloned()
    .flat_map(move |i1| {
      phase_settings_options
        .par_iter()
        .cloned()
        .flat_map(move |i2| {
          phase_settings_options
            .par_iter()
            .cloned()
            .flat_map(move |i3| {
              phase_settings_options
                .par_iter()
                .cloned()
                .flat_map(move |i4| {
                  phase_settings_options
                    .par_iter()
                    .cloned()
                    .map(move |i5| [i1, i2, i3, i4, i5])
                })
            })
        })
    })
    .map(|phase_settings| compute_thruster_signal(&sequence, &phase_settings))
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
  fn answer() {
    let sequence = intcode::IntcodeSequence::parse(&PUZZLE_INPUT);
    let result = get_highest_phase_settings(&sequence, &(0..5).collect::<Vec<_>>());
    assert!(result < 4968420);
    // assert_eq!(result, 0);
  }
}

// #[cfg(test)]
// mod part_two {
//   use super::*;
//   #[test]
//   fn test_cases() {}
//   #[test]
//   fn answer() {}
// }
