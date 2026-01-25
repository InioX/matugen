use clap::{ArgAction, Parser};
use std::path::PathBuf;

use crate::{color::base16::Backend, SchemesEnum};

#[derive(Parser, Clone)]
#[command(version, long_about = None)]
pub struct Cli {
    /// Optional name to operate on
    // name: Option<String>,
    #[command(subcommand)]
    pub source: crate::color::color::Source,

    /// Sets a custom color scheme type
    #[arg(
        short,
        long,
        value_name = "TYPE",
        global = true,
        default_value = "scheme-tonal-spot"
    )]
    pub r#type: Option<crate::scheme::SchemeTypes>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE", global = true)]
    pub config: Option<PathBuf>,

    /// Adds a prefix before paths
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

    /// Whether to add the image field into json output.
    #[arg(long, global = true)]
    pub include_image_in_json: Option<bool>,

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

    /// Imports a json file to use as render data (can be used multiple times)
    #[arg(value_enum, long, global = true, value_name = "FILE")]
    pub import_json: Option<Vec<String>>,

    /// Imports a json string to use as render data (can be used multiple times)
    #[arg(value_enum, long, global = true, value_name = "STRING")]
    pub import_json_string: Option<Vec<String>>,

    /// Uses a custom resize filter for extracting source color
    #[arg(value_enum, short, long, global = true)]
    pub resize_filter: Option<FilterType>,

    #[arg(long, global = true, action=ArgAction::SetTrue)]
    pub continue_on_error: Option<bool>,

    /// The color which should be used as the source_color if no good color was found from an image. (Overrides config value)
    #[arg(value_enum, long, global = true, value_name = "STRING")]
    pub fallback_color: Option<String>,

    /// Whether to make the outputted json compatible with the --import-json flag.
    #[arg(long, global = true, action=ArgAction::SetTrue)]
    pub alternative_json_output: Option<bool>,

    /// Backend to use for base16 color scheme generation
    #[arg(value_enum, short, long, global = true)]
    pub base16_backend: Option<Backend>,

    #[cfg(feature = "filter-docs")]
    /// Outputs filter documentation in HTML format
    #[arg(long, global = true, action=ArgAction::SetTrue)]
    pub filter_docs_html: Option<bool>,
}

#[derive(Parser, Debug, Clone, clap::ValueEnum)]
pub enum FilterType {
    Nearest,
    Triangle,
    CatmullRom,
    Gaussian,
    Lanczos3,
}

impl From<&FilterType> for image::imageops::FilterType {
    fn from(filter: &FilterType) -> Self {
        match filter {
            FilterType::Nearest => Self::Nearest,
            FilterType::Triangle => Self::Triangle,
            FilterType::CatmullRom => Self::CatmullRom,
            FilterType::Gaussian => Self::Gaussian,
            FilterType::Lanczos3 => Self::Lanczos3,
        }
    }
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

impl ToString for Format {
    fn to_string(&self) -> String {
        match &self {
            Format::Hex => "hex",
            Format::Rgb => "rgb",
            Format::Rgba => "rgba",
            Format::Hsl => "hsl",
            Format::Hsla => "hsla",
            Format::Strip => "hex_stripped",
        }
        .to_owned()
    }
}
