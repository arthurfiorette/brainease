use brainease_lexer::syntax::Instruction;

use crate::{
  create_instruction::create_instruction, filter::filter_bf_chars, parse_loop::parse_loop,
};

pub fn parse_bf(bf_code: &str) -> Vec<Instruction> {
  let chars = filter_bf_chars(bf_code);

  let mut instructions = Vec::new();
  let mut index = 0;

  while index < chars.len() {
    let char = chars[index];

    if char == '[' {
      let (loop_instruction, new_index) = parse_loop(&chars, index + 1);
      instructions.push(loop_instruction);
      index = new_index;
      continue;
    }

    let ins = create_instruction(&char);

    if let Some(bf_char) = ins {
      instructions.push(bf_char);
    }

    index += 1;
  }

  instructions
}

#[cfg(test)]
pub mod tests {
  use super::*;
  use brainease_lexer::syntax::{CellOrPointer, GotoDirection};

  #[test]
  fn simple_loop() {
    let ins = parse_bf("[+]");

    assert_eq!(ins.len(), 1);

    assert_eq!(
      ins,
      vec![Instruction::Loop {
        cell: CellOrPointer::Pointer,
        inner: vec![Instruction::Increment {
          cell: CellOrPointer::Pointer,
          value: 1,
        }],
      }]
    );
  }

  #[test]
  fn big_loop() {
    let ins = parse_bf("[+[-]]");

    assert_eq!(ins.len(), 1);

    assert_eq!(
      ins,
      vec![Instruction::Loop {
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
      }]
    );
  }

  #[test]
  fn loop_and_then_increment() {
    let ins = parse_bf("[+]++");

    // assert_eq!(ins.len(), 3);

    assert_eq!(
      ins,
      vec![
        Instruction::Loop {
          cell: CellOrPointer::Pointer,
          inner: vec![Instruction::Increment {
            cell: CellOrPointer::Pointer,
            value: 1,
          }],
        },
        Instruction::Increment {
          cell: CellOrPointer::Pointer,
          value: 1,
        },
        Instruction::Increment {
          cell: CellOrPointer::Pointer,
          value: 1,
        }
      ]
    );
  }

  #[test]
  fn test_all_instructions() {
    let ins = parse_bf(".,+-<>[],>");

    assert_eq!(ins.len(), 9);

    assert_eq!(
      ins,
      vec![
        Instruction::Print(CellOrPointer::Pointer),
        Instruction::Read(CellOrPointer::Pointer),
        Instruction::Increment {
          cell: CellOrPointer::Pointer,
          value: 1,
        },
        Instruction::Decrement {
          cell: CellOrPointer::Pointer,
          value: 1,
        },
        Instruction::Goto {
          by: None,
          dir: GotoDirection::Left,
        },
        Instruction::Goto {
          by: None,
          dir: GotoDirection::Right,
        },
        Instruction::Loop {
          cell: CellOrPointer::Pointer,
          inner: vec![],
        },
        Instruction::Read(CellOrPointer::Pointer),
        Instruction::Goto {
          by: None,
          dir: GotoDirection::Right,
        }
      ]
    );
  }
}
