use crate::logger;

/// Checks if the given line is empty or is a comment (starts with `#`).
pub fn is_empty_line(line: &str) -> bool {
  line.starts_with('#') || line.chars().all(char::is_whitespace)
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
  fn tests_empty_line() {
    assert!(is_empty_line(""));
    assert!(is_empty_line("        "));
    assert!(is_empty_line("# comment"));
    assert!(is_empty_line("         　       　    "));

    assert!(!is_empty_line("a        "));
    assert!(!is_empty_line("           a"));
    assert!(!is_empty_line("   aasd a"));
    assert!(!is_empty_line("   　  a"));
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
}
