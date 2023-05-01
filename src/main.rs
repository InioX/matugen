extern crate pretty_env_logger;
#[macro_use]
extern crate paris_log;

mod util;
use crate::util::{
    arguments::{Cli, Commands},
    color::{show_color, Color},
    config::ConfigFile,
    image::source_color_from_image,
    template::Template,
};

use log::{Level, LevelFilter};

use color_eyre::{eyre::Result, Report};
use material_color_utilities_rs::{palettes::core::CorePalette, scheme::Scheme};

use clap::Parser;

fn main() -> Result<(), Report> {
    color_eyre::install()?;
    let args = Cli::parse();

    let log_level: LevelFilter = if args.verbose == Some(true) {
        LevelFilter::Info
    } else if args.quiet == Some(true) {
        LevelFilter::Error
    } else {
        LevelFilter::Warn
    };

    pretty_env_logger::env_logger::builder()
        .format_module_path(false)
        .format_timestamp(None)
        .filter_level(log_level)
        .try_init()?;

    let mut palette: CorePalette = match args.source {
        Commands::Image { path } => CorePalette::new(source_color_from_image(&path)?[0], true),
        Commands::Color { color } => {
            if ! color.starts_with("#") {
                // Do something here
            };
            
            let hex_stripped = color[1..].to_string();

            let source_color = Color {
                red: u8::from_str_radix(&hex_stripped[0..2], 16)?,
                green: u8::from_str_radix(&hex_stripped[2..4], 16)?,
                blue: u8::from_str_radix(&hex_stripped[4..6], 16)?,
                alpha: 255,
            };

            CorePalette::new(
                [255, source_color.red, source_color.green, source_color.blue],
                true,
            )
        }
    };

    let scheme: Scheme = if args.lightmode == Some(true) {
        Scheme::light_from_core_palette(&mut palette)
    } else {
        Scheme::dark_from_core_palette(&mut palette)
    };

    let colors = vec![
        "primary",
        "on_primary",
        "primary_container",
        "on_primary_container",
        "secondary",
        "on_secondary",
        "secondary_container",
        "on_secondary_container",
        "tertiary",
        "on_tertiary",
        "tertiary_container",
        "on_tertiary_container",
        "error",
        "on_error",
        "error_container",
        "on_error_container",
        "background",
        "on_background",
        "surface",
        "on_surface",
        "surface_variant",
        "on_surface_variant",
        "outline",
        "outline_variant",
        "shadow",
        "scrim",
        "inverse_surface",
        "inverse_on_surface",
        "inverse_primary",
    ];

    if args.quiet == Some(false) {
        show_color(&scheme, &colors);
    }

    let _config: ConfigFile = ConfigFile::read()?;

    Template::new(&colors, scheme, _config)?;

    Ok(())
}
