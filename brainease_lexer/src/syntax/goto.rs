use std::{num::ParseIntError, str::FromStr};
use super::CellPosition;

#[derive(Debug, Clone, PartialEq)]
pub enum GotoDirection {
  Left,
  Right,
}

impl FromStr for GotoDirection {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "left" => Ok(GotoDirection::Left),
      "right" => Ok(GotoDirection::Right),
      _ => Err(()),
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GotoBy {
  ByCell(CellPosition),
  ByValue(CellPosition),
}

impl FromStr for GotoBy {
  type Err = ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if s.contains('*') {
      Ok(GotoBy::ByCell(s[1..].parse()?))
    } else {
      Ok(GotoBy::ByValue(s.parse()?))
    }
  }
}

#[cfg(test)]
pub mod tests {
  use super::*;

  #[test]
  pub fn test_goto_by_from_str_by_cell() {
    assert_eq!(GotoBy::from_str("*2"), Ok(GotoBy::ByCell(2)));
    assert_eq!(GotoBy::from_str("*22312"), Ok(GotoBy::ByCell(22312)));
    assert_eq!(GotoBy::from_str("*1234"), Ok(GotoBy::ByCell(1234)));

    assert!(GotoBy::from_str("*").is_err());
    assert!(GotoBy::from_str("*a").is_err());
    assert!(GotoBy::from_str("*-1").is_err());
    assert!(GotoBy::from_str("*1a").is_err());
  }

  #[test]
  pub fn test_goto_by_from_str_by_value() {
    assert_eq!(GotoBy::from_str("1"), Ok(GotoBy::ByValue(1)));
    assert_eq!(GotoBy::from_str("22312"), Ok(GotoBy::ByValue(22312)));
    assert_eq!(GotoBy::from_str("1234"), Ok(GotoBy::ByValue(1234)));

    assert!(GotoBy::from_str("").is_err());
    assert!(GotoBy::from_str("a").is_err());
    assert!(GotoBy::from_str("-1").is_err());
    assert!(GotoBy::from_str("1a").is_err());
  }
}
