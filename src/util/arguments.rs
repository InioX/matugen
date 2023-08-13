use clap::{arg, ArgAction, Parser, Subcommand};
use material_color_utilities_rs::palettes::core::ColorPalette;
use std::path::PathBuf;

use crate::SchemesEnum;

#[derive(Parser)]
#[command(version, long_about = None)]
pub struct Cli {
    /// Optional name to operate on
    // name: Option<String>,
    #[command(subcommand)]
    pub source: Commands,

    /// Sets a custom color palette
    #[arg(
        short,
        long,
        value_name = "PALETTE",
        global = true,
        default_value = "default"
    )]
    pub palette: Option<ColorPalette>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE", global = true)]
    pub config: Option<PathBuf>,

    #[arg(short, long, global = true, action=ArgAction::SetTrue)]
    pub verbose: Option<bool>,

    /// Whether to only show errors.
    #[arg(short, long, global = true, action=ArgAction::SetTrue)]
    pub quiet: Option<bool>,

    /// Which mode to use for the color scheme
    #[arg(value_enum, short, long, global = true, value_name = "MODE", default_value = "dark")]
    pub mode: Option<SchemesEnum>,

    /// Will not generate templates, reload apps, set wallpaper or run any commands
    #[arg(long, global = true, action=ArgAction::SetTrue)]
    pub dry_run: Option<bool>,

    /// Whether to show colors or not
    #[arg(long, global = true, action=ArgAction::SetTrue, default_value = "false")]
    pub show_colors: Option<bool>,
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
