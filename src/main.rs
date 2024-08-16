#![allow(clippy::too_many_arguments)]

extern crate pretty_env_logger;
#[macro_use]
extern crate paris_log;

mod reload;
mod helpers;
mod wallpaper;
mod util;

use helpers::{check_version, set_wallpaper, setup_logging};
use matugen::{color::color::get_source_color, scheme::scheme::{get_custom_color_schemes, get_schemes}};

use crate::util::{
    arguments::Cli,
    color::{dump_json, show_color},
    config::ConfigFile,
    template::Template,
};

use clap::Parser;
use color_eyre::{eyre::Result, Report};

use matugen::scheme::scheme::{SchemesEnum, Schemes};

fn main() -> Result<(), Report> {
    color_eyre::install()?;
    let args = Cli::parse();

    setup_logging(&args)?;

    let (config, config_path) = ConfigFile::read(&args)?;

    if config.config.version_check == Some(true) {
        check_version();
    }

    let source_color = get_source_color(&args.source).unwrap();

    let (scheme_dark, scheme_light) = get_schemes(source_color, &args.r#type, &args.contrast);

    let default_scheme = args
        .mode
        .expect("Something went wrong while parsing the mode");

    let schemes = get_custom_color_schemes(source_color, scheme_dark, scheme_light, &config.config.custom_colors, &args.r#type, &args.contrast);

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
            &source_color,
            &default_scheme,
            &config.config.custom_keywords,
            &args.prefix,
            config_path,
        )?;

        if config.config.reload_apps == Some(true) {
            #[cfg(any(target_os = "linux", target_os = "netbsd"))]
            reload::unix::reload(&args, &config)?;
        }

        if config.config.set_wallpaper == Some(true) {
            set_wallpaper(&args.source)?;
        }
    }

    Ok(())
}