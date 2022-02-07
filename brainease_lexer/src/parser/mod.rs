mod file;
mod parse_instruction;

pub use file::*;
pub use parse_instruction::*;

use crate::syntax::Instruction;

/// A struct representing the result after parsing a whole instruction
#[derive(Debug, Clone, PartialEq)]
pub struct LineResult {
  pub new_indentation: usize,
  pub instruction: Option<Instruction>,
  pub next_line: usize,
}
