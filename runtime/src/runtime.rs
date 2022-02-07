use std::time::Instant;

use crate::executor::execute;
use brainease_lexer::syntax::{CellValue, Instruction};

/// A struct representing the result after parsing a whole instruction.
/// Which may or may not already started or ended.
#[derive(Clone, Debug)]
pub struct Runtime {
  pub instructions: Vec<Instruction>,
  pub current_instruction: usize,
  pub(crate) memory: Vec<CellValue>,
}

impl Runtime {
  pub fn new(instructions: Vec<Instruction>, memory_length: usize) -> Runtime {
    Runtime {
      instructions,
      memory: vec![0; memory_length],
      current_instruction: 0,
    }
  }

  /// Runs his instructions and return the elapsed time in seconds.
  pub fn run(&mut self) -> f64 {
    let instructions = self.instructions.iter().enumerate();

    let now = Instant::now();
    let runtime = &mut self.clone();

    for (index, instruction) in instructions {
      self.current_instruction = index;
      execute(instruction, runtime);
    }

    now.elapsed().as_secs_f64()
  }

  pub fn get_memory(&self) -> Vec<CellValue> {
    self.memory.clone()
  }
}
