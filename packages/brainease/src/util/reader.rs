use std::{fs, io, path::Path};

pub fn read_file(path: &Path) -> io::Result<String> {
  log::trace!("Reading {}", path.display());

  let content = fs::read_to_string(path);

  if content.is_err() {
    log::error!("Could not find file {}", path.display());
  }

  content
}

pub fn write_file(path: &Path, content: String) -> io::Result<()> {
  log::trace!("Writing {}", path.display());

  let result = fs::write(path, content);

  if result.is_err() {
    log::error!("Could not write to {}", path.display());
  }

  result
}
