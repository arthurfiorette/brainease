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

  /// Parses the given string into an if logic enum.
  pub fn from_str(s: &str) -> Option<Self> {
    match s {
      "==" => Some(IfLogic::Equal),
      "!=" => Some(IfLogic::NotEqual),
      "<" => Some(IfLogic::Less),
      ">" => Some(IfLogic::Greater),
      "<=" => Some(IfLogic::LessOrEqual),
      ">=" => Some(IfLogic::GreaterOrEqual),
      _ => None,
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
    assert_eq!(IfLogic::from_str("=="), Some(IfLogic::Equal));
    assert_eq!(IfLogic::from_str("!="), Some(IfLogic::NotEqual));
    assert_eq!(IfLogic::from_str("<"), Some(IfLogic::Less));
    assert_eq!(IfLogic::from_str(">"), Some(IfLogic::Greater));
    assert_eq!(IfLogic::from_str("<="), Some(IfLogic::LessOrEqual));
    assert_eq!(IfLogic::from_str(">="), Some(IfLogic::GreaterOrEqual));
    assert_eq!(IfLogic::from_str(""), None);
    assert_eq!(IfLogic::from_str("==="), None);
  }
}
