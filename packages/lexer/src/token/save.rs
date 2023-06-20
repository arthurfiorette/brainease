use lazy_regex::{regex, Captures, Lazy, Regex};

use crate::{logger, syntax::Instruction, util::interpret_escape_chars};

use super::Token;

#[derive(Debug, PartialEq, Eq)]
pub struct SaveToken;

impl Token for SaveToken {
  fn name(&self) -> &'static str {
    "save"
  }
  fn regex(&self) -> &'static Lazy<Regex> {
    static REGEX: &Lazy<Regex> = regex!(r"^save\s'(.)'\sat\s\*(\d+|@)\s*$");

    REGEX
  }

  fn read_instruction(
    &self,
    _: &[&str],
    captures: Captures,
    line_index: usize,
    _: usize,
  ) -> (usize, Option<Instruction>) {
    let char = interpret_escape_chars(&captures[1])
      .parse::<char>()
      .unwrap();

    let cell = captures[2].parse().unwrap();

    if char as usize > u8::MAX as usize {
      logger::invalid_char(&line_index, &char);
      panic!()
    }

    (
      line_index + 1,
      Some(Instruction::Save {
        cell,
        value: char as u8,
      }),
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::syntax::find_match;

  #[test]
  fn regex() {
    let regex = (SaveToken).regex();

    assert!(regex.is_match("save 'a' at *9"));
    assert!(regex.is_match("save 'a' at *@"));

    assert!(!regex.is_match("save 'a' at *1   asdfgsdfh random text :)      "));
    assert!(!regex.is_match(" save 'a' at *1"));
    assert!(!regex.is_match("save a at *1"));
    assert!(!regex.is_match(" save a at *9"));
    assert!(!regex.is_match("save a at *a"));
    assert!(!regex.is_match(" save a at *a"));
    assert!(!regex.is_match("save 1 at*1"));
  }

  #[test]
  fn captures() {
    let text = "save 'a' at *467";
    let (token, captures) = find_match(text).unwrap();

    assert_eq!(token, &SaveToken);
    assert_eq!(&captures[1], "a");
    assert_eq!(&captures[2], "467");
  }
}
