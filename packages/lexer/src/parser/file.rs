use super::parse_instruction;
use crate::syntax::Instruction;

pub fn parse_file(file: &str) -> Vec<Instruction> {
    let lines: Vec<&str> = file.lines().collect();
    let (_, instructions) = parse_partial_file(&lines, 0, 0, 0);

    instructions
}

/// Parses a file from a specified starting point. Returns the next line to parse and the
/// read instructions.
pub fn parse_partial_file(
    file: &[&str],
    starting_line: usize,
    starting_indentation: usize,
    last_contentful_line: usize,
) -> (usize, Vec<Instruction>) {
    let mut instructions = Vec::new();
    let mut line_index = starting_line;
    let mut indentation = starting_indentation;

    // This variable is used to keep track of the last contentful line
    // that was parsed, so if, after an indentation block, there is one or
    // more empty lines, we don't need to include them at the "next_line"
    // needed to be parsed.
    let mut last_contentful_line = last_contentful_line;

    while line_index < file.len() {
        let result = parse_instruction(file, indentation, line_index);

        // Occurred an error while parsing the instruction.
        // Return an empty vector.
        if result.error {
            return (file.len(), Vec::new());
        }

        // Indentation is over, we're done.
        if result.new_indentation < starting_indentation {
            break;
        }

        if let Some(instruction) = result.instruction {
            instructions.push(instruction);

            // It's used next_line - 1 instead of line_index, because
            // the line_index may not account empty lines (and etc) between
            // the line_index and his previous one. So, next_line - 1
            // will always be the last contentful line.
            last_contentful_line = result.next_line - 1;
        }

        line_index = result.next_line;
        indentation = result.new_indentation;
    }

    (last_contentful_line + 1, instructions)
}

#[cfg(test)]
mod tests {
    use crate::syntax::{CellOrChar, CellOrPointer, Instruction};

    use super::parse_file;

    #[test]
    pub fn test_double_loop() {
        let code = "
save '.' at *@
loop *@
  loop *@
    print *@
";

        let parsed = parse_file(code);

        assert_eq!(
            parsed,
            vec![
                Instruction::Save {
                    cell: CellOrPointer::Pointer,
                    value: b'.',
                },
                Instruction::Loop {
                    cell: CellOrPointer::Pointer,
                    inner: vec![Instruction::Loop {
                        cell: CellOrPointer::Pointer,
                        inner: vec![Instruction::Print(CellOrChar::pointer())]
                    }]
                }
            ]
        )
    }
}
