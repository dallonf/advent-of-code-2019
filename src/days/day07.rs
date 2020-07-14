// Day 7: Amplification Circuit

use crate::logic::intcode;

use crate::prelude::*;

pub type PhaseSettingSequence = [u8; 5];

pub fn compute_thruster_signal(
  sequence: intcode::IntcodeSequence,
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

lazy_static! {
  static ref PUZZLE_INPUT: Vec<String> = puzzle_input::lines_for_day("07");
}

#[cfg(test)]
mod part_one {
  use super::*;
  use crate::logic::intcode::IntcodeSequenceUtils;

  #[test]
  fn test_cases() {
    assert_eq!(
      compute_thruster_signal(
        intcode::IntcodeSequence::parse("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"),
        &[4, 3, 2, 1, 0],
      ),
      43210
    );
  }
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
