use std::process;

use super::LineResult;
use crate::{
  logger,
  syntax::{TokenKind, TokenParser},
  util::{is_empty_line, match_indentation},
};

pub fn parse_instruction(
  file: &[&str],
  indentation: usize,
  line_index: usize,
) -> LineResult {
  let mut line = file[line_index];

  if is_empty_line(line) {
    return LineResult {
      new_indentation: indentation,
      instruction: None,
      next_line: line_index + 1,
    };
  }

  // Indentation was over, try again with one indentation less.
  if !match_indentation(indentation, line) {
    // This should never, REALLY, happen.
    // But time may go backwards.
    if indentation < 2 {
      logger::unknown_indentation(&line_index, &indentation);
      process::exit(1);
    }

    return LineResult {
      new_indentation: indentation - 2,
      instruction: None,
      next_line: line_index,
    };
  }

  // Clears indentation
  line = &line[indentation..];

  if let Some((token, captures)) = TokenKind::find_match(line) {
    let parser: TokenParser = token.parser();

    let (next_line, instruction) = parser(file, captures, line_index, indentation);

    return LineResult {
      instruction,
      next_line,
      new_indentation: indentation,
    };
  }

  logger::unknown_line(&line_index, line);

  LineResult {
    new_indentation: indentation,
    instruction: None,
    next_line: line_index + 1,
  }
}
