use lazy_regex::{regex, Captures, Lazy, Regex};

use crate::syntax::Instruction;

use super::Token;

#[derive(Debug, PartialEq)]
pub struct BreakToken;

impl Token for BreakToken {
  fn name(&self) -> &'static str {
    "read"
  }
  fn regex(&self) -> &'static Lazy<Regex> {
    static REGEX: &Lazy<Regex> = regex!(r"^(return|exit)?\s*$");

    REGEX
  }

  fn read_instruction(
    &self,
    _: &[&str],
    captures: Captures,
    line_index: usize,
    _: usize,
  ) -> (usize, Option<Instruction>) {
    let is_exit = &captures[1] == "exit";

    (line_index + 1, Some(Instruction::Break(is_exit)))
  }
}

#[cfg(test)]
pub mod tests {
  use super::*;

  #[test]
  fn regex() {
    let regex = (BreakToken).regex();

    assert!(regex.is_match("return"));
    assert!(regex.is_match("exit"));
    assert!(regex.is_match("return  "));
    assert!(regex.is_match("exit  "));

    assert!(!regex.is_match("return  asdfgsdfh random text :)      "));
    assert!(!regex.is_match("exit   asdfgsdfh random text :)      "));
    assert!(!regex.is_match(" exit"));
    assert!(!regex.is_match(" return"));
  }
}
