use lazy_regex::{regex, Lazy, Regex};

use crate::{logger, syntax::Instruction};

use super::Token;

#[derive(Debug, PartialEq, Eq)]

pub struct IncrementToken;

impl Token for IncrementToken {
    fn name(&self) -> &'static str {
        "increment"
    }

    fn regex(&self) -> &'static Lazy<Regex> {
        static REGEX: &Lazy<Regex> = regex!(r"^inc\s(\d{1,3})\sin\s\*(\d+|@)\s*$");

        REGEX
    }

    fn read_instruction(
        &self,
        _: &[&str],
        captures: lazy_regex::Captures,
        line_index: usize,
        _: usize,
    ) -> (usize, Option<Instruction>) {
        let val = captures[1].parse().unwrap();
        let cell = captures[2].parse().unwrap();

        if val > u8::MAX as usize {
            logger::value_too_big(&line_index, &val, u8::MAX as usize);
            return (line_index + 1, None);
        }

        (
            line_index + 1,
            Some(Instruction::Increment {
                cell,
                value: val as u8,
            }),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::find_match;

    #[test]
    fn regex() {
        let regex = (IncrementToken).regex();

        assert!(regex.is_match("inc 1 in *1"));
        assert!(regex.is_match("inc 1 in *@"));

        assert!(!regex.is_match("inc 1 in *1   asdfgsdfh random text :)      "));
        assert!(!regex.is_match(" inc 1 in *1"));
        assert!(!regex.is_match("inc a in *1"));
        assert!(!regex.is_match(" inc a in *1"));
        assert!(!regex.is_match("inc a in *a"));
        assert!(!regex.is_match(" inc a in *a"));
        assert!(!regex.is_match("inc 1 in*1"));
    }

    #[test]
    fn captures() {
        let text = "inc 123 in *467";
        let (token, captures) = find_match(text).unwrap();

        assert_eq!(token, &IncrementToken);
        assert_eq!(&captures[1], "123");
        assert_eq!(&captures[2], "467");
    }
}
