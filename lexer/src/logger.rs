pub fn unknown_line(line_index: &usize, line: &str) {
  log::error!("Couldn't recognize line {} \"{}\"", line_index + 1, line);
}
