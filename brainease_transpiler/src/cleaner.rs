pub static BF_BYTES: [u8; 8] = [b'+', b'-', b'>', b'<', b'.', b',', b'[', b']'];
pub static BF_CHARS: [char; 8] = ['+', '-', '>', '<', '.', ',', '[', ']'];

pub fn is_bf_char(c: &char) -> bool {
  BF_BYTES.contains(&(*c as u8))
}

// Return an array of brainf*ck keywords as u8 bytes
pub fn clean_bf_code(bf_code: &str) -> Vec<u8> {
  bf_code
    .as_bytes()
    .iter()
    .cloned()
    .filter(|c| BF_BYTES.contains(c))
    .collect()
}

#[cfg(test)]
pub mod tests {
  use super::*;

  #[test]
  fn normal_chars() {
    for char in BF_CHARS {
      assert!(is_bf_char(&char));
    }
  }

  #[test]
  fn clean_code() {
    for char1 in BF_CHARS {
      for char2 in BF_CHARS {
        for char3 in BF_CHARS {
          for char4 in BF_CHARS {
            let code = format!(
              "ignored {} chars {} hello {} world {}",
              char1, char2, char3, char4
            );

            let cleaned = clean_bf_code(&code);

            assert_eq!(
              String::from_utf8_lossy(&cleaned),
              format!("{}{}{}{}", char1, char2, char3, char4),
              // Error msg
              "Clean code with ({}, {}, {}, {})",
              char1,
              char2,
              char3,
              char4
            );
          }
        }
      }
    }
  }
}
