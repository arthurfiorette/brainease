pub mod as_string;
pub mod create_instruction;
pub mod filter;
pub mod parse_loop;
pub mod parser;

#[cfg(test)]
mod tests {
  use brainease_lexer::syntax::{CellOrPointer, GotoDirection, Instruction};

  use crate::as_string::AsString;

  use super::*;

  #[test]
  fn parse_bf() {
    let code = ">>[->.+,]-.,";
    let raw_ins = vec![
      Instruction::Goto {
        by: None,
        dir: GotoDirection::Right,
      },
      Instruction::Goto {
        by: None,
        dir: GotoDirection::Right,
      },
      Instruction::Loop {
        cell: CellOrPointer::Pointer,
        inner: vec![
          Instruction::Decrement {
            cell: CellOrPointer::Pointer,
            value: 1,
          },
          Instruction::Goto {
            by: None,
            dir: GotoDirection::Right,
          },
          Instruction::Print(CellOrPointer::Pointer),
          Instruction::Increment {
            cell: CellOrPointer::Pointer,
            value: 1,
          },
          Instruction::Read(CellOrPointer::Pointer),
        ],
      },
      Instruction::Decrement {
        cell: CellOrPointer::Pointer,
        value: 1,
      },
      Instruction::Print(CellOrPointer::Pointer),
      Instruction::Read(CellOrPointer::Pointer),
    ];

    let ins = parser::parse_bf(code);

    assert_eq!(ins.len(), 6);
    assert_eq!(ins, raw_ins);

    let stringified_ins = AsString::all(&ins, 0);

    assert_eq!(stringified_ins.len(), 6);

    let parsed_instructions =
      brainease_lexer::parser::parse_file(&stringified_ins.join("\n"));

    assert_eq!(parsed_instructions.len(), 6);
    assert_eq!(parsed_instructions, raw_ins);
  }
}
