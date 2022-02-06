use crate::util::{is_empty_line, match_indentation};

use super::LineResult;

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

  println!("{}, {}", line, line_index);
  todo!();
}
