use clap_verbosity_flag::{LogLevel, Verbosity};

pub fn setup_logger<L>(verbose: &Verbosity<L>)
where
  L: LogLevel,
{
  env_logger::builder()
    .filter_level(verbose.log_level_filter())
    .format_indent(None)
    .format_module_path(false)
    .format_target(false)
    .format_timestamp(None)
    .init();
}
