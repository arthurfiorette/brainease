use brainease_lexer::syntax::Instruction;

/// Returns a valid brainease code.
pub fn transpile_code(code: &[Instruction]) -> String {
  transpile_block(code, 0).join("\n")
}

pub fn transpile_block(code: &[Instruction], initial_indentation: usize) -> Vec<String> {
  code
    .iter()
    .map(|i| transpile_instruction(i, initial_indentation))
    .collect()
}

/// Returns the instruction as raw string. Without
pub fn transpile_instruction(instruction: &Instruction, indentation: usize) -> String {
  use brainease_lexer::syntax::Instruction::*;

  let transpiled = match instruction {
    Break(exit) => exit.to_string(),

    Increment { cell, value } => {
      format!("inc {} in {}", value, cell.to_string())
    }

    Decrement { cell, value } => {
      format!("dec {} in {}", value, cell.to_string())
    }

    Move { current, next } => {
      format!("move {} to {}", current.to_string(), next.to_string())
    }

    Swap { from, with } => {
      format!("swap {} and {}", from.to_string(), with.to_string())
    }

    Save { cell, value } => {
      format!("save '{}' at {}", *value as char, cell.to_string())
    }

    Read(cell) => {
      format!("read {}", cell.to_string())
    }

    Write(cell) => {
      format!("write {}", cell.to_string())
    }

    Print(cell) => {
      format!("print {}", cell.to_string())
    }

    Goto { dir, by } => {
      if let Some(by) = by {
        format!("goto {} by {}", dir.to_string(), by.to_string())
      } else {
        format!("goto {}", dir.to_string())
      }
    }

    Loop { cell, inner } => {
      let parsed = transpile_block(inner, indentation + 2);
      let indented = parsed.join("\n");

      format!("loop {}\n{}", cell.to_string(), indented)
    }

    If {
      cell,
      cell_or_value,
      logic,
      inner,
      ..
    } => {
      let parsed = transpile_block(inner, indentation + 2);
      let indented = parsed.join("\n");

      format!(
        "if {} {} {}\n{}",
        cell.to_string(),
        logic.to_string(),
        cell_or_value.to_string(),
        indented
      )
    }
  };

  " ".repeat(indentation) + &transpiled
}
