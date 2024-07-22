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

use material_colors::{color::Argb, scheme::Scheme};

use clap::{Parser, ValueEnum};
use color_eyre::{eyre::Result, Report};
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io::Write};
use update_informer::{registry, Check};

use util::color::{generate_dynamic_scheme, make_custom_color};

pub struct Schemes {
    pub light: IndexMap<String, Argb, ahash::random_state::RandomState>,
    pub dark: IndexMap<String, Argb, ahash::random_state::RandomState>,
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

    let (config, config_path) = ConfigFile::read(&args)?;

    if config.config.version_check == Some(true) {
        check_version();
    }

    let source_color = get_source_color(&args.source)?;

    let scheme_dark = Scheme::from(generate_dynamic_scheme(
        &args.r#type,
        source_color,
        true,
        args.contrast,
    ));
    let scheme_light = Scheme::from(generate_dynamic_scheme(
        &args.r#type,
        source_color,
        false,
        args.contrast,
    ));

    let default_scheme = args
        .mode
        .expect("Something went wrong while parsing the mode");

    let empty = HashMap::new();
    let custom_colors = config
        .config
        .custom_colors
        .as_ref()
        .unwrap_or(&empty)
        .iter()
        .map(|(name, color)| {
            make_custom_color(
                color
                    .to_custom_color(name.to_string())
                    .expect(&format!("Failed to parse custom color: {}, {:?}", name, color)),
                &args.r#type,
                source_color,
                args.contrast,
            )
        });
    macro_rules! from_color {
        ($color: expr, $variant: ident) => {
            [
                (format!("{}_source", $color.color.name), $color.color.value),
                (format!("{}_value", $color.color.name), $color.color.value),
                (format!("{}", $color.color.name), $color.$variant.color),
                (
                    format!("on_{}", $color.color.name),
                    $color.$variant.on_color,
                ),
                (
                    format!("{}_container", $color.color.name),
                    $color.$variant.color_container,
                ),
                (
                    format!("on_{}_container", $color.color.name),
                    $color.$variant.on_color_container,
                ),
            ]
        };
    }
    let custom_colors_dark = custom_colors.clone().flat_map(|c| from_color!(c, dark));
    let custom_colors_light = custom_colors.flat_map(|c| from_color!(c, light));

    let schemes: Schemes = Schemes {
        dark: IndexMap::from_iter(scheme_dark.into_iter().chain(custom_colors_dark)),
        light: IndexMap::from_iter(scheme_light.into_iter().chain(custom_colors_light)),
    };

    if args.show_colors == Some(true) {
        show_color(&schemes, &source_color);
    }

    if let Some(ref format) = args.json {
        dump_json(&schemes, &source_color, format);
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
            &args.prefix,
            config_path
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
                &config.config.feh_options,
                &config.config.swww_options,
            )?;
        }
    }

    Ok(())
}

fn check_version() {
    let name = env!("CARGO_PKG_NAME");
    let current_version = env!("CARGO_PKG_VERSION");
    // for testing
    // let current_version = "2.2.0";

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
