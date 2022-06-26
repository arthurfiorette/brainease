pub mod bf_parser;
pub mod cleaner;
pub mod transpiler;

pub use transpiler::transpile_code;

#[cfg(test)]
mod tests {
  use crate::{bf_parser, transpiler::transpile_block};
  use brainease_lexer::syntax::{
    CellOrChar, CellOrPointer, GotoBy, GotoDirection, Instruction,
  };

  #[test]
  fn parse_bf() {
    let code = ">>[->.+,]-.,";
    let raw_length = 5;
    let raw_ins = vec![
      Instruction::Goto {
        by: Some(GotoBy::Number(2)),
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

          Instruction::Print(CellOrChar::pointer()),
          
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

      Instruction::Print(CellOrChar::pointer()),
      
      Instruction::Read(CellOrPointer::Pointer),
    ];

    let ins = bf_parser::parse_bf(code);

    assert_eq!(ins.len(), raw_length);
    assert_eq!(ins, raw_ins);

    let stringified_ins = transpile_block(&ins, 0);

    assert_eq!(stringified_ins.len(), raw_length);

    let parsed_instructions =
      brainease_lexer::parser::parse_file(&stringified_ins.join("\n"));

    assert_eq!(parsed_instructions.len(), raw_length);
    assert_eq!(parsed_instructions, raw_ins);
  }
}
