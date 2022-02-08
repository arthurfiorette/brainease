pub fn unknown_line(line_index: &usize, line: &str) {
  log::error!("Couldn't recognize line {}. \"{}\"", line_index + 1, line);
}

pub fn extra_characters(line_index: &usize, extra: &str) {
  log::warn!("Extra characters at line {}, \"{}\".", line_index + 1, extra);
}

pub fn value_too_big(line_index: &usize, val: &usize, max: usize) {
  log::error!(
    "Got value too big at line {}. {}, maximum is {}.",
    line_index + 1,
    val,
    max
  );
}

pub fn invalid_char(line_index: &usize, char: &char) {
  log::error!(
    "Invalid character at line {} character {}.",
    line_index + 1,
    char
  );
}

pub fn unknown_indentation(line_index: &usize, indentation: &usize) {
  log::error!(
    "Unmatched indentation at line {}. Expected more than 2, got {}.",
    line_index + 1,
    indentation,
  );
}
