use std::str::FromStr;

use super::{CellOrPointer, CellPosition};

#[derive(Debug, Clone, PartialEq)]
pub enum CellOrChar {
  Cell(CellOrPointer),
  Char(char),
}

impl CellOrChar {
  pub fn pointer() -> CellOrChar {
    CellOrChar::Cell(CellOrPointer::Pointer)
  }

  pub fn cell(cell: CellPosition) -> CellOrChar {
    CellOrChar::Cell(CellOrPointer::Cell(cell))
  }
}

impl FromStr for CellOrChar {
  type Err = ();

  /// Parses the given string into an cell or char enum.
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if let Some(char) = s.chars().next() {
      if !char.is_numeric() && char != '@' {
        return Ok(CellOrChar::Char(char));
      }

      if let Ok(cell_or_pointer) = CellOrPointer::from_str(s) {
        return Ok(CellOrChar::Cell(cell_or_pointer));
      }
    }

    Err(())
  }
}

impl ToString for CellOrChar {
  fn to_string(&self) -> String {
    match self {
      CellOrChar::Cell(val) => val.to_string(),
      CellOrChar::Char(val) => format!("'{}'", val),
    }
  }
}

#[cfg(test)]
pub mod tests {
  use super::*;
  #[test]

  pub fn test() {
    assert_eq!(
      CellOrChar::from_str("2"),
      Ok(CellOrChar::Cell(CellOrPointer::Cell(2)))
    );

    assert_eq!(CellOrChar::from_str("@"), Ok(CellOrChar::pointer()));
    assert_eq!(CellOrChar::from_str("A"), Ok(CellOrChar::Char('A')));
    assert_eq!(CellOrChar::from_str(" "), Ok(CellOrChar::Char(' ')));
  }
}
