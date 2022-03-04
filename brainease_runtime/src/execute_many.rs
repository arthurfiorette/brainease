use brainease_lexer::syntax::Instruction;

use crate::{executor::execute, io_handler::IoHandler, runtime::Runtime};

pub fn execute_many<I>(
  instructions: &[Instruction],
  runtime: &mut Runtime<I>,
) -> Result<(), I::Err>
where
  I: IoHandler,
{
  for instruction in instructions {
    execute(instruction, runtime)?;
  }

  // Ensure the output buffer is flushed every loop.
  runtime.io_handler.flush()?;

  Ok(())
}
