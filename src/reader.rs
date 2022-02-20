use std::{fs, path::Path, process};

pub fn read_file(path: &Path) -> String {
  log::trace!("Reading {}", path.display());

  let content = fs::read_to_string(path);

  if let Err(err) = content {
    log::error!("Could not find file {}", path.display());
    log::trace!("{}", err);
    process::exit(1);
  }

  content.unwrap()
}
