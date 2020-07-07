/// Computes as defined in Day 02. No support for input/output;
/// instead returns the value at position 0.
pub fn compute_v02(sequence: &mut super::IntcodeSequence) -> usize {
  super::compute(sequence, None);
  sequence[0]
}

pub fn parse_and_compute_v02(input: &str) -> usize {
  let mut sequence = super::parse(input);
  compute_v02(&mut sequence)
}
