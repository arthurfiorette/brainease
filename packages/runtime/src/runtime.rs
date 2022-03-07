use std::time::{Duration, Instant};

use crate::{execute_many::execute_many, io_handler::IoHandler};
use brainease_lexer::syntax::{BreakType, CellPosition, CellValue, Instruction};

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

    // Exit early can be ignored because the program will exit anyway.
    if let Some(break_type) = execute_many(&self.instructions.clone(), self)? {
      match break_type {
        // Continue is a no-op
        BreakType::Continue => {
          log::error!("There's a continue outside of a loop.")
        }

        // The program has already exited. No need to do anything.
        BreakType::Exit | BreakType::Break => {}
      }
    }

    self.io_handler.flush()?;

    Ok(now.elapsed())
  }
}
