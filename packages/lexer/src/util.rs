/// Creates a function that returns a new string with everything before a # character.
pub fn strip_comments(line: &str) -> &str {
    line.find('#')
        .map_or(line, |index| &line[..index])
        .trim_end()
}

/// Returns true if the given line has the exact given number of spaces.
pub fn match_indentation(spaces: usize, line: &str) -> bool {
    let mut chars = line.chars();

    for _ in 0..spaces {
        if chars.next().map_or(true, |c| !c.is_whitespace()) {
            return false;
        }
    }

    chars.next().map_or(true, |c| !c.is_whitespace())
}

/// Replaces all characters that should be escaped with their escaped version.
///
/// <https://doc.rust-lang.org/reference/tokens.html#ascii-escapes>
pub fn interpret_escape_chars(text: &str) -> String {
    text.replace("\\n", "\n")
        .replace("\\t", "\t")
        .replace("\\r", "\r")
        .replace("\\\\", "\\")
        .replace("\\0", "\0")
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn indentation_samples() {
        for i in 1..10 {
            assert!(match_indentation(i, &" ".repeat(i)));
            assert!(match_indentation(i, &format!("{}exact", " ".repeat(i))));
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
    fn strip_comments_samples() {
        assert_eq!(strip_comments("# whole line comment"), "");
        assert_eq!(strip_comments("    # comment instruction"), "");
        assert_eq!(strip_comments("# comment instruction"), "");
        assert_eq!(strip_comments("######## a lot of #'s"), "");
        assert_eq!(strip_comments("instruction # comment"), "instruction");
        assert_eq!(
            strip_comments("instruction # comment another instruction"),
            "instruction"
        );
    }

    #[test]
    fn strip_comments_trailing_spaces() {
        let space_chars = [
            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '　', ' ', ' ', ' ', ' ', ' ', ' ',
            ' ', '　', ' ', ' ', ' ', ' ', '　', ' ',
        ];

        for i in 1..20 {
            for space in space_chars {
                let expected = format!("text{}", space.to_string().repeat(i));
                assert_eq!(strip_comments(&format!("{}# comment", expected)), "text");
            }
        }
    }

    #[test]
    fn test_escape_characters() {
        let characters = [r#"\n"#, r#"\r"#, r#"\t"#, r#"\\"#, r#"\0"#];
        let escaped = ["\n", "\r", "\t", "\\", "\0"];

        for (c1, e1) in characters.iter().zip(escaped.iter()) {
            for (c2, e2) in characters.iter().zip(escaped.iter()) {
                for (c3, e3) in characters.iter().zip(escaped.iter()) {
                    for (c4, e4) in characters.iter().zip(escaped.iter()) {
                        let original = format!(
                            "random text {} between {} all {} escape {} characters",
                            c1, c2, c3, c4
                        );

                        let expected = format!(
                            "random text {} between {} all {} escape {} characters",
                            e1, e2, e3, e4
                        );

                        assert_eq!(interpret_escape_chars(&original), expected);
                    }
                }
            }
        }
    }
}
