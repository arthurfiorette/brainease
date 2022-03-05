mod cli;
mod run;
mod transpile;
mod util;

use std::io::{self, stdout, Write};

use clap::Parser;
use cli::{Cli, Commands};

fn main() -> io::Result<()> {
  let args = Cli::parse();

  util::setup_logger(&args.verbose);

  let result = match &args.command {
    Commands::Transpile(opts) => transpile::run(opts),
    Commands::Run(opts) => run::run(opts),
  };

  if let Err(err) = result {
    log::trace!("{:#?}", err)
  }

  stdout().flush()?;

  Ok(())
}
