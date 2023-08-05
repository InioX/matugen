use clap::{arg, ArgAction, Parser, Subcommand, ValueEnum};
use material_color_utilities_rs::palettes::core::ColorPalette;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, long_about = None)]
pub struct Cli {
    /// Optional name to operate on
    // name: Option<String>,
    #[command(subcommand)]
    pub source: Commands,

    /// Sets a custom color palette
    #[arg(short, long, value_name = "COLORSCHEME", global = true, default_value="default")]
    pub palette: Option<ColorPalette>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE", global = true)]
    pub config: Option<PathBuf>,

    #[arg(short, long, global = true, action=ArgAction::SetTrue)]
    pub verbose: Option<bool>,

    #[arg(short, long, global = true, action=ArgAction::SetTrue)]
    pub quiet: Option<bool>,

    /// Whether to use lightmode for the color scheme
    #[arg(short, long, global = true, action=ArgAction::SetTrue)]
    pub lightmode: Option<bool>,

    /// Whether to use amoled mode for the color scheme
    #[arg(short, long, global = true, action=ArgAction::SetTrue)]
    pub amoled: Option<bool>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// The image to use for generating a color scheme
    Image { path: String },
    /// The source color to use for generating a color scheme
    #[clap(subcommand)]
    Color(ColorFormat),
}

#[derive(Parser, Debug)]
pub enum ColorFormat {
   Hex { string: String },
   Rgb { string: String },
   Hsl { string: String },
}