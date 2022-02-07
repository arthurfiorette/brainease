use super::parse_instruction;
use crate::syntax::Instruction;

pub fn parse_file(file: &[&str]) -> Vec<Instruction> {
  let mut instructions = Vec::new();

  let mut line_index = 0;

  while line_index < file.len() {
    let result = parse_instruction(file, 0, line_index);

    if let Some(instruction) = result.instruction {
      instructions.push(instruction);
    }

    line_index = result.next_line;
  }

  instructions
}
