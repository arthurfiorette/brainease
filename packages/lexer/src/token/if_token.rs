use lazy_regex::{regex, Captures, Lazy, Regex};

use crate::{
    parser::parse_partial_file,
    syntax::{CellOrPointer, Instruction},
    util::interpret_escape_chars,
};

use super::Token;

#[derive(Debug, PartialEq, Eq)]
pub struct IfToken;

impl Token for IfToken {
    fn name(&self) -> &'static str {
        "if"
    }

    fn regex(&self) -> &'static Lazy<Regex> {
        // https://regex101.com/r/kZ8kfi/2
        static REGEX: &Lazy<Regex> =
            regex!(r"^if\s\*(\d+|@)\s(==|!=|>|<|<=|>=)\s(\d{1,3}|\*(?:\d+|@)|'\\?.')\s*$");

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
        let logic = captures[2].parse().unwrap();

        let (next_line, inner) =
            parse_partial_file(file, line_index + 1, indentation + 2, line_index);

        // 4 types of If
        //
        // 1:   if *12 == 'A'  # Char
        // 2:   if *12 == *@   # Pointer
        // 3:   if *12 == *12  # Cell
        // 4:   if *12 == 12   # Value

        // If 1
        if captures[3].starts_with('\'') {
            let char = interpret_escape_chars(
                // Only the second char -- 'A' -> A
                &captures[3][1..2],
            )
            .parse::<char>()
            .unwrap();

            return (
                next_line,
                Some(Instruction::If {
                    cell,
                    logic,
                    inner,
                    is_cell: false,
                    cell_or_value: CellOrPointer::Cell(char as usize),
                }),
            );
        }

        // If 2 and 3
        if let Some(txt) = captures[3].strip_prefix('*') {
            let cell_or_value = txt.parse::<CellOrPointer>().unwrap();

            return (
                next_line,
                Some(Instruction::If {
                    cell,
                    logic,
                    inner,
                    is_cell: true,
                    cell_or_value,
                }),
            );
        }

        // If 4

        let value = captures[3].parse::<usize>().unwrap();

        (
            next_line,
            Some(Instruction::If {
                cell,
                logic,
                inner,
                is_cell: false,
                cell_or_value: CellOrPointer::Cell(value),
            }),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::find_match;

    static LEFT_OPERATORS: [&str; 2] = ["*@", "*1"];
    static OPERATORS: [&str; 6] = ["==", "!=", "<=", ">=", ">", "<"];
    static RIGHT_OPERATORS: [&str; 6] = ["*@", "*53", "23", "'A'", "' '", "'\\n'"];

    #[test]
    fn regex() {
        let regex = (IfToken).regex();

        for left in LEFT_OPERATORS {
            for right in RIGHT_OPERATORS {
                for wrongly_operator in [
                    "&", "|", "^", "%", "<<", ">>", "~", "!", "=", "+", "-", "*", "/", "\\", "--",
                ] {
                    let expression = format!("if {} {} {}", left, wrongly_operator, right);

                    assert!(!regex.is_match(&expression));
                }

                for operator in OPERATORS {
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
            }
        }
    }

    #[test]
    fn captures() {
        for left in LEFT_OPERATORS {
            for operator in OPERATORS {
                for right in RIGHT_OPERATORS {
                    let expression = format!("if {} {} {}", left, operator, right);

                    let (token, captures) = find_match(&expression).unwrap();

                    assert_eq!(token, &IfToken);
                    assert_eq!(format!("*{}", &captures[1]), left);
                    assert_eq!(&captures[2], operator);
                    assert_eq!(&captures[3], right);
                }
            }
        }
    }
}
