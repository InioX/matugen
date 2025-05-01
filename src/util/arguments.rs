use clap::{arg, ArgAction, Parser};
use std::path::PathBuf;

use crate::SchemesEnum;

#[derive(Parser, Clone)]
#[command(version, long_about = None)]
pub struct Cli {
    /// Optional name to operate on
    // name: Option<String>,
    #[command(subcommand)]
    pub source: matugen::color::color::Source,

    /// Sets a custom color scheme type
    #[arg(
        short,
        long,
        value_name = "TYPE",
        global = true,
        default_value = "scheme-tonal-spot"
    )]
    pub r#type: Option<matugen::scheme::SchemeTypes>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE", global = true)]
    pub config: Option<PathBuf>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "PATH", global = true)]
    pub prefix: Option<PathBuf>,

    /// Value from -1 to 1. -1 represents minimum contrast, 0 represents
    /// standard (i.e. the design as spec'd), and 1 represents maximum contrast.
    #[arg(long, global = true, allow_negative_numbers = true)]
    pub contrast: Option<f64>,
    
    /// Value from -1 to 1. -1 represents minimum lightness, 0 represents
    /// standard (i.e. the design as spec'd), and 1 represents maximum lightness.
    #[arg(long, global = true, allow_negative_numbers = true)]
    pub lightness: Option<f64>,

    #[arg(short, long, global = true, action=ArgAction::SetTrue)]
    pub verbose: Option<bool>,

    /// Whether to show no output.
    #[arg(short, long, global = true, action=ArgAction::SetTrue)]
    pub quiet: Option<bool>,

    /// Whether to show debug output.
    #[arg(short, long, global = true, action=ArgAction::SetTrue)]
    pub debug: Option<bool>,

    /// Which mode to use for the color scheme
    #[arg(
        value_enum,
        short,
        long,
        global = true,
        value_name = "MODE",
        default_value = "dark"
    )]
    pub mode: Option<SchemesEnum>,

    /// Will not generate templates, reload apps, set wallpaper or run any commands
    #[arg(long, global = true, action=ArgAction::SetTrue)]
    pub dry_run: Option<bool>,

    /// Whether to show colors or not
    #[arg(long, global = true, action=ArgAction::SetTrue, default_value = "false")]
    pub show_colors: Option<bool>,

    #[cfg(feature = "dump-json")]
    /// Whether to dump json of colors
    #[arg(value_enum, short, long, global = true, value_name = "JSON")]
    pub json: Option<Format>,
}

#[derive(Parser, Debug)]
pub enum ColorFormat {
    Hex { string: String },
    Rgb { string: String },
    Hsl { string: String },
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Format {
    Hex,
    Rgb,
    Rgba,
    Hsl,
    Hsla,
    Strip,
}
