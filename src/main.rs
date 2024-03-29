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

use indexmap::IndexMap;

use material_colors::{Hct, Scheme};

use clap::{Parser, ValueEnum};
use color_eyre::{eyre::Result, Report};
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use std::io::Write;
use update_informer::{registry, Check};

use util::{arguments::SchemeTypes, color::harmonize_colors};

use material_colors::{
    SchemeContent, SchemeExpressive, SchemeFidelity, SchemeFruitSalad, SchemeMonochrome,
    SchemeNeutral, SchemeRainbow, SchemeTonalSpot,
};

pub struct Schemes {
    pub light: IndexMap<String, [u8; 4], ahash::random_state::RandomState>,
    pub dark: IndexMap<String, [u8; 4], ahash::random_state::RandomState>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum SchemesEnum {
    Light,
    Dark,
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

    let scheme_dark = generate_scheme(&args.r#type, source_color, true, args.contrast);
    let scheme_light = generate_scheme(&args.r#type, source_color, false, args.contrast);

    let config: ConfigFile = ConfigFile::read(&args)?;

    let default_scheme = args
        .mode
        .expect("Something went wrong while parsing the mode");

    let schemes: Schemes = Schemes {
        dark: IndexMap::from_iter(scheme_dark),
        light: IndexMap::from_iter(scheme_light),
    };

    let mut harmonized_colors = None;

    if let Some(colors) = &config.config.colors_to_harmonize {
        harmonized_colors = Some(harmonize_colors(&source_color, colors));
    };

    if args.show_colors == Some(true) {
        show_color(&schemes, &source_color);
    }

    if let Some(ref format) = args.json {
        dump_json(&schemes, &source_color, format, &harmonized_colors);
    }

    if args.dry_run == Some(false) {
        Template::generate(
            &schemes,
            &config.templates,
            &args.source,
            &config.config.prefix,
            &source_color,
            &default_scheme,
            &config.config.custom_keywords,
            &harmonized_colors,
        )?;

        if config.config.reload_apps == Some(true) {
            #[cfg(any(target_os = "linux", target_os = "netbsd"))]
            reload::unix::reload(&args, &config)?;
        }

        if config.config.set_wallpaper == Some(true) {
            let path = match &args.source {
                Source::Image { path } => path,
                Source::Color { .. } => return Ok(()),
                Source::WebImage { .. } => return Ok(()),
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

fn generate_scheme(
    scheme_type: &Option<SchemeTypes>,
    source_color: [u8; 4],
    is_dark: bool,
    contrast_level: Option<f64>,
) -> Scheme {
    match scheme_type.unwrap() {
        SchemeTypes::SchemeContent => {
            Scheme::from(SchemeContent::new(Hct::new(source_color), is_dark, contrast_level).scheme)
        }
        SchemeTypes::SchemeExpressive => Scheme::from(
            SchemeExpressive::new(Hct::new(source_color), is_dark, contrast_level).scheme,
        ),
        SchemeTypes::SchemeFidelity => Scheme::from(
            SchemeFidelity::new(Hct::new(source_color), is_dark, contrast_level).scheme,
        ),
        SchemeTypes::SchemeFruitSalad => Scheme::from(
            SchemeFruitSalad::new(Hct::new(source_color), is_dark, contrast_level).scheme,
        ),
        SchemeTypes::SchemeMonochrome => Scheme::from(
            SchemeMonochrome::new(Hct::new(source_color), is_dark, contrast_level).scheme,
        ),
        SchemeTypes::SchemeNeutral => {
            Scheme::from(SchemeNeutral::new(Hct::new(source_color), is_dark, contrast_level).scheme)
        }
        SchemeTypes::SchemeRainbow => {
            Scheme::from(SchemeRainbow::new(Hct::new(source_color), is_dark, contrast_level).scheme)
        }
        SchemeTypes::SchemeTonalSpot => Scheme::from(
            SchemeTonalSpot::new(Hct::new(source_color), is_dark, contrast_level).scheme,
        ),
    }
}
