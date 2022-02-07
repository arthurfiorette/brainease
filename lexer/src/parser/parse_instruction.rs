use super::LineResult;
use crate::{
  logger,
  syntax::{TokenKind, TokenParser},
  util::{is_empty_line, match_indentation},
};

pub fn parse_instruction(
  file: &[&str],
  current_indentation: usize,
  line_index: usize,
) -> LineResult {
  let mut line = file[line_index];

  if is_empty_line(line) {
    return LineResult {
      new_indentation: current_indentation,
      instruction: None,
      next_line: line_index + 1,
    };
  }

  // Ended indentation block, just return what we got.
  if !match_indentation(current_indentation, line) {
    return LineResult {
      new_indentation: current_indentation - 2,
      instruction: None,
      next_line: line_index + 1,
    };
  }

  // Clears indentation
  line = &line[current_indentation..];

  if let Some((token, captures)) = TokenKind::find_match(line) {
    let parser: TokenParser = token.parser();

    let (next_line, instruction) =
      parser(file, captures, line_index, current_indentation);

    return LineResult {
      instruction,
      next_line,
      new_indentation: current_indentation,
    };
  }

  logger::unknown_line(&line_index, line);

  LineResult {
    new_indentation: current_indentation,
    instruction: None,
    next_line: line_index + 1,
  }
}
