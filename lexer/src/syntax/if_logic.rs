use std::str::FromStr;

use super::CellValue;

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
