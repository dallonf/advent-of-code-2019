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

pub struct IntcodeComputerState<'a> {
  pub sequence: &'a mut IntcodeSequence,
  pointer: usize,
}

pub struct IntcodeComputerStart<'a> {
  pub state: IntcodeComputerState<'a>,
}
pub struct IntcodeComputerInputState<'a> {
  pub state: IntcodeComputerState<'a>,
}
pub struct IntcodeComputerOutputState<'a> {
  pub state: IntcodeComputerState<'a>,
  pub output: isize,
}
pub struct IntcodeComputerHaltState<'a> {
  pub state: IntcodeComputerState<'a>,
}

pub enum IntcodeComputer<'a> {
  Input(IntcodeComputerInputState<'a>),
  Output(IntcodeComputerOutputState<'a>),
  Halt(IntcodeComputerHaltState<'a>),
}
impl IntcodeComputer<'_> {
  pub fn new(sequence: &mut IntcodeSequence) -> IntcodeComputerStart {
    IntcodeComputerStart {
      state: IntcodeComputerState {
        sequence,
        pointer: 0,
      },
    }
  }
}
impl<'a> IntcodeComputerState<'a> {
  fn compute(mut self) -> IntcodeComputer<'a> {
    loop {
      let result = compute_instruction(self.sequence, self.pointer);
      match result {
        ProgramState::Continue(new_position) => {
          self.pointer = new_position;
        }
        ProgramState::OutputAndContinue {
          pointer: new_position,
          output,
        } => {
          self.pointer = new_position;
          return IntcodeComputer::Output(IntcodeComputerOutputState {
            state: self,
            output,
          });
        }
        ProgramState::WaitForInput => {
          return IntcodeComputer::Input(IntcodeComputerInputState { state: self });
        }
        ProgramState::Halt => {
          return IntcodeComputer::Halt(IntcodeComputerHaltState { state: self })
        }
      }
    }
  }
}
impl<'a> IntcodeComputerStart<'a> {
  pub fn start(self) -> IntcodeComputer<'a> {
    self.state.compute()
  }
}
impl<'a> IntcodeComputerInputState<'a> {
  pub fn execute(mut self, input: isize) -> IntcodeComputer<'a> {
    let instruction = parse_instruction(&self.state.sequence, self.state.pointer, 1);
    let destination_addr = instruction.raw_parameters[0];
    self.state.sequence.set(destination_addr, input);
    self.state.pointer = instruction.next_pointer;
    self.state.compute()
  }
}
impl<'a> IntcodeComputerOutputState<'a> {
  pub fn execute(self) -> IntcodeComputer<'a> {
    self.state.compute()
  }
}

#[derive(Debug, PartialEq)]
pub enum ProgramState {
  Continue(usize),
  WaitForInput,
  OutputAndContinue { pointer: usize, output: isize },
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
      ProgramState::WaitForInput
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

#[derive(Debug, PartialEq, Eq)]
struct InstructionParameters {
  raw_parameters: Vec<isize>,
  parameter_modes: Vec<u8>,
  parameters: Vec<isize>,
  next_pointer: usize,
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
