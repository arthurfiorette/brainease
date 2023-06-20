use lazy_regex::{regex, Captures, Lazy, Regex};

use crate::syntax::Instruction;

use super::Token;

#[derive(Debug, PartialEq, Eq)]
pub struct SwapToken;

impl Token for SwapToken {
  fn name(&self) -> &'static str {
    "swap"
  }
  fn regex(&self) -> &'static Lazy<Regex> {
    static REGEX: &Lazy<Regex> = regex!(r"^swap\s\*(\d+|@)\sand\s\*(\d+|@)\s*$");

    REGEX
  }

  fn read_instruction(
    &self,
    _: &[&str],
    captures: Captures,
    line_index: usize,
    _: usize,
  ) -> (usize, Option<Instruction>) {
    let from = captures[1].parse().unwrap();
    let with = captures[2].parse().unwrap();

    (line_index + 1, Some(Instruction::Swap { from, with }))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::syntax::find_match;

  #[test]
  fn regex() {
    let regex = (SwapToken).regex();

    assert!(regex.is_match("swap *1 and *1"));
    assert!(regex.is_match("swap *1 and *@"));

    assert!(!regex.is_match("swap *1 and *1   asdfgsdfh random text :)      "));
    assert!(!regex.is_match(" swap *1 and *1"));
    assert!(!regex.is_match("swap *a and *1"));
    assert!(!regex.is_match(" swap *a and *1"));
    assert!(!regex.is_match("swap *a and *a"));
    assert!(!regex.is_match(" swap *a and *a"));
    assert!(!regex.is_match("swap *1 and*1"));
  }

  #[test]
  fn captures() {
    let text = "swap *123 and *467";
    let (token, captures) = find_match(text).unwrap();

    assert_eq!(token, &SwapToken);
    assert_eq!(&captures[1], "123");
    assert_eq!(&captures[2], "467");
  }
}
