use lazy_regex::{regex, Captures, Lazy, Regex};

use crate::{
  syntax::{CellOrChar, Instruction},
  util::interpret_escape_chars,
};

use super::Token;

#[derive(Debug, PartialEq, Eq)]
pub struct WriteToken;

impl Token for WriteToken {
  fn name(&self) -> &'static str {
    "write"
  }

  fn regex(&self) -> &'static Lazy<Regex> {
    static REGEX: &Lazy<Regex> = regex!(r"^write\s(?:(?:\*(\d+|@))|'(\\?.)')\s*$");

    REGEX
  }

  fn read_instruction(
    &self,
    _: &[&str],
    captures: Captures,
    line_index: usize,
    _: usize,
  ) -> (usize, Option<Instruction>) {
    let cell: CellOrChar = if let Some(c) = captures.get(1) {
      CellOrChar::Cell(c.as_str().parse().unwrap())
    } else {
      CellOrChar::Char(
        interpret_escape_chars(&captures[2])
          .parse::<char>()
          .unwrap(),
      )
    };

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
    assert!(regex.is_match("write 'A'"));

    assert!(!regex.is_match("write 'AB'"));
    assert!(!regex.is_match("write ''"));
    assert!(!regex.is_match("write *1   asdfgsdfh random text :)      "));
    assert!(!regex.is_match(" write *1"));
    assert!(!regex.is_match("write *a"));
    assert!(!regex.is_match(" write *a"));
    assert!(!regex.is_match(" write *a"));
    assert!(!regex.is_match("write 'a' at *1"));
  }

  #[test]
  fn captures() {
    let (token, captures) = find_match("write *467").unwrap();

    assert_eq!(token, &WriteToken);
    assert_eq!(&captures[1], "467");
    assert!(captures.get(2).is_none());

    let (token, captures) = find_match("write 'H'").unwrap();

    assert_eq!(token, &WriteToken);
    assert!(captures.get(1).is_none());
    assert_eq!(&captures[2], "H");

    let (token, captures) = find_match("write ' '").unwrap();

    assert_eq!(token, &WriteToken);
    assert!(captures.get(1).is_none());
    assert_eq!(&captures[2], " ");

    let (token, captures) = find_match("write '1'").unwrap();

    assert_eq!(token, &WriteToken);
    assert!(captures.get(1).is_none());
    assert_eq!(&captures[2], "1");

    let (token, captures) = find_match("write *@").unwrap();

    assert_eq!(token, &WriteToken);
    assert_eq!(&captures[1], "@");
    assert!(captures.get(2).is_none());
  }
}
