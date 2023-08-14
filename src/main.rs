extern crate pretty_env_logger;
#[macro_use]
extern crate paris_log;

mod util;
use crate::util::{
    arguments::Cli,
    color::{show_color, get_source_color},
    config::ConfigFile,
    template::Template,
};

use log::LevelFilter;
use std::process::Command;

use color_eyre::{eyre::Result, eyre::WrapErr, Report};
use material_color_utilities_rs::{
    palettes::core::{ColorPalette, CorePalette},
    scheme::Scheme,
};

use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};

use util::{reload::reload_apps_linux, wallpaper::set_wallaper};
pub struct Schemes {
    pub light: Scheme,
    pub dark: Scheme,
    pub amoled: Scheme,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum SchemesEnum {
    Light,
    Dark,
    Amoled,
}

const COLORS: [&str; 30] = [
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

const COLORS_ANDROID: [&str; 25] = [
    "color_accent_primary",
    "color_accent_primary_variant",
    "color_accent_secondary",
    "color_accent_secondary_variant",
    "color_accent_tertiary",
    "color_accent_tertiary_variant",
    "text_color_primary",
    "text_color_secondary",
    "text_color_tertiary",
    "text_color_primary_inverse",
    "text_color_secondary_inverse",
    "text_color_tertiary_inverse",
    "color_background",
    "color_background_floating",
    "color_surface",
    "color_surface_variant",
    "color_surface_highlight",
    "surface_header",
    "under_surface",
    "off_state",
    "accent_surface",
    "text_primary_on_accent",
    "text_secondary_on_accent",
    "volume_background",
    "scrim",
];

fn main() -> Result<(), Report> {
    color_eyre::install()?;
    let args = Cli::parse();
    
    setup_logging(&args)?;

    let source_color = get_source_color(&args.source)?;

    let mut palette: CorePalette = generate_palette(&args.palette.unwrap(), source_color)?;

    let config: ConfigFile = ConfigFile::read(&args)?;

    let default_scheme = args.mode.expect("Something went wrong while parsing the mode");

    let schemes: Schemes = Schemes {
        light: Scheme::light_from_core_palette(&mut palette),
        dark: Scheme::dark_from_core_palette(&mut palette),
        amoled: Scheme::pure_dark_from_core_palette(&mut palette),
    };


    if args.dry_run == Some(false) {
        Template::generate(
            &COLORS,
            &schemes,
            &config,
            &args,
            &source_color,
            &default_scheme,
        )?;

        if config.config.reload_apps == Some(true) {
            reload_apps_linux(&args, &config)?;
        }

        if config.config.set_wallpaper == Some(true) {
            set_wallaper(&config, &args)?;
        }

        run_after(&config)?;
    }

    if args.show_colors == Some(true) {
        show_color(&schemes, &COLORS, &source_color);
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
