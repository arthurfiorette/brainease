use std::{num::ParseIntError, str::FromStr};

use super::CellPosition;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellOrPointer {
  Cell(CellPosition),
  Pointer,
}

impl FromStr for CellOrPointer {
  type Err = ParseIntError;

  fn from_str(str: &str) -> Result<Self, Self::Err> {
    if str.eq("@") {
      Ok(CellOrPointer::Pointer)
    } else {
      Ok(CellOrPointer::Cell(str.parse()?))
    }
  }
}

impl CellOrPointer {
  pub fn or(&self, val: CellPosition) -> CellPosition {
    match *self {
      CellOrPointer::Cell(cell) => cell,
      CellOrPointer::Pointer => val,
    }
  }

  ///# Panics
  ///
  /// Panics if the value is an [`CellOrPointer::Pointer`].
  pub fn unwrap(&self) -> CellPosition {
    match *self {
      CellOrPointer::Cell(cell) => cell,
      CellOrPointer::Pointer => panic!(),
    }
  }
}
