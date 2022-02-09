use std::str::FromStr;

use lazy_regex::Captures;

use crate::{parser::parse_partial_file, util};

use super::{CellValue, Instruction};

/// A simple enum that represents the values of
/// `==`, `!=`, `<`, `<=`, `>` and `>=`.
#[derive(Debug, Clone, PartialEq)]
pub enum IfLogic {
  Equal,
  NotEqual,
  Less,
  Greater,
  LessOrEqual,
  GreaterOrEqual,
}

impl IfLogic {
  /// Runs the if logic with the given values.
  pub fn matches(&self, value: CellValue, other: CellValue) -> bool {
    match self {
      IfLogic::Equal => value == other,
      IfLogic::NotEqual => value != other,
      IfLogic::Less => value < other,
      IfLogic::Greater => value > other,
      IfLogic::LessOrEqual => value <= other,
      IfLogic::GreaterOrEqual => value >= other,
    }
  }

  pub fn parse(
    file: &[&str],
    captures: Captures,
    line_index: usize,
    indentation: usize,
  ) -> (usize, Option<Instruction>) {
    let first_cell = captures[1].parse().unwrap();
    let logic = IfLogic::from_str(&captures[2]).unwrap();

    let is_if_cell = file[line_index].matches('*').count() == 2;
    let value_or_cell = captures[if is_if_cell { 4 } else { 3 }].parse().unwrap();

    util::log_extra_chars(&line_index, &captures[5]);

    let (next_line, inner) = parse_partial_file(file, line_index + 1, indentation + 2, line_index);

    (
      next_line,
      Some(Instruction::If {
        cell: first_cell,
        logic,
        is_cell: is_if_cell,
        cell_or_value: value_or_cell,
        inner,
      }),
    )
  }
}

impl FromStr for IfLogic {
  type Err = ();

  /// Parses the given string into an if logic enum.
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "==" => Ok(IfLogic::Equal),
      "!=" => Ok(IfLogic::NotEqual),
      "<" => Ok(IfLogic::Less),
      ">" => Ok(IfLogic::Greater),
      "<=" => Ok(IfLogic::LessOrEqual),
      ">=" => Ok(IfLogic::GreaterOrEqual),
      _ => Err(()),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_if_equal() {
    let if_logic = IfLogic::Equal;
    assert_eq!(if_logic.matches(1, 1), true);
    assert_eq!(if_logic.matches(1, 2), false);
    assert_eq!(if_logic.matches(2, 1), false);
  }

  #[test]
  fn test_if_not_equal() {
    let if_logic = IfLogic::NotEqual;
    assert_eq!(if_logic.matches(1, 1), false);
    assert_eq!(if_logic.matches(1, 2), true);
    assert_eq!(if_logic.matches(2, 1), true);
  }

  #[test]
  fn test_if_less() {
    let if_logic = IfLogic::Less;
    assert_eq!(if_logic.matches(1, 1), false);
    assert_eq!(if_logic.matches(1, 2), true);
    assert_eq!(if_logic.matches(2, 1), false);
  }

  #[test]
  fn test_if_greater() {
    let if_logic = IfLogic::Greater;
    assert_eq!(if_logic.matches(1, 1), false);
    assert_eq!(if_logic.matches(1, 2), false);
    assert_eq!(if_logic.matches(2, 1), true);
  }

  #[test]
  fn test_if_less_or_equal() {
    let if_logic = IfLogic::LessOrEqual;
    assert_eq!(if_logic.matches(1, 1), true);
    assert_eq!(if_logic.matches(1, 2), true);
    assert_eq!(if_logic.matches(2, 1), false);
  }

  #[test]
  fn test_if_greater_or_equal() {
    let if_logic = IfLogic::GreaterOrEqual;
    assert_eq!(if_logic.matches(1, 1), true);
    assert_eq!(if_logic.matches(1, 2), false);
    assert_eq!(if_logic.matches(2, 1), true);
  }

  #[test]
  fn test_if_logic_from_str() {
    assert!(IfLogic::from_str("==").is_ok());
    assert!(IfLogic::from_str("!=").is_ok());
    assert!(IfLogic::from_str("<").is_ok());
    assert!(IfLogic::from_str(">").is_ok());
    assert!(IfLogic::from_str("<=").is_ok());
    assert!(IfLogic::from_str(">=").is_ok());

    assert!(IfLogic::from_str("").is_err());
    assert!(IfLogic::from_str("===").is_err());
  }
}
