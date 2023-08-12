extern crate pretty_env_logger;
#[macro_use]
extern crate paris_log;

mod util;
use crate::util::{
    arguments::{Cli, ColorFormat, Commands},
    color::show_color,
    config::ConfigFile,
    image::source_color_from_image,
    template::Template,
};

use log::LevelFilter;
use std::process::Command;
use std::str::FromStr;

use color_eyre::{eyre::Result, eyre::WrapErr, Report};
use material_color_utilities_rs::{
    palettes::core::{ColorPalette, CorePalette},
    scheme::Scheme,
};

use colorsys::{ColorAlpha, Hsl, Rgb};

use clap::Parser;
use util::{reload::reload_apps_linux, wallpaper::set_wallaper};

fn main() -> Result<(), Report> {
    color_eyre::install()?;
    let args = Cli::parse();

    setup_logging(&args)?;

    let source_color = get_source_color(&args)?;

    let mut palette: CorePalette = generate_palette(&args.palette.unwrap(), source_color)?;

    let config: ConfigFile = ConfigFile::read(&args)?;

    let scheme: Scheme = if args.lightmode == Some(true) {
        Scheme::light_from_core_palette(&mut palette)
    } else if args.amoled == Some(true) {
        Scheme::pure_dark_from_core_palette(&mut palette)
    } else {
        Scheme::dark_from_core_palette(&mut palette)
    };

    let colors = vec![
        "source_color",
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

    Template::generate(&colors, scheme, &config, &args, &source_color)?;

    if config.config.reload_apps == Some(true) {
        reload_apps_linux(&args, &config)?;
    }

    if config.config.set_wallpaper == Some(true) {
        set_wallaper(&config, &args)?;
    }

    run_after(&config)?;

    if args.quiet == Some(false) {
        show_color(&scheme, &colors, &source_color);
    }

    Ok(())
}

fn run_after(config: &ConfigFile) -> Result<(), Report> {
    if let Some(commands) = &config.config.run_after {
        for command in commands {
            if command.is_empty() {
                continue;
            }

            info!("Running: {:?}", command);

            let mut cmd = Command::new(&command[0]);
            for arg in command.iter().skip(1) {
                cmd.arg(arg);
            }
            cmd.spawn()
                .wrap_err(format!("Error when runnning command: {:?}", cmd))?;
        }
    };
    Ok(())
}

fn setup_logging(args: &Cli) -> Result<(), Report> {
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
    Ok(())
}

fn get_source_color(args: &Cli) -> Result<[u8; 4], Report> {
    let source_color: [u8; 4] = match &args.source {
        Commands::Image { path } => source_color_from_image(path)?[0],
        Commands::Color(color) => {
            let src: Rgb;

            match color {
                ColorFormat::Hex { string } => {
                    src = Rgb::from_hex_str(string).expect("Invalid hex color string provided")
                }
                ColorFormat::Rgb { string } => {
                    src = string.parse().expect("Invalid rgb color string provided")
                }
                ColorFormat::Hsl { string } => {
                    src = Hsl::from_str(string)
                        .expect("Invalid hsl color string provided")
                        .into()
                }
            }
            [
                src.alpha() as u8,
                src.red() as u8,
                src.blue() as u8,
                src.green() as u8,
            ]
        }
    };
    Ok(source_color)
}

fn generate_palette(
    color_palette: &ColorPalette,
    source_color: [u8; 4],
) -> Result<CorePalette, Report> {
    debug!("{:?}", source_color);

    Ok(CorePalette::new(
        [
            source_color[0],
            source_color[1],
            source_color[2],
            source_color[3],
        ],
        true,
        color_palette,
    ))
}
