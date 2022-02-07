use super::parse_instruction;
use crate::syntax::Instruction;

pub fn parse_file(file: &[&str]) -> Vec<Instruction> {
  parse_partial_file(file, 0, 0).1
}

/// Parses a file from a specified starting point. Returns the next line to parse and the
/// read instructions.
pub fn parse_partial_file(
  file: &[&str],
  starting_line: usize,
  starting_indentation: usize,
) -> (usize, Vec<Instruction>) {
  let mut instructions = Vec::new();

  let mut line_index = starting_line;

  while line_index < file.len() {
    let result = parse_instruction(file, starting_indentation, line_index);

    // Indentation is over, we're done.
    if result.new_indentation < starting_indentation {
      break;
    }

    if let Some(instruction) = result.instruction {
      instructions.push(instruction);
    }

    line_index = result.next_line;
  }

  (line_index + 1, instructions)
}
