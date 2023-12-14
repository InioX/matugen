extern crate pretty_env_logger;
#[macro_use]
extern crate paris_log;

mod reload;
mod util;
mod wallpaper;

use crate::util::{
    arguments::{Cli, Source},
    color::{dump_json, get_source_color, show_color},
    config::ConfigFile,
    template::Template,
};

use material_color_utilities_rs::{
    palettes::core::{ColorPalette, CorePalette},
    scheme::{scheme::Scheme, scheme_android::SchemeAndroid},
};

use std::collections::HashMap;

use clap::{Parser, ValueEnum};
use color_eyre::{eyre::Result, Report};
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use std::io::Write;
use update_informer::{registry, Check};

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

    let log_level: LevelFilter = if args.verbose == Some(true) {
        LevelFilter::Info
    } else if args.quiet == Some(true) {
        LevelFilter::Off
    } else if args.debug == Some(true) {
        LevelFilter::Debug
    } else {
        LevelFilter::Warn
    };
    
    setup_logging(log_level)?;
    
    check_version();
    
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

    if args.show_colors == Some(true) {
        show_color(&schemes, &source_color);
    }

    if let Some(format) = args.json {
        dump_json(&schemes, &source_color, format);
    }

    if args.dry_run == Some(false) {
        Template::generate(&schemes, &config.templates, &args.source, &config.config.prefix, &source_color, &default_scheme, config.config.custom_keywords)?;

        if config.config.reload_apps == Some(true) {
            #[cfg(any(target_os = "linux", target_os = "netbsd"))]
            reload::unix::reload(&args, &config)?;
        }

        if config.config.set_wallpaper == Some(true) {
            let path = match &args.source {
                Source::Image { path } => path,
                Source::Color { .. } => return Ok(()),
            };

            #[cfg(any(target_os = "linux", target_os = "netbsd"))]
            let wallpaper_tool = match &config.config.wallpaper_tool {
                Some(wallpaper_tool) => wallpaper_tool,
                None => {
                    if cfg!(windows) {
                        return Ok(());
                    }
                    return Ok(warn!(
                        "<d>Wallpaper tool not set, not setting wallpaper...</>"
                    ));
                }
            };

            #[cfg(target_os = "windows")]
            wallpaper::windows::set(&path)?;

            #[cfg(target_os = "macos")]
            wallpaper::macos::set(&path)?;

            #[cfg(any(target_os = "linux", target_os = "netbsd"))]
            wallpaper::unix::set(
                path,
                wallpaper_tool,
                &config.config.swww_options,
                &config.config.feh_options,
            )?;
        }
    }

    Ok(())
}

fn check_version() {
    let name = env!("CARGO_PKG_NAME");
    let current_version = env!("CARGO_PKG_VERSION");
    let informer = update_informer::new(registry::Crates, name, current_version);

    if let Some(version) = informer.check_version().ok().flatten() {
        warn!(
            "New version is available: <b><red>{}</> -> <b><green>{}</>",
            current_version, version
        );
    }
}

fn setup_logging(log_level: LevelFilter) -> Result<(), Report> {
    pretty_env_logger::env_logger::builder()
        .format_module_path(false)
        .format_timestamp(None)
        .filter_level(log_level)
        .format(|buf, record| writeln!(buf, "{}", record.args()))
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
