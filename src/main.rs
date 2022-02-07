use brainease_lexer::parser;
use brainease_runtime::runtime::Runtime;
use clap::Parser;
use std::{env, fs};

pub mod args;

fn main() {
  let args = args::Args::parse();

  {
    let log_env = env::var("RUST_LOG");
    if log_env.is_err() || log_env.unwrap().is_empty() {
      env::set_var("RUST_LOG", args.log_level);
    }

    env_logger::builder()
      .format_indent(None)
      .format_module_path(false)
      .format_target(false)
      .format_timestamp(None)
      .init();
  }

  let main_file = fs::read_to_string(&args.main);

  if main_file.is_err() {
    log::error!("File {} could not be read.", args.main);
    return;
  }

  log::debug!("Running {}...", args.main);

  let main_file = main_file.unwrap();

  let instructions = parser::parse_file(&main_file);
  let mut runtime = Runtime::new(instructions, args.memory);

  // A little space between stdout
  println!();
  let elapsed_time = runtime.run();
  println!("\n");

  log::debug!("Elapsed time: {}s.", elapsed_time);
}
