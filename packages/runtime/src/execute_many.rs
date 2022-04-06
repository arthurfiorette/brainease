use brainease_lexer::syntax::{BreakType, Instruction};

use crate::{executor::execute, io_handler::IoHandler, runtime::Runtime};

/// Returns true if the program should exit early.
/// Currently, this only occurs if there was a `break all` instruction
pub fn execute_many<I>(
  instructions: &[Instruction],
  runtime: &mut Runtime<I>,
  ignore_break_all: bool,
) -> Result<Option<BreakType>, I::Err>
where
  I: IoHandler,
{
  for instruction in instructions {
    // For any type of break type, just return early.
    if let Some(break_type) = execute(instruction, runtime)? {
      // Don't return BreakType::Break if it's the top level.
      if ignore_break_all && break_type == BreakType::BreakAll {
        continue;
      }

      return Ok(Some(break_type));
    }
  }

  // Ensure the output buffer is flushed every loop.
  runtime.io_handler.flush()?;

  Ok(None)
}
