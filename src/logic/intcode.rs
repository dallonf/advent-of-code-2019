use std::convert::{TryFrom, TryInto};

pub mod compat;

pub type IntcodeSequence = Vec<isize>;
trait IntcodeIndexable {
  fn index(&self, i: isize) -> isize;
  fn index_mut(&mut self, i: isize) -> &mut isize;
  fn set(&mut self, i: isize, val: isize);
}
impl IntcodeIndexable for IntcodeSequence {
  fn index(&self, i: isize) -> isize {
    self[usize::try_from(i).unwrap()]
  }
  fn index_mut(&mut self, i: isize) -> &mut isize {
    &mut self[usize::try_from(i).unwrap()]
  }
  fn set(&mut self, i: isize, val: isize) {
    self[usize::try_from(i).unwrap()] = val;
  }
}
trait Unsign {
  fn expect_unsigned(self) -> usize;
}
impl Unsign for isize {
  fn expect_unsigned(self) -> usize {
    self.try_into().unwrap()
  }
}

#[derive(Debug, PartialEq)]
pub enum ProgramState {
  Continue(usize),
  OutputAndContinue { pointer: usize, output: isize },
  Halt,
}

pub fn parse(input: &str) -> IntcodeSequence {
  input.split(",").map(|num| num.parse().unwrap()).collect()
}

pub fn compute_instruction(
  sequence: &mut IntcodeSequence,
  instruction_pointer: usize,
  input: Option<isize>,
) -> ProgramState {
  let instruction = sequence[instruction_pointer];
  let opcode = instruction % 100;
  match opcode {
    1 => {
      // Add
      let instruction = parse_instruction(&sequence, instruction_pointer, 3);
      let a = instruction.parameters[0];
      let b = instruction.parameters[1];
      sequence.set(instruction.raw_parameters[2], a + b);
      ProgramState::Continue(instruction.next_pointer)
    }
    2 => {
      // Multiply
      let instruction = parse_instruction(&sequence, instruction_pointer, 3);
      let a = instruction.parameters[0];
      let b = instruction.parameters[1];
      sequence.set(instruction.raw_parameters[2], a * b);
      ProgramState::Continue(instruction.next_pointer)
    }
    3 => {
      // Input
      let instruction = parse_instruction(&sequence, instruction_pointer, 1);
      let destination_addr = instruction.raw_parameters[0];
      sequence.set(destination_addr, input.expect("Program expected input!"));
      ProgramState::Continue(instruction.next_pointer)
    }
    4 => {
      // Output
      let instruction = parse_instruction(&sequence, instruction_pointer, 1);
      ProgramState::OutputAndContinue {
        pointer: instruction.next_pointer,
        output: instruction.parameters[0],
      }
    }
    5 => {
      // Jump If True
      let instruction = parse_instruction(&sequence, instruction_pointer, 2);
      if instruction.parameters[0] != 0 {
        ProgramState::Continue(instruction.parameters[1].expect_unsigned())
      } else {
        ProgramState::Continue(instruction.next_pointer)
      }
    }
    6 => {
      // Jump If False
      let instruction = parse_instruction(&sequence, instruction_pointer, 2);
      if instruction.parameters[0] == 0 {
        ProgramState::Continue(instruction.parameters[1].expect_unsigned())
      } else {
        ProgramState::Continue(instruction.next_pointer)
      }
    }
    7 => {
      // Less Than
      let instruction = parse_instruction(&sequence, instruction_pointer, 3);
      sequence.set(
        instruction.raw_parameters[2],
        if instruction.parameters[0] < instruction.parameters[1] {
          1
        } else {
          0
        },
      );
      ProgramState::Continue(instruction.next_pointer)
    }
    8 => {
      // Equals
      let instruction = parse_instruction(&sequence, instruction_pointer, 3);
      sequence.set(
        instruction.raw_parameters[2],
        if instruction.parameters[0] == instruction.parameters[1] {
          1
        } else {
          0
        },
      );
      ProgramState::Continue(instruction.next_pointer)
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

pub fn compute(sequence: &mut IntcodeSequence, input: Option<isize>) -> Option<isize> {
  let mut instruction_pointer = 0;
  let mut output: Option<isize> = None;
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
  raw_parameters: Vec<isize>,
  parameter_modes: Vec<u8>,
  parameters: Vec<isize>,
  next_pointer: usize,
}

pub fn parse_and_compute(code: &str, input: Option<isize>) -> Option<isize> {
  let mut sequence = parse(code);
  compute(&mut sequence, input)
}

fn parse_instruction(
  sequence: &IntcodeSequence,
  pointer: usize,
  num_params: u8,
) -> InstructionParameters {
  let raw_parameters: Vec<isize> = (1..num_params + 1)
    .map(|i| sequence[pointer + usize::from(i)])
    .collect();

  let instruction: usize = sequence[pointer].try_into().unwrap();
  let parameter_modes: Vec<u8> = (0..num_params)
    .map(|i| {
      let place = 10usize.pow((i + 2).into());
      ((instruction / place) % 10).try_into().unwrap()
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
          sequence.index(raw_param)
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
    next_pointer: pointer + 1 + usize::from(num_params),
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
