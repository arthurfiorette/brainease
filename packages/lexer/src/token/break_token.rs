use lazy_regex::{regex, Captures, Lazy, Regex};

use crate::syntax::Instruction;

use super::Token;

#[derive(Debug, PartialEq)]
pub struct BreakToken;

impl Token for BreakToken {
  fn name(&self) -> &'static str {
    "break"
  }

  fn regex(&self) -> &'static Lazy<Regex> {
    static REGEX: &Lazy<Regex> = regex!(r"^(exit|break|continue|break\sall)?\s*$");

    REGEX
  }

  fn read_instruction(
    &self,
    _: &[&str],
    captures: Captures,
    line_index: usize,
    _: usize,
  ) -> (usize, Option<Instruction>) {
    let break_type = captures[1].parse().unwrap();

    (line_index + 1, Some(Instruction::Break(break_type)))
  }
}

#[cfg(test)]
pub mod tests {
  use super::*;

  #[test]
  fn regex() {
    let regex = (BreakToken).regex();

    let tokens = ["exit", "break", "continue", "break all"];

    for token in tokens {
      assert!(regex.is_match(token));
      assert!(regex.is_match(&format!("{}   ", token)));

      assert!(!regex.is_match(&format!("{} random text", token)));
      assert!(!regex.is_match(&format!(" {}", token)));
    }
  }
}
