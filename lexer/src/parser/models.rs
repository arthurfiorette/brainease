use crate::{headers::Headers, syntax::Instruction};

/// A struct with the result of parsing a single file
#[derive(Debug, Clone, PartialEq)]
pub struct FileResult {
  /// All present headers
  pub headers: Headers,

  /// The group of top-level instructions
  pub instructions: Vec<Instruction>,
}

/// A struct representing the result after parsing a whole instruction
#[derive(Debug, Clone, PartialEq)]
pub struct LineResult {
  pub new_indentation: usize,
  pub instruction: Option<Instruction>,
  pub next_line: usize,
}
