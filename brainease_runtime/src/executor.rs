use brainease_lexer::syntax::{GotoBy, GotoDirection, Instruction};

use crate::{io_handler::IoHandler, runtime::Runtime};

pub fn execute<I>(
  instruction: &Instruction,
  runtime: &mut Runtime<I>,
) -> Result<(), I::Err>
where
  I: IoHandler,
{
  match instruction {
    Instruction::Increment { cell, value } => {
      let pointer = cell.or(runtime.pointer);

      runtime.memory[pointer] = runtime.memory[pointer].wrapping_add(*value);
    }

    Instruction::Decrement { cell, value } => {
      let pointer = cell.or(runtime.pointer);

      runtime.memory[pointer] = runtime.memory[pointer].wrapping_sub(*value);
    }

    Instruction::Move { current, next } => {
      let current_pointer = current.or(runtime.pointer);
      let next_pointer = next.or(runtime.pointer);

      runtime.memory[next_pointer] = runtime.memory[current_pointer];
      runtime.memory[current_pointer] = 0;
    }

    Instruction::Swap { from, with } => {
      let from_pointer = from.or(runtime.pointer);
      let with_pointer = with.or(runtime.pointer);

      runtime.memory.swap(from_pointer, with_pointer);
    }

    Instruction::Save { cell, value } => {
      let pointer = cell.or(runtime.pointer);

      runtime.memory[pointer] = *value;
    }

    Instruction::Read(cell) => {
      let pointer = cell.or(runtime.pointer);

      runtime.memory[pointer] = runtime.io_handler.read_input()?;
    }

    Instruction::Write(cell) => {
      let pointer = cell.or(runtime.pointer);
      let val = runtime.memory[pointer];

      runtime
        .io_handler
        .write_output(val.to_string().as_bytes())?;
    }

    Instruction::Print(cell) => {
      let pointer = cell.or(runtime.pointer);

      runtime
        .io_handler
        .write_output(&[runtime.memory[pointer]])?;
    }

    Instruction::Loop { cell, inner } => {
      let pointer = cell.or(runtime.pointer);

      while runtime.memory[pointer] != 0 {
        for instruction in inner {
          execute(instruction, runtime)?;
        }
      }
    }

    Instruction::If {
      cell,
      cell_or_value,
      logic,
      inner,
      is_cell,
    } => {
      let cell_pointer = cell.or(runtime.pointer);

      let other = if *is_cell {
        let cell_or_value_pointer = cell_or_value.or(runtime.pointer);
        runtime.memory[cell_or_value_pointer]
      } else {
        cell_or_value.unwrap() as u8
      };

      if logic.matches(runtime.memory[cell_pointer], other) {
        for instruction in inner {
          execute(instruction, runtime)?;
        }
      }
    }

    Instruction::Goto { dir, by } => {
      let mut amount = if let Some(goto_by) = by {
        match *goto_by {
          GotoBy::Number(value) => value,
          GotoBy::Cell(cell) => runtime.memory[cell] as usize,
          GotoBy::Pointer => runtime.pointer,
        }
      } else {
        1
      };

      match dir {
        GotoDirection::Right => {
          // How much cells do we have until an memory overflow. (1, 2, 3, PANIC(Array out of bounds))
          let space_left = runtime.memory.len() - runtime.pointer;

          if amount >= space_left {
            amount -= space_left;

            // End of the memory, so go to the start.
            runtime.pointer = 0;
          }

          runtime.pointer += amount;
        }

        GotoDirection::Left => {
          // How much cells do we have until an negative overflow. (3, 2, 1, 0, PANIC(Subtract with overflow))
          let space_left = runtime.pointer;

          if amount > space_left {
            amount -= space_left;

            // Start of the memory, so go to the end.
            runtime.pointer = runtime.memory.len();
          }

          runtime.pointer -= amount;
        }
      }
    }
  }

  Ok(())
}
