use std::convert::TryInto;

/// Computes as defined in Day 02. No support for input/output;
/// instead returns the value at position 0.
pub fn compute_v02(sequence: &mut super::IntcodeSequence) -> usize {
  compute_v05(sequence, None);
  sequence[0].try_into().unwrap()
}

pub fn compute_instruction_v02(
  sequence: &mut super::IntcodeSequence,
  instruction_pointer: usize,
) -> super::ProgramState {
  super::compute_instruction(sequence, instruction_pointer)
}

pub fn parse_and_compute_v02(input: &str) -> usize {
  let mut sequence = super::parse(input);
  compute_v02(&mut sequence)
}

pub fn compute_v05(sequence: &mut super::IntcodeSequence, input: Option<isize>) -> Option<isize> {
  let mut input = input;
  let mut output: Option<isize> = None;
  let computer = super::IntcodeComputer::new(sequence.clone());
  let mut computer = computer.start();
  loop {
    match computer {
      super::IntcodeComputer::Input(state) => match input {
        Some(input_value) => {
          computer = state.execute(input_value);
          input = None;
        }
        None => panic!("Program expected input but there was one"),
      },
      super::IntcodeComputer::Output(state) => {
        output = Some(state.output);
        computer = state.execute();
      }
      super::IntcodeComputer::Halt(state) => {
        *sequence = state.state.sequence;
        return output;
      }
    }
  }
}

pub fn parse_and_compute_v05(code: &str, input: Option<isize>) -> Option<isize> {
  let mut sequence = super::parse(code);
  compute_v05(&mut sequence, input)
}
