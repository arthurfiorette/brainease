use std::time::Instant;

use brainease_lexer::syntax::{CellValue, Instruction, CellPosition};

use crate::{
  executor::execute,
  io_handler::{DefaultIoHandler, IoHandler},
};

/// A struct representing the result after parsing a whole instruction.
/// Which may or may not already started or ended.
#[derive(Clone)]
pub struct Runtime<I>
where
  I: IoHandler + Clone,
{
  pub instructions: Vec<Instruction>,
  pub pointer: CellPosition,
  pub(crate) memory: Vec<CellValue>,
  pub io_handler: Box<I>,
}

impl<I: Clone> Runtime<I>
where
  I: IoHandler,
{
  pub fn new(
    instructions: Vec<Instruction>,
    memory_length: usize,
  ) -> Runtime<DefaultIoHandler> {
    Runtime::build(instructions, memory_length, DefaultIoHandler {})
  }

  pub fn build(
    instructions: Vec<Instruction>,
    memory_length: usize,
    io_handler: I,
  ) -> Runtime<I> {
    Runtime {
      pointer: 0,
      instructions,
      memory: vec![0; memory_length],
      io_handler: Box::new(io_handler),
    }
  }

  /// Runs his instructions and return the elapsed time in seconds.
  pub fn run(&mut self) -> f64 {
    let now = Instant::now();

    let mut current_instruction = 0;
    let runtime = &mut self.clone();

    while current_instruction < self.instructions.len() {
      execute(&self.instructions[current_instruction], runtime);
      current_instruction += 1;
    }

    // Please help. This should not be right
    self.io_handler = runtime.io_handler.clone();
    self.memory = runtime.memory.clone();
    self.instructions = runtime.instructions.clone();

    now.elapsed().as_secs_f64()
  }
}
