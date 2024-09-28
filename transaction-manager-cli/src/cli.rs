use clap::{Parser};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Input CSV containing transactions
    // TODO: Could make this an argument that takes a flag
    pub input: PathBuf,

    // TODO: In the future we could add an output flag
    //   which would let us choose the output file
}
