use lazy_regex::{regex, Captures, Lazy, Regex};

use crate::syntax::Instruction;

use super::Token;

#[derive(Debug, PartialEq)]
pub struct MoveToken;

impl Token for MoveToken {
  fn name(&self) -> &'static str {
    "move"
  }
  fn regex(&self) -> &'static Lazy<Regex> {
    static REGEX: &Lazy<Regex> = regex!(r"^move\s\*(\d+|@)\sto\s\*(\d+|@)\s*$");
    REGEX
  }

  fn read_instruction(
    &self,
    _: &[&str],
    captures: Captures,
    line_index: usize,
    _: usize,
  ) -> (usize, Option<Instruction>) {
    let current = captures[1].parse().unwrap();
    let next = captures[2].parse().unwrap();

    (line_index + 1, Some(Instruction::Move { current, next }))
  }
}

#[cfg(test)]
pub mod tests {
  use super::*;
  use crate::syntax::find_match;

  #[test]
  fn regex() {
    let regex = (MoveToken).regex();

    assert!(regex.is_match("move *1 to *1"));
    assert!(regex.is_match("move *1 to *@"));

    assert!(!regex.is_match("move *1 to *1   asdfgsdfh random text :)      "));
    assert!(!regex.is_match(" move *1 to *1"));
    assert!(!regex.is_match("move *a to *1"));
    assert!(!regex.is_match(" move *a to *1"));
    assert!(!regex.is_match("move *a to *a"));
    assert!(!regex.is_match(" move *a to *a"));
    assert!(!regex.is_match("move *1 to*1"));
  }

  #[test]
  fn captures() {
    let text = "move *123 to *467";
    let (token, captures) = find_match(text).unwrap();

    assert_eq!(token, &MoveToken);
    assert_eq!(&captures[1], "123");
    assert_eq!(&captures[2], "467");
  }
}
