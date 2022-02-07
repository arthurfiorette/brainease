use clap::Parser;

/// Brainease command line interface.
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
  /// The main brainease file to run
  #[clap(short = 'f', long = "file")]
  pub main: String,

  /// The length to initialize the memory array
  #[clap(short = 'm', long = "memory", default_value = "3000")]
  pub memory: usize,

  /// The compiler log level to use
  #[clap(long = "log", default_value = "trace")]
  pub log_level: String,
}
