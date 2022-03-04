use crate::cleaner::clean_bf_code;
use brainease_lexer::syntax::{CellOrPointer, GotoBy, GotoDirection, Instruction};

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

// pub fn transpile_cleaned(code: &[u8], start_index: usize) -> (Vec<Instruction>, usize) {}

pub fn parse_bf_code(code: &[u8], index: usize) -> (Option<Instruction>, usize) {
  let mut index = index;

  match code[index] {
    // Unotimizable
    b'.' => (Some(Instruction::Print(CellOrPointer::Pointer)), index + 1),

    b',' => (Some(Instruction::Read(CellOrPointer::Pointer)), index + 1),

    // Groupable
    b'>' => {
      index += 1;
      let mut count = 1;

      while index < code.len() && code[index] == b'>' {
        index += 1;
        count += 1;
      }

      (
        Some(Instruction::Goto {
          dir: GotoDirection::Right,
          by: if count == 1 {
            None
          } else {
            Some(GotoBy::Number(count))
          },
        }),
        index,
      )
    }

    b'<' => {
      index += 1;
      let mut count = 1;

      while index < code.len() && code[index] == b'<' {
        index += 1;
        count += 1;
      }

      (
        Some(Instruction::Goto {
          dir: GotoDirection::Left,
          by: if count == 1 {
            None
          } else {
            Some(GotoBy::Number(count))
          },
        }),
        index,
      )
    }

    b'+' => {
      index += 1;
      let mut count = 1;

      while index < code.len() && code[index] == b'+' {
        index += 1;
        count += 1;
      }

      (
        Some(Instruction::Increment {
          cell: CellOrPointer::Pointer,
          value: count,
        }),
        index,
      )
    }

    b'-' => {
      index += 1;
      let mut count = 1;

      while index < code.len() && code[index] == b'-' {
        index += 1;
        count += 1;
      }

      (
        Some(Instruction::Decrement {
          cell: CellOrPointer::Pointer,
          value: count,
        }),
        index,
      )
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
