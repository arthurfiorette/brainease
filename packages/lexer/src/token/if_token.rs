use lazy_regex::{regex, Captures, Lazy, Regex};

use crate::{parser::parse_partial_file, syntax::Instruction};

use super::Token;

#[derive(Debug, PartialEq)]
pub struct IfToken;

impl Token for IfToken {
  fn name(&self) -> &'static str {
    "if"
  }
  fn regex(&self) -> &'static Lazy<Regex> {
    static REGEX: &Lazy<Regex> =
      regex!(r"^if\s\*(\d+|@)\s(==|!=|>|<|<=|>=)\s(\d{1,3}|\*(\d+|@))\s*$");
    REGEX
  }

  fn read_instruction(
    &self,
    file: &[&str],
    captures: Captures,
    line_index: usize,
    indentation: usize,
  ) -> (usize, Option<Instruction>) {
    let first_cell = captures[1].parse().unwrap();
    let logic = captures[2].parse().unwrap();

    let is_if_cell = file[line_index].matches('*').count() == 2;
    let value_or_cell = captures[if is_if_cell { 4 } else { 3 }].parse().unwrap();

    let (next_line, inner) =
      parse_partial_file(file, line_index + 1, indentation + 2, line_index);

    (
      next_line,
      Some(Instruction::If {
        cell: first_cell,
        logic,
        is_cell: is_if_cell,
        cell_or_value: value_or_cell,
        inner,
      }),
    )
  }
}

#[cfg(test)]
pub mod tests {
  use super::*;
  use crate::syntax::find_match;

  #[test]
  fn regex() {
    let regex = (IfToken).regex();

    for left in ["*@", "*1"] {
      for right in ["*@", "*53", "23"] {
        for operator in ["==", "!=", "<=", ">=", ">", "<"] {
          let expression = format!("if {} {} {}", left, operator, right);
          assert!(regex.is_match(&expression));

          // Exceeding character
          assert!(!regex.is_match(&format!("{} random extra characters", expression)));

          // left padding
          assert!(!regex.is_match(&format!(" {}", expression)));

          // right padding
          assert!(regex.is_match(&format!("{}         ", expression)));

          // without left space
          assert!(!regex.is_match(&format!("if {}{} {}", left, operator, right)));

          // without right space
          assert!(!regex.is_match(&format!("if {} {}{}", left, operator, right)));

          // without both space
          assert!(!regex.is_match(&format!("if {}{}{}", left, operator, right)));
        }

        for wrongly_operator in [
          "&", "|", "^", "%", "<<", ">>", "~", "!", "=", "+", "-", "*", "/", "\\", "--",
        ] {
          let expression = format!("if {} {} {}", left, wrongly_operator, right);

          assert!(!regex.is_match(&expression));
        }
      }
    }
  }

  #[test]
  fn captures() {
    for left in ["*@", "*1"] {
      for right in ["*@", "*53"] {
        for operator in ["==", "!=", "<=", ">=", ">", "<"] {
          let expression = format!("if {} {} {}", left, operator, right);

          let (token, captures) = find_match(&expression).unwrap();

          assert_eq!(token, &IfToken);
          assert_eq!(format!("*{}", &captures[1]), left);
          assert_eq!(&captures[2], operator);
          assert_eq!(format!("*{}", &captures[4]), right);
        }
      }
    }
  }
}
