use super::CellPosition;
use std::{num::ParseIntError, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
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

impl ToString for GotoDirection {
    fn to_string(&self) -> String {
        match self {
            GotoDirection::Left => "left".to_string(),
            GotoDirection::Right => "right".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GotoBy {
    Cell(CellPosition),
    Pointer,
    /// Value means a number of cells to jump.
    Number(usize),
}

impl FromStr for GotoBy {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if let Some(num) = s.strip_prefix('*') {
            GotoBy::Cell(num.parse()?)
        } else if s.eq("@") {
            GotoBy::Pointer
        } else {
            GotoBy::Number(s.parse()?)
        })
    }
}

impl ToString for GotoBy {
    fn to_string(&self) -> String {
        match self {
            GotoBy::Cell(val) => format!("*{}", val),
            GotoBy::Pointer => "*@".to_string(),
            GotoBy::Number(val) => val.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_goto_by_from_str_cell() {
        assert_eq!(GotoBy::from_str("*2"), Ok(GotoBy::Cell(2)));
        assert_eq!(GotoBy::from_str("*22312"), Ok(GotoBy::Cell(22312)));
        assert_eq!(GotoBy::from_str("*1234"), Ok(GotoBy::Cell(1234)));
    }

    #[test]
    fn test_goto_by_from_str_number() {
        assert_eq!(GotoBy::from_str("1"), Ok(GotoBy::Number(1)));
        assert_eq!(GotoBy::from_str("22312"), Ok(GotoBy::Number(22312)));
        assert_eq!(GotoBy::from_str("1234"), Ok(GotoBy::Number(1234)));
    }

    #[test]
    fn test_goto_by_from_str_pointer() {
        assert_eq!(GotoBy::from_str("@"), Ok(GotoBy::Pointer));
    }

    #[test]
    fn test_goto_by_from_str_errors() {
        for val in ["", "a", "-1", "1a", "*", "*a", "*-1", "*1a"] {
            assert!(GotoBy::from_str(val).is_err());
        }
    }
}
