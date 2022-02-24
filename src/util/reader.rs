use std::{fs, io, path::Path};

pub fn read_file(path: &Path) -> io::Result<String> {
  log::trace!("Reading {}", path.display());

  let content = fs::read_to_string(path);

  if let Err(err) = &content {
    log::error!("Could not find {}", path.display());
    log::trace!("{:#?}", err);
  }

  content
}

pub fn write_file(path: &Path, contents: Vec<String>) -> io::Result<()> {
  log::trace!("Writing {}", path.display());

  let result = fs::write(path, contents.join("\n"));

  if let Err(err) = &result {
    log::error!("Could not write to {}", path.display());
    log::trace!("{:#?}", err);
  }

  result
}
