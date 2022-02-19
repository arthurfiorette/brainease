use brainease_lexer::parser;
use brainease_runtime::{io_handler::DefaultIoHandler, runtime::Runtime};
use clap::Parser;
use std::{
  fs,
  io::{stdout, Write},
  process,
};

pub mod args;
pub mod util;

fn main() {
  let args = args::Args::parse();
  util::fallback_rust_log(&args.log_level);

  log::trace!("Cli args: {:?}", args);

  let (absolute_path, filename) = util::normalize_path(&args.main);

  log::trace!("Reading '{}'", filename);

  let main_file = fs::read_to_string(&absolute_path);

  if main_file.is_err() {
    log::error!("Could not read {} ", absolute_path);
    process::exit(1);
  }

  let main_file = main_file.unwrap();
  let instructions = parser::parse_file(&main_file);

  log::trace!("Instructions: {:?}", instructions);

  let mut runtime = Runtime::<DefaultIoHandler>::new(instructions, args.memory);

  log::debug!("Starting runtime for {}", filename);

  // A little space between stdout
  let elapsed_time = runtime.run();

  stdout().write_all(b"\n").unwrap();
  log::debug!("Elapsed time: {}s.", elapsed_time);
}
