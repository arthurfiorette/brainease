use crate::logger;

/// Checks if the given line is empty or is a comment (starts with `#`).
pub fn is_empty_line(line: &str) -> bool {
  for char in line.chars() {
    if char == '#' {
      return true;
    }

    if !char.is_whitespace() {
      return false;
    }
  }

  true
}

/// Returns true if the given line has the exact given number of spaces.
pub fn match_indentation(spaces: usize, line: &str) -> bool {
  let mut chars = line.chars();

  for _ in 0..spaces {
    let char = chars.next();
    if char.is_none() || !char.unwrap().is_whitespace() {
      return false;
    }
  }

  // Ensures that indentation has ended.
  let char = chars.next();
  char.is_none() || !char.unwrap().is_whitespace()
}

pub fn log_extra_chars(line_index: &usize, str: &str) {
  if !str.is_empty() {
    logger::extra_characters(line_index, str);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_empty_line_comment() {
    for i in 1..50 {
      assert!(is_empty_line(&format!("{}# comment", " ".repeat(i))));
    }
  }

  #[test]
  fn test_empty_line_random_chars() {
    let space_chars = [
      ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '　', ' ', ' ', ' ', ' ', ' ',
      ' ', ' ', '　', ' ', ' ', ' ', ' ', '　', ' ',
    ];

    for char in space_chars {
      for char2 in space_chars {
        for i in 1..20 {
          let space = format!("{}{}", char, char2).repeat(i);

          assert!(is_empty_line(&space));
          assert!(is_empty_line(&format!("{} # a comment", space)));
          assert!(!is_empty_line(&format!("{} not a comment", space)));
        }
      }
    }
  }

  #[test]
  fn test_not_empty_line() {
    for i in 1..50 {
      assert!(!is_empty_line(&format!("{} not a comment", " ".repeat(i))));
    }
  }

  #[test]
  fn indentation_with_empty_lines() {
    for i in 1..10 {
      assert!(!match_indentation(i, ""));
    }
  }

  #[test]
  fn exact_indentation() {
    for i in 0..50 {
      assert!(match_indentation(
        i,
        &format!("{}{}", " ".repeat(i), "exact")
      ));
    }
  }

  #[test]
  fn indentation_more() {
    for i in 0..50 {
      assert!(!match_indentation(
        i,
        &format!("{}{}", " ".repeat(i), " one space more")
      ));
    }
  }

  #[test]
  fn indentation_less() {
    for i in 1..50 {
      assert!(!match_indentation(
        i,
        &format!("{}{}", " ".repeat(i - 1), "one space less")
      ));
    }
  }

  #[test]
  fn oversized_indentation() {
    for i in 2..50 {
      assert!(!match_indentation(
        2,
        &format!("{}{}", " ".repeat(i + 1), "end")
      ));
    }
  }

  #[test]
  fn comments_after_code() {
    assert!(is_empty_line("    # comment instruction"));
    assert!(is_empty_line("# comment instruction"));
    assert!(is_empty_line("######## a lot of #'s"));
    assert!(!is_empty_line("instruction # comment"));
    assert!(!is_empty_line("instruction # comment another instruction"));
  }
}
