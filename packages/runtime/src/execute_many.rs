use brainease_lexer::syntax::Instruction;

use crate::{executor::execute, io_handler::IoHandler, runtime::Runtime};

/// Returns true if the program should exit early.
/// Currently, this only occurs if there was a `break all` instruction
pub fn execute_many<I>(
  instructions: &[Instruction],
  runtime: &mut Runtime<I>,
) -> Result<bool, I::Err>
where
  I: IoHandler,
{
  let mut should_exit = false;

  for instruction in instructions {
    if let Instruction::Break(exit) = instruction {
      should_exit = *exit;
      break;
    }

    // Execute other instructions
    should_exit = execute(instruction, runtime)?;

    if should_exit {
      break;
    }
  }

  // Ensure the output buffer is flushed every loop.
  runtime.io_handler.flush()?;

  Ok(should_exit)
}
