use lazy_regex::{Captures, Lazy, Regex};
use std::slice::Iter;

use super::{regex, token_parser, TokenParser};

#[derive(PartialEq, Debug, Clone)]
pub enum TokenKind {
  Increment,
  Decrement,
  Move,
  Swap,
  Save,
  Read,
  Write,
  Print,
  Loop,
  If,
  Goto,
}

impl TokenKind {
  pub fn regex(&self) -> &Lazy<Regex> {
    match *self {
      TokenKind::Increment => regex::INCREMENT,
      TokenKind::Decrement => regex::DECREMENT,
      TokenKind::Move => regex::MOVE,
      TokenKind::Swap => regex::SWAP,
      TokenKind::Save => regex::SAVE,
      TokenKind::Read => regex::READ,
      TokenKind::Write => regex::WRITE,
      TokenKind::Print => regex::PRINT,
      TokenKind::Loop => regex::LOOP,
      TokenKind::If => regex::IF,
      TokenKind::Goto => regex::GOTO,
    }
  }

  pub fn parser(&self) -> TokenParser {
    match *self {
      TokenKind::Increment => token_parser::INCREMENT,
      TokenKind::Decrement => token_parser::DECREMENT,
      TokenKind::Move => token_parser::MOVE,
      TokenKind::Swap => token_parser::SWAP,
      TokenKind::Save => token_parser::SAVE,
      TokenKind::Read => token_parser::READ,
      TokenKind::Write => token_parser::WRITE,
      TokenKind::Print => token_parser::PRINT,
      TokenKind::Loop => token_parser::LOOP,
      TokenKind::If => token_parser::IF,
      TokenKind::Goto => token_parser::GOTO,
    }
  }

  pub fn iter() -> Iter<'static, Self> {
    static TOKEN_KINDS: [TokenKind; 11] = [
      TokenKind::Increment,
      TokenKind::Decrement,
      TokenKind::Move,
      TokenKind::Swap,
      TokenKind::Save,
      TokenKind::Read,
      TokenKind::Write,
      TokenKind::Print,
      TokenKind::Loop,
      TokenKind::If,
      TokenKind::Goto
    ];

    TOKEN_KINDS.iter()
  }

  /// Returns a token that his regex matches the given string.
  /// Also returns the captures of the regex.
  pub fn find_match(text: &str) -> Option<(&TokenKind, Captures)> {
    let possible_tokens = TokenKind::iter();

    for token in possible_tokens {
      let regex = token.regex();
      let captures = regex.captures(text);

      // Regex match
      if let Some(captures) = captures {
        return Some((token, captures));
      }
    }

    None
  }
}

// Thanks copilot for these tests:)
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn all_returns_all_token_kinds() {
    assert_eq!(TokenKind::iter().len(), 11);
  }

  #[test]
  fn tests_detect_increment() {
    let text = "inc 123 in *467";
    let (token, captures) = TokenKind::find_match(text).unwrap();

    assert_eq!(token, &TokenKind::Increment);
    assert_eq!(&captures[1], "123");
    assert_eq!(&captures[2], "467");
  }

  #[test]
  fn tests_detect_decrement() {
    let text = "dec 123 in *467";
    let (token, captures) = TokenKind::find_match(text).unwrap();

    assert_eq!(token, &TokenKind::Decrement);
    assert_eq!(&captures[1], "123");
    assert_eq!(&captures[2], "467");
  }

  #[test]
  fn tests_detect_move() {
    let text = "move *123 to *467";
    let (token, captures) = TokenKind::find_match(text).unwrap();

    assert_eq!(token, &TokenKind::Move);
    assert_eq!(&captures[1], "123");
    assert_eq!(&captures[2], "467");
  }

  #[test]
  fn tests_detect_swap() {
    let text = "swap *123 and *467";
    let (token, captures) = TokenKind::find_match(text).unwrap();

    assert_eq!(token, &TokenKind::Swap);
    assert_eq!(&captures[1], "123");
    assert_eq!(&captures[2], "467");
  }

  #[test]
  fn tests_detect_save() {
    let text = "save 'a' at *467";
    let (token, captures) = TokenKind::find_match(text).unwrap();

    assert_eq!(token, &TokenKind::Save);
    assert_eq!(&captures[1], "a");
    assert_eq!(&captures[2], "467");
  }

  #[test]
  fn tests_detect_read() {
    let text = "read *467";
    let (token, captures) = TokenKind::find_match(text).unwrap();

    assert_eq!(token, &TokenKind::Read);
    assert_eq!(&captures[1], "467");
  }

  #[test]
  fn tests_detect_write() {
    let text = "write *467";
    let (token, captures) = TokenKind::find_match(text).unwrap();

    assert_eq!(token, &TokenKind::Write);
    assert_eq!(&captures[1], "467");
  }

  #[test]
  fn tests_detect_print() {
    let text = "print *467";
    let (token, captures) = TokenKind::find_match(text).unwrap();

    assert_eq!(token, &TokenKind::Print);
    assert_eq!(&captures[1], "467");
  }

  #[test]
  fn tests_detect_loop() {
    let text = "loop *467";
    let (token, captures) = TokenKind::find_match(text).unwrap();

    assert_eq!(token, &TokenKind::Loop);
    assert_eq!(&captures[1], "467");
  }

  #[test]
  fn tests_detect_if() {
    let text = "if *467 == 467";
    let (token, captures) = TokenKind::find_match(text).unwrap();

    assert_eq!(token, &TokenKind::If);
    assert_eq!(&captures[1], "467");
    assert_eq!(&captures[2], "==");
    assert_eq!(&captures[3], "467");
  }
}
