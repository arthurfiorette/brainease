// TODO: Trocar: to_string_indent nativo e to_string chama to_string_indent com 0
// Calcular os indent do loop e if como: (indentAtual + 2), em véz de somente 2

use brainease_lexer::syntax::Instruction;

pub trait AsString {
  fn as_string(&self, indent: usize) -> String;

  fn all(all: &[Self], indent: usize) -> Vec<String>
  where
    Self: Sized,
  {
    all.iter().map(|i| i.as_string(indent)).collect::<Vec<_>>()
  }
}

impl AsString for Instruction {
  fn as_string(&self, indent: usize) -> String {
    let raw = match self {
      Instruction::Increment { cell, value } => {
        format!("inc {} in {}", value, cell.to_string())
      }

      Instruction::Decrement { cell, value } => {
        format!("dec {} in {}", value, cell.to_string())
      }

      Instruction::Move { current, next } => {
        format!("move {} to {}", current.to_string(), next.to_string())
      }

      Instruction::Swap { from, with } => {
        format!("swap {} and {}", from.to_string(), with.to_string())
      }

      Instruction::Save { cell, value } => {
        format!("save '{}' at {}", *value as char, cell.to_string())
      }

      Instruction::Read(cell) => {
        format!("read {}", cell.to_string())
      }

      Instruction::Write(cell) => {
        format!("write {}", cell.to_string())
      }

      Instruction::Print(cell) => {
        format!("print {}", cell.to_string())
      }

      Instruction::Goto { dir, by } => {
        if let Some(by) = by {
          format!("goto {} by {}", dir.to_string(), by.to_string())
        } else {
          format!("goto {}", dir.to_string())
        }
      }

      Instruction::Loop { cell, inner } => {
        let parsed = AsString::all(inner, indent + 2);
        format!("loop {}\n{}", cell.to_string(), parsed.join("\n"))
      }

      Instruction::If {
        cell,
        cell_or_value,
        logic,
        inner,
        ..
      } => {
        let parsed = AsString::all(inner, indent + 2);
        format!(
          "\nif {} {} {}\n{}",
          cell.to_string(),
          logic.to_string(),
          cell_or_value.to_string(),
          parsed.join("\n")
        )
      }
    };

    " ".repeat(indent) + &raw
  }
}

#[cfg(test)]
mod tests {
  use brainease_lexer::{
    parser::parse_file,
    syntax::{CellOrPointer, GotoBy, GotoDirection},
  };

  use super::*;

  #[test]
  fn instructions_two_ways() {
    let instructions = [
      Instruction::Increment {
        cell: CellOrPointer::Pointer,
        value: 1,
      },
      Instruction::Decrement {
        cell: CellOrPointer::Pointer,
        value: 1,
      },
      Instruction::Move {
        current: CellOrPointer::Pointer,
        next: CellOrPointer::Pointer,
      },
      Instruction::Swap {
        from: CellOrPointer::Pointer,
        with: CellOrPointer::Pointer,
      },
      Instruction::Save {
        cell: CellOrPointer::Pointer,
        value: 1,
      },
      Instruction::Read(CellOrPointer::Pointer),
      Instruction::Write(CellOrPointer::Pointer),
      Instruction::Print(CellOrPointer::Pointer),
      Instruction::Goto {
        dir: GotoDirection::Right,
        by: Some(GotoBy::Number(1)),
      },
      Instruction::Goto {
        dir: GotoDirection::Left,
        by: Some(GotoBy::Number(1)),
      },
      Instruction::Goto {
        dir: GotoDirection::Right,
        by: Some(GotoBy::Cell(1)),
      },
      Instruction::Goto {
        dir: GotoDirection::Left,
        by: Some(GotoBy::Cell(1)),
      },
      Instruction::Goto {
        dir: GotoDirection::Right,
        by: None,
      },
      Instruction::Goto {
        dir: GotoDirection::Left,
        by: None,
      },
      Instruction::Loop {
        cell: CellOrPointer::Pointer,
        inner: vec![Instruction::Write(CellOrPointer::Pointer)],
      },
      Instruction::Loop {
        cell: CellOrPointer::Cell(1),
        inner: vec![Instruction::Write(CellOrPointer::Pointer)],
      },
    ];

    for instruction in instructions {
      let kind = instruction.kind();

      // Always equals
      assert_eq!(instruction.as_string(0), instruction.as_string(0));

      // Regex matches the generated string
      assert!(
        kind.regex().is_match(&instruction.as_string(0)),
        "Regex for {:?} is not matching",
        kind
      );

      // His string can be parsed back to the same instruction
      assert_eq!(
        parse_file(&instruction.as_string(0)),
        vec![instruction],
        "Parser for {:?} is not working",
        kind
      );
    }
  }
}
