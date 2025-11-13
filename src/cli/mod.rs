use clap_derive::{Parser, Subcommand};

pub const SORT_OPTIONS: [&str; 4] = ["SLT", "SLB", "SN", "SL"];

pub const DIRECTION: [&str; 2] = ["ASCENDING", "DESCENDING"];

// CLI for FASTA_FIXA
#[derive(Parser)]
#[command(version="v0.1.0", about, long_about = None)]
pub struct CLI {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Sort {
        // INPUT FASTA
        #[arg(short = 'f', long, required = true)]
        fasta: String,

        // LINE LENGTH OF FASTA OUTPUT
        #[arg(short = 'l', long, default_value_t = 70)]
        line_length: usize,

        // SORT OPTION
        #[arg(short = 's', long, required = true, value_parser = clap::builder::PossibleValuesParser::new(SORT_OPTIONS))]
        sort_option: String,

        // DIRECTION
        #[arg(short = 'd', long, default_value_t = String::from("ASCENDING"), value_parser = clap::builder::PossibleValuesParser::new(DIRECTION))]
        direction: String,

        // OUTPUT FILE PREFIX
        #[arg(short = 'p', long, default_value_t = String::from("sorted_fasta"))]
        prefix: String,

        // OUTPUT LOCATION
        #[arg(short = 'o', long, default_value_t = String::from("./"))]
        output: String,
    },
}
