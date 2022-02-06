use super::{parse_instruction, FileResult};
use crate::headers::Headers;

pub fn parse_file(file: &[&str]) -> FileResult {
  let mut instructions = Vec::new();
  let headers = Headers::from_lines(file);

  let mut line_index = 0;

  while line_index < file.len() {
    let result = parse_instruction(file, 0, line_index);

    if let Some(instruction) = result.instruction {
      instructions.push(instruction);
    }

    line_index = result.next_line;
  }

  FileResult {
    headers,
    instructions,
  }
}
