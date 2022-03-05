use lazy_regex::{regex, Captures, Lazy, Regex};

use crate::syntax::Instruction;

use super::Token;

#[derive(Debug, PartialEq)]
pub struct WriteToken;

impl Token for WriteToken {
  fn name(&self) -> &'static str {
    "write"
  }

  fn regex(&self) -> &'static Lazy<Regex> {
    static REGEX: &Lazy<Regex> = regex!(r"^write\s\*(\d+|@)\s*$");
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

    (line_index + 1, Some(Instruction::Write(cell)))
  }
}
#[cfg(test)]
pub mod tests {
  use super::*;
  use crate::syntax::find_match;

  #[test]
  fn regex() {
    let regex = (WriteToken).regex();

    assert!(regex.is_match("write *5"));
    assert!(regex.is_match("write *@"));

    assert!(!regex.is_match("write *1   asdfgsdfh random text :)      "));
    assert!(!regex.is_match(" write *1"));
    assert!(!regex.is_match("write *a"));
    assert!(!regex.is_match(" write *a"));
    assert!(!regex.is_match("write 'a'"));
    assert!(!regex.is_match(" write *a"));
    assert!(!regex.is_match("write 'a' at *1"));
  }

  #[test]
  fn captures() {
    let text = "write *467";
    let (token, captures) = find_match(text).unwrap();

    assert_eq!(token, &WriteToken);
    assert_eq!(&captures[1], "467");
  }
}
