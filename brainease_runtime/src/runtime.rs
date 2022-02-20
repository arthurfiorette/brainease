use std::time::{Duration, Instant};

use crate::{executor::execute, io_handler::IoHandler};
use brainease_lexer::syntax::{CellPosition, CellValue, Instruction};

/// A struct representing the result after parsing a whole instruction.
/// Which may or may not already started or ended.
#[derive(Clone)]
pub struct Runtime<I: IoHandler> {
  pub instructions: Vec<Instruction>,
  pub pointer: CellPosition,
  pub(crate) memory: Vec<CellValue>,
  pub io_handler: I,
}

impl<I: IoHandler> Runtime<I> {
  pub fn new(
    instructions: Vec<Instruction>,
    memory_length: usize,
    io_handler: I,
  ) -> Runtime<I> {
    Runtime {
      pointer: 0,
      instructions,
      memory: vec![0; memory_length],
      io_handler,
    }
  }

  /// Runs his instructions and return the elapsed time in seconds.
  pub fn run(&mut self) -> Result<Duration, I::Err> {
    let now = Instant::now();

    let mut current_instruction = 0;
    let runtime = &mut self.clone();

    while current_instruction < self.instructions.len() {
      execute(&self.instructions[current_instruction], runtime)?;
      current_instruction += 1;
    }

    self.io_handler.flush()?;

    // FIXME: Preserve state after execution.
    // Without having to clone everything :P
    self.io_handler = runtime.io_handler.clone();
    self.memory = runtime.memory.clone();
    self.instructions = runtime.instructions.clone();

    Ok(now.elapsed())
  }
}
