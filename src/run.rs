use std::{
  io::{self, stdout, Write},
  path::Path,
};

use brainease_lexer::parser;
use brainease_runtime::{
  io_handler::{DefaultIoHandler, IoHandler},
  runtime::Runtime,
};
use clap::Parser;

use crate::util;

#[derive(Parser, Debug)]
#[clap(name = "run", about = "Run Brainease source code")]
pub struct RunOpts {
  /// The main brainease file to run
  #[clap(short = 'f', long = "file")]
  main: String,

  /// The length to initialize the memory array
  #[clap(short = 'm', long = "memory", default_value = "30000")]
  memory_length: usize,
}

pub fn run(opts: &RunOpts) -> io::Result<()> {
  let main = Path::new(&opts.main);

  // Checks for .main file format
  if !opts.main.ends_with(".brain") {
    log::error!(
      "{:?} has an unknown file format of {:?}",
      main.file_name().unwrap(),
      main.extension().unwrap()
    );

    return Ok(());
  }

  let main_content = util::read_file(main)?;

  log::trace!("Parsing {}", main.display());

  let instructions = parser::parse_file(&main_content);

  log::trace!("{:#?}", instructions);

  let mut runtime = Runtime::new(instructions, opts.memory_length, DefaultIoHandler {});

  start_runtime(&mut runtime)?;

  Ok(())
}

pub fn start_runtime<I>(runtime: &mut Runtime<I>) -> Result<(), I::Err>
where
  I: IoHandler,
{
  log::debug!("Starting runtime");

  let elapsed_time = runtime.run()?;

  log::debug!("Elapsed time: {}s.", elapsed_time.as_secs_f64());

  Ok(())
}
