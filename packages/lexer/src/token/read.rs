use lazy_regex::{regex, Captures, Lazy, Regex};

use crate::syntax::Instruction;

use super::Token;

#[derive(Debug, PartialEq)]
pub struct ReadToken;

impl Token for ReadToken {
  fn name(&self) -> &'static str {
    "read"
  }
  fn regex(&self) -> &'static Lazy<Regex> {
    static REGEX: &Lazy<Regex> = regex!(r"^read\s\*(\d+|@)\s*$");
    REGEX
  }

  fn read_instruction(
    &self,
    _: &[&str],
    captures: Captures,
    line_index: usize,
    _: usize,
  ) -> (usize, Option<Instruction>) {
    let cell = captures[1].parse().unwrap();

    (line_index + 1, Some(Instruction::Read(cell)))
  }
}

#[cfg(test)]
pub mod tests {
  use super::*;
  use crate::syntax::find_match;

  #[test]
  fn regex() {
    let regex = (ReadToken).regex();

    assert!(regex.is_match("read *1"));
    assert!(regex.is_match("read *@"));

    assert!(!regex.is_match("read *7   asdfgsdfh random text :)      "));
    assert!(!regex.is_match(" read *1"));
    assert!(!regex.is_match("read *a"));
    assert!(!regex.is_match(" read *a"));
    assert!(!regex.is_match("read *'a'"));
    assert!(!regex.is_match(" read *a"));
    assert!(!regex.is_match("read 'a' at *1"));
  }

  #[test]
  fn captures() {
    let text = "read *467";
    let (token, captures) = find_match(text).unwrap();

    assert_eq!(token, &ReadToken);
    assert_eq!(&captures[1], "467");
  }
}
