use brainease_lexer::syntax::{GotoBy, GotoDirection, Instruction};

use crate::{io_handler::IoHandler, runtime::Runtime};

pub fn execute<I: Clone + IoHandler>(
  instruction: &Instruction,
  runtime: &mut Runtime<I>,
) {
  match instruction {
    Instruction::Increment { cell, value } => {
      runtime.memory[*cell] = runtime.memory[*cell].wrapping_add(*value);
    }

    Instruction::Decrement { cell, value } => {
      runtime.memory[*cell] = runtime.memory[*cell].wrapping_sub(*value);
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
      runtime.memory[*cell] = runtime.io_handler.read_input();
    }

    Instruction::Write(cell) => {
      runtime
        .io_handler
        .write_output(runtime.memory[*cell].to_ne_bytes()[0]);
    }

    Instruction::Print(cell) => {
      runtime.io_handler.write_output(runtime.memory[*cell]);
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
      cell_or_value: value,
      logic,
      inner,
      is_cell,
    } => {
      let other = if *is_cell {
        runtime.memory[*value]
      } else {
        *value as u8
      };

      if logic.matches(runtime.memory[*cell], other) {
        for instruction in inner {
          execute(instruction, runtime);
        }
      }
    }

    Instruction::Goto { dir, by } => {
      let amount = if let Some(goto_by) = by {
        match goto_by {
          GotoBy::ByCell(cell) => runtime.memory[*cell] as usize,
          GotoBy::ByValue(value) => *value,
        }
      } else {
        1
      };

      match dir {
        GotoDirection::Right => runtime.pointer += amount,
        GotoDirection::Left => runtime.pointer -= amount,
      };
    }
  }
}
