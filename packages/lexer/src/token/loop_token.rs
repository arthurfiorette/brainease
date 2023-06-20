use lazy_regex::{regex, Captures, Lazy, Regex};

use crate::{parser::parse_partial_file, syntax::Instruction};

use super::Token;

#[derive(Debug, PartialEq, Eq)]
pub struct LoopToken;

impl Token for LoopToken {
    fn name(&self) -> &'static str {
        "loop"
    }

    fn regex(&self) -> &'static Lazy<Regex> {
        static REGEX: &Lazy<Regex> = regex!(r"^loop\s\*(\d+|@)\s*$");

        REGEX
    }

    fn read_instruction(
        &self,
        file: &[&str],
        captures: Captures,
        line_index: usize,
        indentation: usize,
    ) -> (usize, Option<Instruction>) {
        let cell = captures[1].parse().unwrap();

        // Read and parses the inner indentation block
        let (next_line, inner) =
            parse_partial_file(file, line_index + 1, indentation + 2, line_index);

        (next_line, Some(Instruction::Loop { cell, inner }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::find_match;

    #[test]
    fn regex() {
        let regex = (LoopToken).regex();

        assert!(regex.is_match("loop *1"));
        assert!(regex.is_match("loop *@"));

        assert!(!regex.is_match("loop *2  asdfgsdfh random text :)      "));
        assert!(!regex.is_match(" loop"));
        assert!(!regex.is_match("loop *a"));
        assert!(!regex.is_match(" loop *a"));
        assert!(!regex.is_match("loop 'a'"));
        assert!(!regex.is_match(" loop *a"));
        assert!(!regex.is_match("loop 'a' at *1"));
    }

    #[test]
    fn captures() {
        let text = "loop *467";
        let (token, captures) = find_match(text).unwrap();

        assert_eq!(token, &LoopToken);
        assert_eq!(&captures[1], "467");
    }
}
