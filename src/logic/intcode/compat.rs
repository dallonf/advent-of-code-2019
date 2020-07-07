use std::convert::TryInto;

/// Computes as defined in Day 02. No support for input/output;
/// instead returns the value at position 0.
pub fn compute_v02(sequence: &mut super::IntcodeSequence) -> usize {
  super::compute(sequence, None);
  sequence[0].try_into().unwrap()
}

pub fn compute_instruction_v02(
  sequence: &mut super::IntcodeSequence,
  instruction_pointer: usize,
) -> super::ProgramState {
  super::compute_instruction(sequence, instruction_pointer, None)
}

pub fn parse_and_compute_v02(input: &str) -> usize {
  let mut sequence = super::parse(input);
  compute_v02(&mut sequence)
}
