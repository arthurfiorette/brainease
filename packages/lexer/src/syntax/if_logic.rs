use std::str::FromStr;

use super::CellValue;

/// A simple enum that represents the values of
/// `==`, `!=`, `<`, `<=`, `>` and `>=`.
#[derive(Debug, Clone, PartialEq, Eq)]
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

impl ToString for IfLogic {
  fn to_string(&self) -> String {
    match self {
      IfLogic::Equal => "==".to_string(),
      IfLogic::NotEqual => "!=".to_string(),
      IfLogic::Less => "<".to_string(),
      IfLogic::Greater => ">".to_string(),
      IfLogic::LessOrEqual => "<=".to_string(),
      IfLogic::GreaterOrEqual => ">=".to_string(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_if_equal() {
    let if_logic = IfLogic::Equal;
    assert!(if_logic.matches(1, 1));
    assert!(!if_logic.matches(1, 2));
    assert!(!if_logic.matches(2, 1));
  }

  #[test]
  fn test_if_not_equal() {
    let if_logic = IfLogic::NotEqual;
    assert!(!if_logic.matches(1, 1));
    assert!(if_logic.matches(1, 2));
    assert!(if_logic.matches(2, 1));
  }

  #[test]
  fn test_if_less() {
    let if_logic = IfLogic::Less;
    assert!(!if_logic.matches(1, 1));
    assert!(if_logic.matches(1, 2));
    assert!(!if_logic.matches(2, 1));
  }

  #[test]
  fn test_if_greater() {
    let if_logic = IfLogic::Greater;
    assert!(!if_logic.matches(1, 1));
    assert!(!if_logic.matches(1, 2));
    assert!(if_logic.matches(2, 1));
  }

  #[test]
  fn test_if_less_or_equal() {
    let if_logic = IfLogic::LessOrEqual;
    assert!(if_logic.matches(1, 1));
    assert!(if_logic.matches(1, 2));
    assert!(!if_logic.matches(2, 1));
  }

  #[test]
  fn test_if_greater_or_equal() {
    let if_logic = IfLogic::GreaterOrEqual;
    assert!(if_logic.matches(1, 1));
    assert!(!if_logic.matches(1, 2));
    assert!(if_logic.matches(2, 1));
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
