use super::parse_instruction;
use crate::syntax::Instruction;

pub fn parse_file(file: &str) -> Vec<Instruction> {
  let lines: Vec<&str> = file.lines().collect();
  parse_partial_file(&lines, 0, 0, 0).1
}

/// Parses a file from a specified starting point. Returns the next line to parse and the
/// read instructions.
pub fn parse_partial_file(
  file: &[&str],
  starting_line: usize,
  starting_indentation: usize,
  last_contentful_line: usize,
) -> (usize, Vec<Instruction>) {
  let mut instructions = Vec::new();
  let mut line_index = starting_line;
  let mut indentation = starting_indentation;

  // This variable is used to keep track of the last contentful line
  // that was parsed, so if, after an indentation block, there is one or
  // more empty lines, we don't need to include them at the "next_line"
  // needed to be parsed.
  let mut last_contentful_line = last_contentful_line;

  while line_index < file.len() {
    let result = parse_instruction(file, indentation, line_index);

    // Indentation is over, we're done.
    if result.new_indentation < starting_indentation {
      break;
    }

    if let Some(instruction) = result.instruction {
      instructions.push(instruction);

      // line had content
      last_contentful_line = line_index;
    }

    line_index = result.next_line;
    indentation = result.new_indentation;
  }

  (last_contentful_line + 1, instructions)
}
