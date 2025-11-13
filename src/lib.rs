use clap::Parser;

use cli::{CLI, Commands};
use std::io::Error;

use crate::processors::sort_fasta::sort_fasta_main;

mod cli;
mod generics;
mod processors;

pub fn run() -> Result<(), Error> {
    let cli = CLI::parse();

    match &cli.command {
        Some(Commands::Sort {
            fasta,
            line_length,
            sort_option,
            direction,
            prefix,
            output,
        }) => sort_fasta_main(fasta, line_length, sort_option, direction, prefix, output),
        None => {
            println!("No command provided");
        }
    }

    Ok(())
}
