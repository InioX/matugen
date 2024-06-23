use clap::{arg, ArgAction, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Clone, clap::ValueEnum, Debug, Copy)]
pub enum SchemeTypes {
    SchemeContent,
    SchemeExpressive,
    SchemeFidelity,
    SchemeFruitSalad,
    SchemeMonochrome,
    SchemeNeutral,
    SchemeRainbow,
    SchemeTonalSpot,
}

use crate::SchemesEnum;

#[derive(Parser)]
#[command(version, long_about = None)]
pub struct Cli {
    /// Optional name to operate on
    // name: Option<String>,
    #[command(subcommand)]
    pub source: Source,

    /// Sets a custom color scheme type
    #[arg(
        short,
        long,
        value_name = "TYPE",
        global = true,
        default_value = "scheme-tonal-spot"
    )]
    pub r#type: Option<SchemeTypes>,

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

    /// Whether to dump json of colors
    #[arg(
        value_enum,
        short,
        long,
        global = true,
        value_name = "JSON",
        default_value = None,
    )]
    pub json: Option<Format>,
}

#[derive(Subcommand, Debug)]
pub enum Source {
    /// The image to use for generating a color scheme
    Image { path: String },
    /// The image to fetch from web and use for generating a color scheme
    WebImage { url: String },
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

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Format {
    Hex,
    Rgb,
    Rgba,
    Hsl,
    Hsla,
    Strip,
}
