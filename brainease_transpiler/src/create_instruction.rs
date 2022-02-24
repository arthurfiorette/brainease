use brainease_lexer::syntax::{CellOrPointer, GotoDirection, Instruction};

pub fn create_instruction(char: &char) -> Option<Instruction> {
  match char {
    '>' => Some(Instruction::Goto {
      by: None,
      dir: GotoDirection::Right,
    }),

    '<' => Some(Instruction::Goto {
      by: None,
      dir: GotoDirection::Left,
    }),

    '+' => Some(Instruction::Increment {
      cell: CellOrPointer::Pointer,
      value: 1,
    }),

    '-' => Some(Instruction::Decrement {
      cell: CellOrPointer::Pointer,
      value: 1,
    }),

    '.' => Some(Instruction::Print(CellOrPointer::Pointer)),
    ',' => Some(Instruction::Read(CellOrPointer::Pointer)),

    // Loops should not get matched by this function
    _ => None,
  }
}

#[cfg(test)]
pub mod tests {
  use super::*;

  #[test]
  fn normal_chars() {
    for char in ['>', '<', '+', '-', '.', ','] {
      let ins = create_instruction(&char);

      assert!(ins.is_some());
    }
  }

  #[test]
  fn loop_chars() {
    for loop_char in ['[', ']'] {
      let ins = create_instruction(&loop_char);

      assert!(ins.is_none());
    }
  }
}
