/// Checks if the given line is empty or is a comment (starts with `#`).
pub fn is_empty_line(line: &str) -> bool {
  line.starts_with('#') || line.chars().all(char::is_whitespace)
}

/// Returns true if the given line has the exact given number of spaces.
pub fn match_indentation(spaces: usize, line: &str) -> bool {
  let mut chars = line.chars();

  if spaces >= line.len() {
    return chars.all(char::is_whitespace);
  }

  for _ in 0..spaces {
    if !chars.next().unwrap().is_whitespace() {
      return false;
    }
  }

  // Ensures that indentation has ended.
  !chars.next().unwrap().is_whitespace()
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
  fn tests_match_indentation() {
    assert!(match_indentation(0, ""));
    assert!(match_indentation(0, "a"));
    assert!(match_indentation(1, " "));
    assert!(match_indentation(1, " a"));
    assert!(match_indentation(2, "  "));
    assert!(match_indentation(2, "  a"));

    // tabs
    assert!(match_indentation(2, "  a"));
    assert!(match_indentation(4, "    a"));
    assert!(match_indentation(6, "      a"));

    // Exact indentation
    assert!(!match_indentation(0, "      asd asd asd"));
    assert!(!match_indentation(1, "      asd asd asd"));
    assert!(!match_indentation(2, "      asd asd asd"));
    assert!(!match_indentation(3, "      asd asd asd"));
    assert!(!match_indentation(4, "      asd asd asd"));
    assert!(!match_indentation(5, "      asd asd asd"));
    assert!(match_indentation(6, "      asd asd asd"));
  }
}
