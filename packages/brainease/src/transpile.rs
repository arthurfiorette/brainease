use std::{io, path::Path};

use brainease_transpiler::{bf_parser, transpile_code};
use clap::Parser;

use crate::util;

#[derive(Parser, Debug)]
#[clap(
  author,
  version,
  name = "Brainease Transpiler",
  about = "Transpile brainf*ck source into brainease code"
)]
pub struct TranspileOpts {
  /// The bf source file to transpile
  main: String,

  /// The output file to write
  #[clap(short = 'o', long = "output")]
  output: Option<String>,
}

pub fn run(opts: &TranspileOpts) -> io::Result<()> {
  let main = Path::new(&opts.main);
  let main_content = util::read_file(main)?;

  log::trace!("Transpiling {}", main.display());

  let transpiled = bf_parser::parse_bf(&main_content);

  log::trace!("{:#?}", transpiled);

  if let Some(output) = &opts.output {
    let transpiled = transpile_code(&transpiled);
    util::write_file(Path::new(output), transpiled)?;
  }

  Ok(())
}
