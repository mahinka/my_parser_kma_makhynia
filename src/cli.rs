use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version)]
#[command(
    about = "svg_file_parser - rust-based SVG parser designed to interpret Scalable Vector Graphics (SVG) files.\nAuthor - Anastasiia Makhynia"
)]
pub struct Cli {
    #[command(subcommand)]
    pub(crate) command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Command which parse SVG
    Parse {
        ///provide input file
        #[arg(short, long, value_name = "INPUT")]
        input: Option<PathBuf>,
        ///provide output file
        #[arg(short, long, value_name = "OUTPUT")]
        output: Option<PathBuf>,
    },
}
