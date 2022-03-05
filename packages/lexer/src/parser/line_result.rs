use crate::syntax::Instruction;

/// A struct representing the result after parsing a whole instruction
#[derive(Debug, Clone, PartialEq)]
pub struct LineResult {
  pub new_indentation: usize,
  pub instruction: Option<Instruction>,
  pub next_line: usize,
  pub error: bool,
}

impl LineResult {
  pub fn next_line(indentation: usize, current_line: usize) -> LineResult {
    LineResult {
      new_indentation: indentation,
      instruction: None,
      next_line: current_line + 1,
      error: false,
    }
  }

  pub fn new(
    instruction: Option<Instruction>,
    next_line: usize,
    new_indentation: usize,
  ) -> LineResult {
    LineResult {
      instruction,
      new_indentation,
      next_line,
      error: false,
    }
  }

  pub fn error() -> LineResult {
    LineResult {
      new_indentation: 0,
      instruction: None,
      next_line: 0,
      error: true,
    }
  }
}
