pub static BF_CHARS: [char; 8] = ['+', '-', '>', '<', '.', ',', '[', ']'];

pub fn is_bf_char(c: &char) -> bool {
  BF_CHARS.contains(c)
}

pub fn filter_bf_chars(str: &str) -> Vec<char> {
  str.chars().into_iter().filter(is_bf_char).collect()
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
  fn filter() {
    for loop_char in BF_CHARS {
      let rnd_str = format!("random ignored chars {} random ignored chars", loop_char);
      let parsed = filter_bf_chars(&rnd_str);

      assert_eq!(parsed.len(), 1);
      assert_eq!(parsed, vec![loop_char]);
    }
  }
}
