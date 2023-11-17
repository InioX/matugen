extern crate pretty_env_logger;
#[macro_use]
extern crate paris_log;

mod util;
use crate::util::{
    arguments::Cli,
    color::{dump_json, get_source_color, show_color},
    config::ConfigFile,
    template::Template,
};

use log::LevelFilter;
use std::process::Command;

use color_eyre::{eyre::Result, eyre::WrapErr, Report};
use material_color_utilities_rs::{
    palettes::core::{ColorPalette, CorePalette},
    scheme::{scheme::Scheme, scheme_android::SchemeAndroid},
};

use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};

#[cfg(any(target_os = "linux", target_os = "netbsd"))]
use util::{reload::reload_apps_linux, wallpaper::set_wallaper};

pub struct Schemes {
    pub light: Scheme,
    pub dark: Scheme,
    pub amoled: Scheme,
    pub light_android: SchemeAndroid,
    pub dark_android: SchemeAndroid,
    pub amoled_android: SchemeAndroid,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum SchemesEnum {
    Light,
    Dark,
    Amoled,
}

fn main() -> Result<(), Report> {
    color_eyre::install()?;
    let args = Cli::parse();

    setup_logging(&args)?;

    let source_color = get_source_color(&args.source)?;

    let mut palette: CorePalette = generate_palette(&args.palette.unwrap(), source_color)?;

    let config: ConfigFile = ConfigFile::read(&args)?;

    let default_scheme = args
        .mode
        .expect("Something went wrong while parsing the mode");

    let schemes: Schemes = Schemes {
        light: Scheme::light_from_core_palette(&mut palette),
        dark: Scheme::dark_from_core_palette(&mut palette),
        amoled: Scheme::pure_dark_from_core_palette(&mut palette),
        light_android: SchemeAndroid::light_from_core_palette(&mut palette),
        dark_android: SchemeAndroid::dark_from_core_palette(&mut palette),
        amoled_android: SchemeAndroid::pure_dark_from_core_palette(&mut palette),
    };

    if args.dry_run == Some(false) {
        Template::generate(&schemes, &config, &args, &source_color, &default_scheme)?;

        #[cfg(any(target_os = "linux", target_os = "netbsd"))]
        if config.config.reload_apps == Some(true) {
            reload_apps_linux(&args, &config)?;
        }

        #[cfg(any(target_os = "linux", target_os = "netbsd"))]
        if config.config.set_wallpaper == Some(true) {
            set_wallaper(&config, &args)?;
        }

        run_after(&config)?;
    }

    if args.show_colors == Some(true) {
        show_color(&schemes, &source_color);
    }

    if let Some(format) = args.json {
        dump_json(&schemes, &source_color, format);
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
