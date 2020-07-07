pub mod compat;

pub type IntcodeSequence = Vec<usize>;

#[derive(Debug, PartialEq)]
pub enum ProgramState {
  Continue(usize),
  Halt,
}

pub fn parse(input: &str) -> IntcodeSequence {
  input.split(",").map(|num| num.parse().unwrap()).collect()
}

pub fn compute_instruction(
  sequence: &mut IntcodeSequence,
  instruction_pointer: usize,
) -> ProgramState {
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

pub fn compute(sequence: &mut IntcodeSequence, input: Option<usize>) -> Option<usize> {
  let mut instruction_pointer = 0;
  loop {
    let result = compute_instruction(sequence, instruction_pointer);
    match result {
      ProgramState::Continue(new_position) => instruction_pointer = new_position,
      ProgramState::Halt => return None,
    }
  }
}

// pub fn parse_and_compute(input: &str) -> usize {
//   let mut sequence = parse(input);
//   compute(&mut sequence)
// }
