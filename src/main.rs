use brainease_lexer::parser;
use brainease_runtime::{io_handler::DefaultIoHandler, runtime::Runtime};
use clap::Parser;
use std::{
  io::{stdout, Result, Write},
  path::Path,
  process,
};

pub mod args;
pub mod reader;
pub mod util;

fn main() -> Result<()> {
  let args = args::Args::parse();
  util::fallback_rust_log(&args.log_level);

  log::trace!("Cli args: {:?}", args);

  if !args.main.ends_with(".brain") {
    log::error!("{} does not end with .brain", args.main);
    process::exit(1);
  }

  let path = Path::new(&args.main);

  let main_file = reader::read_file(path);
  let instructions = parser::parse_file(&main_file);

  log::trace!("Instructions: {:?}", instructions);

  let mut runtime = Runtime::new(instructions, args.memory, DefaultIoHandler {});

  log::debug!("Starting runtime");

  // A little space between stdout
  let elapsed_time = runtime.run()?;

  stdout().write_all(b"\n").unwrap();

  log::debug!("Elapsed time: {}s.", elapsed_time.as_secs_f64());

  Ok(())
}
