use clap::{Parser, Subcommand};
use clap_verbosity_flag::{Verbosity, WarnLevel};

use crate::{run::RunOpts, transpile::TranspileOpts};

#[derive(Parser, Debug)]
#[clap(author, version, about, name = "Brainease")]
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
