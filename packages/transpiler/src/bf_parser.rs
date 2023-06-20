use crate::cleaner::clean_bf_code;
use brainease_lexer::syntax::{
  CellOrChar, CellOrPointer, GotoBy, GotoDirection, Instruction,
};

pub fn parse_bf(code: &str) -> Vec<Instruction> {
  let cleaned = clean_bf_code(code);

  let mut instructions = Vec::new();
  let mut index = 0;

  while index < cleaned.len() {
    let (instruction, next_index) = parse_bf_code(&cleaned, index);

    index = next_index;

    if let Some(instruction) = instruction {
      instructions.push(instruction);
    }
  }

  instructions
}

pub fn parse_bf_code(code: &[u8], index: usize) -> (Option<Instruction>, usize) {
  let char = code[index];
  let mut index = index;

  match char {
    // Unotimizable
    b'.' => (Some(Instruction::Print(CellOrChar::pointer())), index + 1),

    b',' => (Some(Instruction::Read(CellOrPointer::Pointer)), index + 1),

    // Groupable
    b'>' | b'<' => {
      index += 1;
      let mut count = 1;

      while index < code.len() && code[index] == char {
        index += 1;
        count += 1;
      }

      (
        Some(Instruction::Goto {
          dir: if char == b'>' {
            GotoDirection::Right
          } else {
            GotoDirection::Left
          },
          by: if count == 1 {
            None
          } else {
            Some(GotoBy::Number(count))
          },
        }),
        index,
      )
    }

    b'+' | b'-' => {
      index += 1;
      let mut count = 1;

      while index < code.len() && code[index] == char {
        index += 1;
        count += 1;
      }

      if char == b'+' {
        (
          Some(Instruction::Increment {
            cell: CellOrPointer::Pointer,
            value: count,
          }),
          index,
        )
      } else {
        (
          Some(Instruction::Decrement {
            cell: CellOrPointer::Pointer,
            value: count,
          }),
          index,
        )
      }
    }

    // Loops
    b'[' => {
      index += 1;
      let mut inners = Vec::new();

      // While loop hasn't ended
      while index < code.len() {
        if code[index] == b']' {
          index += 1;
          break;
        }

        let (inner, next_index) = parse_bf_code(code, index);

        index = next_index;

        if let Some(inner) = inner {
          inners.push(inner);
        }
      }

      (
        Some(Instruction::Loop {
          cell: CellOrPointer::Pointer,
          inner: inners,
        }),
        index,
      )
    }

    // Ignore unknown character
    // May be an ] (End of loop), in this case, just ignore it.
    _ => (None, index + 1),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn goto_by_one_is_none() {
    assert_eq!(
      parse_bf(">"),
      vec![Instruction::Goto {
        dir: GotoDirection::Right,
        by: None,
      }]
    );

    assert_eq!(
      parse_bf(">>"),
      vec![Instruction::Goto {
        dir: GotoDirection::Right,
        by: Some(GotoBy::Number(2)),
      }]
    );

    assert_eq!(
      parse_bf("<"),
      vec![Instruction::Goto {
        dir: GotoDirection::Left,
        by: None,
      }]
    );

    assert_eq!(
      parse_bf("<<"),
      vec![Instruction::Goto {
        dir: GotoDirection::Left,
        by: Some(GotoBy::Number(2)),
      }]
    );
  }
}
