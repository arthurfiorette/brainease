use lazy_regex::{regex, Lazy, Regex};

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
  IfCell,
}

impl TokenKind {
  pub fn all() -> Vec<TokenKind> {
    vec![
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
      TokenKind::IfCell,
    ]
  }

  pub fn regex(&self) -> &Lazy<Regex> {
    static INCREMENT: &Lazy<Regex> = regex!(r"^inc\s(\d{1,3})\sin\s(\d+)(.*)");
    static DECREMENT: &Lazy<Regex> = regex!(r"^dec\s(\d{1,3})\sin\s(\d+)(.*)");
    static MOVE: &Lazy<Regex> = regex!(r"^move\s(\d+)\sto\s(\d+)(.*)");
    static SWAP: &Lazy<Regex> = regex!(r"^swap\s(\d+)\sand\s(\d+)(.*)");
    static SAVE: &Lazy<Regex> = regex!(r"^save\s'(\w)'\sat\s(\d+)(.*)");
    static READ: &Lazy<Regex> = regex!(r"^read\s(\d+)(.*)");
    static WRITE: &Lazy<Regex> = regex!(r"^write\s(\d+)(.*)");
    static PRINT: &Lazy<Regex> = regex!(r"^print\s(\d+)(.*)");
    static LOOP: &Lazy<Regex> = regex!(r"^loop\s(\d+)(.*)");
    static IF: &Lazy<Regex> = regex!(r"^if\s(\d+)\s(==|!=|>|<|<=|>=)\s(\d+)(.*)");
    static IFCELL: &Lazy<Regex> =
      regex!(r"^if_cell\s(\d+)\s(==|!=|>|<|<=|>=)\s(\d+)(.*)");

    match &self {
      TokenKind::Increment => INCREMENT,
      TokenKind::Decrement => DECREMENT,
      TokenKind::Move => MOVE,
      TokenKind::Swap => SWAP,
      TokenKind::Save => SAVE,
      TokenKind::Read => READ,
      TokenKind::Write => WRITE,
      TokenKind::Print => PRINT,
      TokenKind::Loop => LOOP,
      TokenKind::If => IF,
      TokenKind::IfCell => IFCELL,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn all_returns_all_token_kinds() {
    let all = TokenKind::all();
    assert_eq!(all.len(), 11);
  }

  #[test]
  fn tests_inc_regex() {
    let regex = TokenKind::Increment.regex();

    assert!(regex.is_match("inc 1 in 1"));
    assert!(regex.is_match("inc 1 in 1   asdfgsdfh random text :)      "));

    assert!(!regex.is_match(" inc 1 in 1"));
    assert!(!regex.is_match("inc a in 1"));
    assert!(!regex.is_match(" inc a in 1"));
    assert!(!regex.is_match("inc a in a"));
    assert!(!regex.is_match(" inc a in a"));
    assert!(!regex.is_match("inc 1 in1"));
  }

  #[test]
  fn tests_dec_regex() {
    let regex = TokenKind::Decrement.regex();

    assert!(regex.is_match("dec 1 in 1"));
    assert!(regex.is_match("dec 1 in 1   asdfgsdfh random text :)      "));

    assert!(!regex.is_match(" dec 1 in 1"));
    assert!(!regex.is_match("dec a in 1"));
    assert!(!regex.is_match(" dec a in 1"));
    assert!(!regex.is_match("dec a in a"));
    assert!(!regex.is_match(" dec a in a"));
    assert!(!regex.is_match("dec 1 in1"));
  }

  #[test]
  fn tests_move_regex() {
    let regex = TokenKind::Move.regex();

    assert!(regex.is_match("move 1 to 1"));
    assert!(regex.is_match("move 1 to 1   asdfgsdfh random text :)      "));

    assert!(!regex.is_match(" move 1 to 1"));
    assert!(!regex.is_match("move a to 1"));
    assert!(!regex.is_match(" move a to 1"));
    assert!(!regex.is_match("move a to a"));
    assert!(!regex.is_match(" move a to a"));
    assert!(!regex.is_match("move 1 to1"));
  }

  #[test]
  fn tests_swap_regex() {
    let regex = TokenKind::Swap.regex();

    assert!(regex.is_match("swap 1 and 1"));
    assert!(regex.is_match("swap 1 and 1   asdfgsdfh random text :)      "));

    assert!(!regex.is_match(" swap 1 and 1"));
    assert!(!regex.is_match("swap a and 1"));
    assert!(!regex.is_match(" swap a and 1"));
    assert!(!regex.is_match("swap a and a"));
    assert!(!regex.is_match(" swap a and a"));
    assert!(!regex.is_match("swap 1 and1"));
  }

  #[test]
  fn tests_save_regex() {
    let regex = TokenKind::Save.regex();

    assert!(regex.is_match("save 'a' at 9"));
    assert!(regex.is_match("save 'a' at 1   asdfgsdfh random text :)      "));

    assert!(!regex.is_match(" save 'a' at 1"));
    assert!(!regex.is_match("save a at 1"));
    assert!(!regex.is_match(" save a at 9"));
    assert!(!regex.is_match("save a at a"));
    assert!(!regex.is_match(" save a at a"));
    assert!(!regex.is_match("save 1 at1"));
  }

  #[test]
  fn tests_read_regex() {
    let regex = TokenKind::Read.regex();

    assert!(regex.is_match("read 1"));
    assert!(regex.is_match("read 7   asdfgsdfh random text :)      "));

    assert!(!regex.is_match(" read 1"));
    assert!(!regex.is_match("read a"));
    assert!(!regex.is_match(" read a"));
    assert!(!regex.is_match("read 'a'"));
    assert!(!regex.is_match(" read a"));
    assert!(!regex.is_match("read 'a' at 1"));
  }

  #[test]
  fn tests_write_regex() {
    let regex = TokenKind::Write.regex();

    assert!(regex.is_match("write 5"));
    assert!(regex.is_match("write 1   asdfgsdfh random text :)      "));

    assert!(!regex.is_match(" write 1"));
    assert!(!regex.is_match("write a"));
    assert!(!regex.is_match(" write a"));
    assert!(!regex.is_match("write 'a'"));
    assert!(!regex.is_match(" write a"));
    assert!(!regex.is_match("write 'a' at 1"));
  }

  #[test]
  fn tests_print_regex() {
    let regex = TokenKind::Print.regex();

    assert!(regex.is_match("print 1"));
    assert!(regex.is_match("print 3   asdfgsdfh random text :)      "));

    assert!(!regex.is_match(" print 1"));
    assert!(!regex.is_match("print a"));
    assert!(!regex.is_match(" print a"));
    assert!(!regex.is_match("print 'a'"));
    assert!(!regex.is_match(" print a"));
    assert!(!regex.is_match("print 'a' at 1"));
  }

  #[test]
  fn tests_loop_regex() {
    let regex = TokenKind::Loop.regex();

    assert!(regex.is_match("loop 1"));
    assert!(regex.is_match("loop 2  asdfgsdfh random text :)      "));

    assert!(!regex.is_match(" loop"));
    assert!(!regex.is_match("loop a"));
    assert!(!regex.is_match(" loop a"));
    assert!(!regex.is_match("loop 'a'"));
    assert!(!regex.is_match(" loop a"));
    assert!(!regex.is_match("loop 'a' at 1"));
  }

  #[test]
  fn tests_if_regex() {
    let regex = TokenKind::If.regex();

    assert!(regex.is_match("if 1 == 2"));
    assert!(regex.is_match("if 1 != 2"));
    assert!(regex.is_match("if 1 <= 2"));
    assert!(regex.is_match("if 1 >= 2"));
    assert!(regex.is_match("if 1 < 2"));
    assert!(regex.is_match("if 1 > 2"));

    assert!(regex.is_match("if 1 == 2 dafhsdghsgfh"));
    assert!(regex.is_match("if 1 != 2 sfghdfgh sdfg"));
    assert!(regex.is_match("if 1 <= 2 dsfg sdfg"));
    assert!(regex.is_match("if 1 >= 2 sdfg sdfg s"));
    assert!(regex.is_match("if 1 < 2 sdf asdf"));
    assert!(regex.is_match("if 1 > 2 asdf asdf"));

    assert!(!regex.is_match(" if 1 == 2"));
    assert!(!regex.is_match(" if 1 != 2"));
    assert!(!regex.is_match(" if 1 <= 2"));
    assert!(!regex.is_match(" if 1 >= 2"));
    assert!(!regex.is_match(" if 1 < 2"));
    assert!(!regex.is_match(" if 1 > 2"));

    assert!(!regex.is_match("if 1== 2"));
    assert!(!regex.is_match("if 1!= 2"));
    assert!(!regex.is_match("if 1<= 2"));
    assert!(!regex.is_match("if 1>= 2"));
    assert!(!regex.is_match("if 1< 2"));
    assert!(!regex.is_match("if 1> 2"));

    assert!(!regex.is_match("if 1==2"));
    assert!(!regex.is_match("if 1!=2"));
    assert!(!regex.is_match("if 1<=2"));
    assert!(!regex.is_match("if 1>=2"));
    assert!(!regex.is_match("if 1<2"));
    assert!(!regex.is_match("if 1>2"));

    assert!(!regex.is_match("if 1 & 2"));
    assert!(!regex.is_match("if 1 | 2"));
    assert!(!regex.is_match("if 1 ^ 2"));
    assert!(!regex.is_match("if 1 << 2"));
    assert!(!regex.is_match("if 1 >> 2"));
    assert!(!regex.is_match("if 1 >>> 2"));
    assert!(!regex.is_match("if 1 + 2"));
    assert!(!regex.is_match("if 1 - 2"));
    assert!(!regex.is_match("if 1 * 2"));
    assert!(!regex.is_match("if 1 / 2"));
    assert!(!regex.is_match("if 1 % 2"));
    assert!(!regex.is_match("if 1 ** 2"));
    assert!(!regex.is_match("if 1 ++ 2"));
    assert!(!regex.is_match("if 1 -- 2"));
    assert!(!regex.is_match("if 1 += 2"));
    assert!(!regex.is_match("if 1 -= 2"));
  }

  #[test]
  fn tests_if_cell_regex() {
    let regex = TokenKind::IfCell.regex();

    assert!(regex.is_match("if_cell 1 == 2"));
    assert!(regex.is_match("if_cell 1 != 2"));
    assert!(regex.is_match("if_cell 1 <= 2"));
    assert!(regex.is_match("if_cell 1 >= 2"));
    assert!(regex.is_match("if_cell 1 < 2"));
    assert!(regex.is_match("if_cell 1 > 2"));

    assert!(regex.is_match("if_cell 1 == 2 dafhsdghsgfh"));
    assert!(regex.is_match("if_cell 1 != 2 sfghdfgh sdfg"));
    assert!(regex.is_match("if_cell 1 <= 2 dsfg sdfg"));
    assert!(regex.is_match("if_cell 1 >= 2 sdfg sdfg s"));
    assert!(regex.is_match("if_cell 1 < 2 sdf asdf"));
    assert!(regex.is_match("if_cell 1 > 2 asdf asdf"));

    assert!(!regex.is_match(" if_cell 1 == 2"));
    assert!(!regex.is_match(" if_cell 1 != 2"));
    assert!(!regex.is_match(" if_cell 1 <= 2"));
    assert!(!regex.is_match(" if_cell 1 >= 2"));
    assert!(!regex.is_match(" if_cell 1 < 2"));
    assert!(!regex.is_match(" if_cell 1 > 2"));

    assert!(!regex.is_match("if_cell 1== 2"));
    assert!(!regex.is_match("if_cell 1!= 2"));
    assert!(!regex.is_match("if_cell 1<= 2"));
    assert!(!regex.is_match("if_cell 1>= 2"));
    assert!(!regex.is_match("if_cell 1< 2"));
    assert!(!regex.is_match("if_cell 1> 2"));

    assert!(!regex.is_match("if_cell 1==2"));
    assert!(!regex.is_match("if_cell 1!=2"));
    assert!(!regex.is_match("if_cell 1<=2"));
    assert!(!regex.is_match("if_cell 1>=2"));
    assert!(!regex.is_match("if_cell 1<2"));
    assert!(!regex.is_match("if_cell 1>2"));

    assert!(!regex.is_match("if_cell 1 & 2"));
    assert!(!regex.is_match("if_cell 1 | 2"));
    assert!(!regex.is_match("if_cell 1 ^ 2"));
    assert!(!regex.is_match("if_cell 1 << 2"));
    assert!(!regex.is_match("if_cell 1 >> 2"));
    assert!(!regex.is_match("if_cell 1 >>> 2"));
    assert!(!regex.is_match("if_cell 1 + 2"));
    assert!(!regex.is_match("if_cell 1 - 2"));
    assert!(!regex.is_match("if_cell 1 * 2"));
    assert!(!regex.is_match("if_cell 1 / 2"));
    assert!(!regex.is_match("if_cell 1 % 2"));
    assert!(!regex.is_match("if_cell 1 ** 2"));
    assert!(!regex.is_match("if_cell 1 ++ 2"));
    assert!(!regex.is_match("if_cell 1 -- 2"));
    assert!(!regex.is_match("if_cell 1 += 2"));
    assert!(!regex.is_match("if_cell 1 -= 2"));
  }
}
