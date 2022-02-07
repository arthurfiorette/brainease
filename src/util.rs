use std::{env, path::Path};

pub fn fallback_rust_log(def: &String) {
  let log_env = env::var("RUST_LOG");
  if log_env.is_err() || log_env.unwrap().is_empty() {
    env::set_var("RUST_LOG", def);
  }

  env_logger::builder()
    .format_indent(None)
    .format_module_path(false)
    .format_target(false)
    .format_timestamp(None)
    .init();
}

/// Returns the absolute path and the filename
pub fn normalize_path(raw: &String) -> (String, String) {
  let path = Path::new(raw);

  // There has to be a better way
  (
    path.canonicalize().unwrap().to_str().unwrap().to_string(),
    path.file_name().unwrap().to_str().unwrap().to_string(),
  )
}
