use super::{CellOrPointer, CellPosition};

/// A simple enum that may contains a Char value or a Cell indicator.
#[derive(Debug, Clone, PartialEq, Eq)]
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

impl ToString for CellOrChar {
  fn to_string(&self) -> String {
    match self {
      CellOrChar::Cell(val) => val.to_string(),
      CellOrChar::Char(val) => format!("'{}'", val),
    }
  }
}
