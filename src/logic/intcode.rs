pub mod compat;

pub type IntcodeSequence = Vec<usize>;

#[derive(Debug, PartialEq)]
pub enum ProgramState {
  Continue(usize),
  OutputAndContinue { pointer: usize, output: usize },
  Halt,
}

pub fn parse(input: &str) -> IntcodeSequence {
  input.split(",").map(|num| num.parse().unwrap()).collect()
}

pub fn compute_instruction(
  sequence: &mut IntcodeSequence,
  instruction_pointer: usize,
  input: Option<usize>,
) -> ProgramState {
  let instruction = sequence[instruction_pointer];
  let opcode = instruction % 100;
  match opcode {
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
      let instruction = parse_instruction(&sequence, instruction_pointer, 3);
      let a = instruction.parameters[0];
      let b = instruction.parameters[1];
      let result = &mut sequence[instruction.raw_parameters[2]];
      *result = a * b;
      ProgramState::Continue(instruction.next_pointer)
    }
    3 => {
      // Input
      let destination_addr = sequence[instruction_pointer + 1];
      sequence[destination_addr] = input.expect("Program expected input!");

      ProgramState::Continue(instruction_pointer + 2)
    }
    4 => {
      // Output
      let source_addr = sequence[instruction_pointer + 1];
      let value = sequence[source_addr];

      ProgramState::OutputAndContinue {
        pointer: instruction_pointer + 2,
        output: value,
      }
    }
    99 => ProgramState::Halt,
    _ => {
      panic!(format!(
        "Unrecognized opcode {} at instruction pointer {}",
        opcode, instruction_pointer
      ));
    }
  }
}

pub fn compute(sequence: &mut IntcodeSequence, input: Option<usize>) -> Option<usize> {
  let mut instruction_pointer = 0;
  let mut output: Option<usize> = None;
  loop {
    let result = compute_instruction(sequence, instruction_pointer, input);
    match result {
      ProgramState::Continue(new_position) => instruction_pointer = new_position,
      ProgramState::OutputAndContinue {
        pointer: new_position,
        output: new_output,
      } => {
        output = Some(new_output);
        instruction_pointer = new_position;
      }
      ProgramState::Halt => return output,
    }
  }
}

#[derive(Debug, PartialEq, Eq)]
struct InstructionParameters {
  raw_parameters: Vec<usize>,
  parameter_modes: Vec<u8>,
  parameters: Vec<usize>,
  next_pointer: usize,
}

pub fn parse_and_compute(code: &str, input: Option<usize>) -> Option<usize> {
  let mut sequence = parse(code);
  compute(&mut sequence, input)
}

fn parse_instruction(
  sequence: &IntcodeSequence,
  pointer: usize,
  num_params: u8,
) -> InstructionParameters {
  let raw_parameters: Vec<usize> = (1..num_params + 1)
    .map(|i| sequence[pointer + i as usize])
    .collect();

  let instruction = sequence[pointer];
  let parameter_modes: Vec<u8> = (0..num_params)
    .map(|i| {
      let place = 10usize.pow((i + 2).into());
      ((instruction / place) % 10) as u8
    })
    .collect();

  let parameters = (0..num_params)
    .map(|i| {
      let i: usize = i.into();
      let raw_param = raw_parameters[i];
      let param_mode = parameter_modes[i];
      match param_mode {
        0 => {
          // Position Mode
          sequence[raw_param]
        }
        1 => {
          // Immediate Mode
          raw_param
        }
        _ => panic!(
          "Unrecognized parameter mode {} at instruction pointer {}",
          param_mode, pointer
        ),
      }
    })
    .collect();

  InstructionParameters {
    raw_parameters,
    parameter_modes,
    parameters,
    next_pointer: pointer + 1 + num_params as usize,
  }
}

#[cfg(test)]
mod part_one {
  use super::*;
  #[test]
  fn test_parse_instruction() {
    let sequence = parse("1002,4,3,4,33");
    assert_eq!(
      parse_instruction(&sequence, 0, 3),
      InstructionParameters {
        raw_parameters: vec![4, 3, 4],
        parameter_modes: vec![0, 1, 0],
        parameters: vec![33, 3, 33],
        next_pointer: 4
      }
    );
  }
}
