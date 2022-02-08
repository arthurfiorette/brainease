use std::time::Instant;

use brainease_lexer::syntax::{CellValue, Instruction};

use crate::{
  executor::execute,
  io_handler::{DefaultIoHandler, IoHandler},
};

/// A struct representing the result after parsing a whole instruction.
/// Which may or may not already started or ended.
#[derive(Clone)]
pub struct Runtime {
  pub instructions: Vec<Instruction>,
  pub current_instruction: usize,
  pub(crate) memory: Vec<CellValue>,
  pub io_handler: &'static dyn IoHandler,
}

impl Runtime {
  pub fn new(instructions: Vec<Instruction>, memory_length: usize) -> Runtime {
    Runtime::build(instructions, memory_length, &DefaultIoHandler {})
  }

  pub fn build(
    instructions: Vec<Instruction>,
    memory_length: usize,
    io_handler: &'static dyn IoHandler,
  ) -> Runtime {
    Runtime {
      instructions,
      memory: vec![0; memory_length],
      current_instruction: 0,
      io_handler,
    }
  }

  /// Runs his instructions and return the elapsed time in seconds.
  pub fn run(&mut self) -> f64 {
    let now = Instant::now();
    let runtime = &mut self.clone();

    for index in 0..self.instructions.len() {
      self.current_instruction = index;
      execute(&self.instructions[index], runtime);
    }

    now.elapsed().as_secs_f64()
  }
}
