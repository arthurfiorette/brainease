use lazy_regex::{regex, Captures, Lazy, Regex};

use crate::syntax::Instruction;

use super::Token;

#[derive(Debug, PartialEq)]
pub struct PrintToken;

impl Token for PrintToken {
  fn name(&self) -> &'static str {
    "print"
  }
  fn regex(&self) -> &'static Lazy<Regex> {
    static REGEX: &Lazy<Regex> = regex!(r"^print\s\*(\d+|@)\s*$");
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

    (line_index + 1, Some(Instruction::Print(cell)))
  }
}

#[cfg(test)]
pub mod tests {
  use super::*;
  use crate::syntax::find_match;

  #[test]
  fn regex() {
    let regex = (PrintToken).regex();

    assert!(regex.is_match("print *1"));
    assert!(regex.is_match("print *@"));

    assert!(!regex.is_match("print *3   asdfgsdfh random text :)      "));
    assert!(!regex.is_match(" print *1"));
    assert!(!regex.is_match("print *a"));
    assert!(!regex.is_match(" print *a"));
    assert!(!regex.is_match("print *'a'"));
    assert!(!regex.is_match(" print *a"));
    assert!(!regex.is_match("print 'a' at *1"));
  }

  #[test]
  fn captures() {
    let text = "print *467";
    let (token, captures) = find_match(text).unwrap();

    assert_eq!(token, &PrintToken);
    assert_eq!(&captures[1], "467");
  }
}
