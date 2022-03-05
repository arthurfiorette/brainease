use crate::{
  logger,
  util::{match_indentation, strip_comments}, syntax::find_match,
};

use super::line_result::LineResult;

pub fn parse_instruction(
  file: &[&str],
  indentation: usize,
  line_index: usize,
) -> LineResult {
  let mut line = strip_comments(file[line_index]);

  if line.is_empty() {
    return LineResult::next_line(indentation, line_index);
  }

  if !match_indentation(indentation, line) {
    // This should never, REALLY, happen. But time may go backwards.
    if indentation < 2 {
      logger::unknown_indentation(&line_index, &indentation);

      return LineResult::error();
    }

    // Indentation has ended.
    // Parse, again, with one indentation less.
    return parse_instruction(file, indentation - 2, line_index);
  }

  // Clears indentation
  line = &line[indentation..];

  if let Some((token, captures)) = find_match(line) {
    let (next_line, instruction) =
      token.read_instruction(file, captures, line_index, indentation);

    return LineResult::new(instruction, next_line, indentation);
  }

  logger::unknown_line(&line_index, line);
  LineResult::error()
}
