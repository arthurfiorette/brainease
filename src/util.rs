use std::env;

pub fn fallback_rust_log(def: &str) {
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
