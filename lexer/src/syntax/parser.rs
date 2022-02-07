use lazy_regex::Captures;

use super::Instruction;

/// A closure that parses a line of code into a `Instruction`.
///
/// Returns the next line to parse and the optional instruction.
pub type TokenParser = fn(
  file: &[&str],
  captures: Captures,
  line_index: usize,
  indentation: usize,
) -> (usize, Option<Instruction>);

pub static INCREMENT: TokenParser = |_file, _captures, _line_index, _indentation| {
  todo!();
};

pub static DECREMENT: TokenParser = |_file, _captures, _line_index, _indentation| {
  todo!();
};

pub static MOVE: TokenParser = |_file, _captures, _line_index, _indentation| {
  todo!();
};

pub static SWAP: TokenParser = |_file, _captures, _line_index, _indentation| {
  todo!();
};

pub static SAVE: TokenParser = |_file, _captures, _line_index, _indentation| {
  todo!();
};

pub static READ: TokenParser = |_file, _captures, _line_index, _indentation| {
  todo!();
};

pub static WRITE: TokenParser = |_file, _captures, _line_index, _indentation| {
  todo!();
};

pub static PRINT: TokenParser = |_file, _captures, _line_index, _indentation| {
  todo!();
};

pub static LOOP: TokenParser = |_file, _captures, _line_index, _indentation| {
  todo!();
};

pub static IF: TokenParser = |_file, _captures, _line_index, _indentation| {
  todo!();
};

pub static IF_CELL: TokenParser = |_file, _captures, _line_index, _indentation| {
  todo!();
};
