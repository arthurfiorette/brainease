use lazy_regex::{regex, Captures, Lazy, Regex};

use crate::{
  syntax::{CellOrChar, Instruction},
  util::interpret_escape_chars,
};

use super::Token;

#[derive(Debug, PartialEq)]
pub struct PrintToken;

impl Token for PrintToken {
  fn name(&self) -> &'static str {
    "print"
  }

  fn regex(&self) -> &'static Lazy<Regex> {
    static REGEX: &Lazy<Regex> = regex!(r"^print\s(?:(?:\*(\d+|@))|'(\\?.)')\s*$");
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
    assert!(regex.is_match("print 'a'"));

    assert!(!regex.is_match("print 'ABC'"));
    assert!(!regex.is_match("print ''"));
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
    let (token, captures) = find_match("print *467").unwrap();

    assert_eq!(token, &PrintToken);
    assert_eq!(&captures[1], "467");
    assert!(captures.get(2).is_none());

    let (token, captures) = find_match("print 'H'").unwrap();

    assert_eq!(token, &PrintToken);
    assert!(captures.get(1).is_none());
    assert_eq!(&captures[2], "H");

    let (token, captures) = find_match("print ' '").unwrap();

    assert_eq!(token, &PrintToken);
    assert!(captures.get(1).is_none());
    assert_eq!(&captures[2], " ");

    let (token, captures) = find_match("print '1'").unwrap();

    assert_eq!(token, &PrintToken);
    assert!(captures.get(1).is_none());
    assert_eq!(&captures[2], "1");

    let (token, captures) = find_match("print *@").unwrap();

    assert_eq!(token, &PrintToken);
    assert_eq!(&captures[1], "@");
    assert!(captures.get(2).is_none());
  }
}
