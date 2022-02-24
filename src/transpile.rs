use std::{io, path::Path};

use brainease_runtime::io_handler::DefaultIoHandler;
use brainease_runtime::runtime::Runtime;
use brainease_transpiler::as_string::AsString;
use brainease_transpiler::parser;
use clap::Parser;

use crate::run::start_runtime;
use crate::util;

#[derive(Parser, Debug)]
#[clap(
  name = "transpile",
  about = "Transpile Brainease source code to Brainfuck"
)]
pub struct TranspileOpts {
  /// The bf source file to transpile
  #[clap(short = 'f', long = "file")]
  main: String,

  /// The output file to write
  #[clap(short = 'o', long = "output")]
  output: Option<String>,

  #[clap(short = 'r', long = "run", help = "Run the transpiled code")]
  run_after: bool,

  #[clap(
    long = "memory",
    help = "The memory length when -r is used",
    default_value = "3000"
  )]
  memory_length: usize,
}

pub fn run(opts: &TranspileOpts) -> io::Result<()> {
  let main = Path::new(&opts.main);
  let main_content = util::read_file(main)?;

  log::trace!("Transpiling {}", main.display());

  let transpiled = parser::parse_bf(&main_content);

  log::trace!("{:#?}", transpiled);

  let output_name = &opts.output.clone().unwrap_or_else(|| {
    if opts.main.ends_with(".bf") {
      opts.main.replace(".bf", ".brain")
    } else {
      format!("{}.brain", opts.main)
    }
  });

  let string_lines = AsString::all(&transpiled, 0);

  util::write_file(Path::new(output_name), string_lines)?;

  if opts.run_after {
    let mut runtime = Runtime::new(transpiled, opts.memory_length, DefaultIoHandler {});

    start_runtime(&mut runtime)?;
  }

  Ok(())
}
