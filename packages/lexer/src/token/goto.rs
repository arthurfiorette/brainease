use lazy_regex::{regex, Captures, Lazy, Regex};

use crate::syntax::{GotoBy, GotoDirection, Instruction};

use super::Token;

#[derive(Debug, PartialEq, Eq)]
pub struct GotoToken;

impl Token for GotoToken {
  fn name(&self) -> &'static str {
    "goto"
  }

  fn regex(&self) -> &'static Lazy<Regex> {
    static REGEX: &Lazy<Regex> =
      regex!(r"^goto\s(left|right)(?:\sby\s(\d+|\*(\d+|@)))?\s*$");

    REGEX
  }

  fn read_instruction(
    &self,
    _: &[&str],
    captures: Captures,
    line_index: usize,
    _: usize,
  ) -> (usize, Option<Instruction>) {
    let dir: GotoDirection = captures[1].parse().unwrap();

    let by: Option<GotoBy> = captures
      .get(2)
      .and_then(|by_value| by_value.as_str().parse().ok());

    (line_index + 1, Some(Instruction::Goto { dir, by }))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::syntax::find_match;

  #[test]
  fn regex() {
    let regex = (GotoToken).regex();

    assert!(regex.is_match("goto right"));
    assert!(regex.is_match("goto right by 5"));
    assert!(regex.is_match("goto right by *5"));
    assert!(regex.is_match("goto left"));
    assert!(regex.is_match("goto left by 5"));
    assert!(regex.is_match("goto left by *5"));

    assert!(!regex.is_match("goto"));
    assert!(!regex.is_match("gotoleft"));
    assert!(!regex.is_match("goto_left"));
    assert!(!regex.is_match("gotoright"));
    assert!(!regex.is_match("goto_right"));

    assert!(!regex.is_match("goto down"));
    assert!(!regex.is_match("goto_down"));
    assert!(!regex.is_match("goto_down by 5"));
    assert!(!regex.is_match("goto_down by *5"));
    assert!(!regex.is_match("goto_down_by 5"));
    assert!(!regex.is_match("goto down by 5"));
    assert!(!regex.is_match("goto down by *5"));

    assert!(!regex.is_match("goto up"));
    assert!(!regex.is_match("goto_up"));
    assert!(!regex.is_match("goto_up by 5"));
    assert!(!regex.is_match("goto_up by *5"));
    assert!(!regex.is_match("goto_up_by 5"));
    assert!(!regex.is_match("goto up by 5"));
    assert!(!regex.is_match("goto up by *5"));
  }

  #[test]
  fn captures() {
    let text = "goto left";
    let (token, captures) = find_match(text).unwrap();

    assert_eq!(token, &GotoToken);
    assert_eq!(&captures[1], "left");

    let text = "goto right";
    let (token, captures) = find_match(text).unwrap();

    assert_eq!(token, &GotoToken);
    assert_eq!(&captures[1], "right");

    let text = "goto left by 2";
    let (token, captures) = find_match(text).unwrap();

    assert_eq!(token, &GotoToken);
    assert_eq!(&captures[1], "left");
    assert_eq!(&captures[2], "2");

    let text = "goto right by 2";
    let (token, captures) = find_match(text).unwrap();

    assert_eq!(token, &GotoToken);
    assert_eq!(&captures[1], "right");
    assert_eq!(&captures[2], "2");
  }
}
