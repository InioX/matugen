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

use log::LevelFilter;
use std::process::Command;

use color_eyre::{eyre::Result, eyre::WrapErr, Report};
use material_color_utilities_rs::{
    palettes::core::{ColorPalette, CorePalette},
    scheme::Scheme,
};

use clap::Parser;
use util::{color, reload::reload_apps_linux, wallpaper::set_wallaper};

fn main() -> Result<(), Report> {
    color_eyre::install()?;
    let args = Cli::parse();

    setup_logging(&args)?;

    let mut palette: CorePalette = generate_palette(&args, &args.palette.unwrap())?;
    let config: ConfigFile = ConfigFile::read(&args)?;

    let scheme: Scheme = if args.lightmode == Some(true) {
        Scheme::light_from_core_palette(&mut palette)
    } else if args.amoled == Some(true) {
        Scheme::pure_dark_from_core_palette(&mut palette)
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

    Template::generate(&colors, scheme, &config, &args)?;

    if config.config.reload_apps == Some(true) {
        reload_apps_linux(&args, &config)?;
    }

    if config.config.set_wallpaper == Some(true) {
        set_wallaper(&config, &args)?;
    }

    if args.quiet == Some(false) {
        show_color(&scheme, &colors);
    }

    println!("palette: 3{:?}", &args.palette);

    run_after(&config)?;

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

fn generate_palette(args: &Cli, color_palette: &ColorPalette) -> Result<CorePalette, Report> {
    let palette: CorePalette = match &args.source {
        Commands::Image { path } => {
            CorePalette::new(source_color_from_image(path)?[0], true, color_palette)
        }
        Commands::Color { color } => {
            let mut newcolor = color.clone();

            if color.starts_with('#') {
                newcolor = newcolor[1..].to_string();
            };

            if newcolor.len() < 6 {
                // Handle error
            }

            let source_color = Color {
                red: u8::from_str_radix(&newcolor[0..2], 16)?,
                green: u8::from_str_radix(&newcolor[2..4], 16)?,
                blue: u8::from_str_radix(&newcolor[4..6], 16)?,
                alpha: 255,
            };

            CorePalette::new(
                [255, source_color.red, source_color.green, source_color.blue],
                true,
                color_palette,
            )
        }
    };
    Ok(palette)
}
