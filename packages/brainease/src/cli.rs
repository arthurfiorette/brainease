use clap::{Parser, Subcommand};
use clap_verbosity_flag::{Verbosity, WarnLevel};

use crate::{run::RunOpts, transpile::TranspileOpts};

#[derive(Parser, Debug)]
#[clap(name = "Brainease")]
#[clap(about = "A brainf*ck-style programming language, but readable")]
pub struct Cli {
  #[clap(subcommand)]
  pub command: Commands,

  #[clap(flatten)]
  pub verbose: Verbosity<WarnLevel>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
  Transpile(TranspileOpts),
  Run(RunOpts),
}
