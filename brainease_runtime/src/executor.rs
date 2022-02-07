use std::io::{stdin, stdout, Read, Write};

use crate::runtime::Runtime;
use brainease_lexer::syntax::Instruction;

pub fn execute(instruction: &Instruction, runtime: &mut Runtime) {
  match instruction {
    Instruction::Increment { cell, value } => {
      let cell_value = runtime.memory[*cell];
      runtime.memory[*cell] = cell_value.wrapping_add(*value);
    }

    Instruction::Decrement { cell, value } => {
      let cell_value = runtime.memory[*cell];
      runtime.memory[*cell] = cell_value.wrapping_sub(*value);
    }

    Instruction::Move { current, next } => {
      runtime.memory[*next] = runtime.memory[*current];
      runtime.memory[*current] = 0;
    }

    Instruction::Swap { from, with } => {
      runtime.memory.swap(*from, *with);
    }

    Instruction::Save { cell, value } => {
      runtime.memory[*cell] = *value;
    }

    Instruction::Read(cell) => {
      let mut input = [0u8];

      stdin()
        .read_exact(&mut input)
        .expect("failed to read stdin");

      runtime.memory[*cell] = input[0];
    }

    Instruction::Write(cell) => {
      print!("{}", runtime.memory[*cell]);
    }

    Instruction::Print(cell) => {
      stdout()
        .write_all(&[runtime.memory[*cell]])
        .expect("failed to write to stdout");
    }

    Instruction::Loop { cell, inner } => {
      while runtime.memory[*cell] != 0 {
        for instruction in inner {
          execute(instruction, runtime);
        }
      }
    }

    Instruction::If {
      cell,
      value,
      logic,
      inner,
    } => {
      if logic.matches(runtime.memory[*cell], *value) {
        for instruction in inner {
          execute(instruction, runtime);
        }
      }
    }

    Instruction::IfCell { a, b, logic, inner } => {
      if logic.matches(runtime.memory[*a], runtime.memory[*b]) {
        for instruction in inner {
          execute(instruction, runtime);
        }
      }
    }
  }
}
