use crate::{messages::Messages, syntax::TokenKind, util::is_empty_line};

/// A struct with all possible headers
#[derive(Debug, Clone, PartialEq)]
pub struct Headers {
  /// The vector size of the brainease program
  /// ```r
  /// @memory_size=30000
  /// ```
  pub memory_size: Option<usize>,

  /// Unrecognized headers
  pub unknown: Vec<String>,
}

impl Headers {
  pub fn from_lines(lines: &[&str]) -> Headers {
    let header_regex = TokenKind::Header.regex();

    let mut headers = Headers {
      memory_size: None,
      unknown: Vec::new(),
    };

    for line in lines {
      if is_empty_line(line) || line.starts_with('@') {
        continue;
      }

      let matches = header_regex.captures(line);

      if matches.is_none() {
        headers.unknown.push(line.to_string());
        continue;
      }

      let captures = matches.unwrap();

      let header = &captures[1];
      let value = &captures.get(2);

      match header {
        "memory_size" => {
          if value.is_none() {
            Messages::HeaderNeedsValue {
              name: header.to_string(),
              line: line.to_string(),
            }
            .log_error();

            continue;
          }

          let value = value.unwrap().as_str().parse::<usize>();

          if value.is_err() {
            Messages::HeaderInvalidValue {
              name: header.to_string(),
              line: line.to_string(),
              required: String::from("number"),
            }
            .log_error();

            continue;
          }

          headers.memory_size = Some(value.unwrap());
        }

        _ => {
          Messages::HeaderInvalidValue {
            name: header.to_string(),
            line: line.to_string(),
            required: String::from("memory_size"),
          }
          .log_warning();

          headers.unknown.push(line.to_string());
        }
      }
    }

    headers
  }
}
