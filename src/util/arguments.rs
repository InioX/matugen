use clap::{arg, ArgAction, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, long_about = None)]
pub struct Cli {
    /// Optional name to operate on
    // name: Option<String>,
    #[command(subcommand)]
    pub source: Commands,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    #[arg(short, long, global = true, action=ArgAction::SetTrue)]
    pub verbose: Option<bool>,

    #[arg(short, long, global = true, action=ArgAction::SetTrue)]
    pub quiet: Option<bool>,

    /// Whether to use lightmode for the colorscheme
    #[arg(short, long, global = true, action=ArgAction::SetTrue)]
    pub lightmode: Option<bool>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// The image to use for generating a colorscheme
    Image { path: String },
    /// The source color to use for generating a colorscheme
    Color { color: String },
}
