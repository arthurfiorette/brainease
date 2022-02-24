use brainease_lexer::syntax::{CellOrPointer, Instruction};

use crate::create_instruction::create_instruction;

/// Return all read instructions and the next char to be read
/// The index may be the next value after the [ (loop start) character
///
/// Example: `[-]` -> Index can be `0` or `1`
pub fn parse_loop(chars: &[char], index: usize) -> (Instruction, usize) {
  let mut instructions = vec![];
  let mut index = index;

  // Parse loop called as first parser. This way, we avoid returning
  // a Instruction::Loop (Instruction::Loop ( Instruction::Decrement ))
  // for the case "[+]".
  if chars[index] == '[' {
    index += 1;
  }

  while index < chars.len() {
    match chars[index] {
      // Loop end
      ']' => {
        index += 1;
        break;
      }

      // New loop
      '[' => {
        let (loop_instruction, next_index) = parse_loop(chars, index + 1);
        instructions.push(loop_instruction);
        index = next_index;
      }

      char => {
        let ins = create_instruction(&char);

        if let Some(bf_char) = ins {
          instructions.push(bf_char);
        }

        index += 1;
      }
    }
  }

  (
    Instruction::Loop {
      cell: CellOrPointer::Pointer,
      inner: instructions,
    },
    index,
  )
}

#[cfg(test)]
pub mod tests {
  use brainease_lexer::syntax::CellOrPointer;

  use crate::filter::filter_bf_chars;

  use super::*;

  #[test]
  fn simple_loop() {
    let chars = filter_bf_chars("[+]");
    let (ins, next_index) = parse_loop(&chars, 0);

    assert_eq!(next_index, 3);
    assert_eq!(next_index, chars.len());

    assert_eq!(
      ins,
      Instruction::Loop {
        cell: CellOrPointer::Pointer,
        inner: vec![Instruction::Increment {
          cell: CellOrPointer::Pointer,
          value: 1,
        }],
      }
    );
  }

  #[test]
  fn big_loop() {
    let chars = filter_bf_chars("[+[-]]");
    let (ins, next_index) = parse_loop(&chars, 0);

    assert_eq!(next_index, 6);
    assert_eq!(next_index, chars.len());

    assert_eq!(
      ins,
      Instruction::Loop {
        cell: CellOrPointer::Pointer,
        inner: vec![
          Instruction::Increment {
            cell: CellOrPointer::Pointer,
            value: 1,
          },
          Instruction::Loop {
            cell: CellOrPointer::Pointer,
            inner: vec![Instruction::Decrement {
              cell: CellOrPointer::Pointer,
              value: 1,
            }],
          }
        ],
      }
    );
  }

  #[test]
  fn loop_and_then_increment() {
    let chars = filter_bf_chars("[+]++");
    let (ins, next_index) = parse_loop(&chars, 0);

    assert_eq!(next_index, 3);

    assert_eq!(
      ins,
      Instruction::Loop {
        cell: CellOrPointer::Pointer,
        inner: vec![Instruction::Increment {
          cell: CellOrPointer::Pointer,
          value: 1,
        }],
      }
    );
  }
}
