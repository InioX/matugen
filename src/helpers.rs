use color_eyre::{eyre::Result, Report};
use log::LevelFilter;
use matugen::{color::color::Source, wallpaper};
use update_informer::{registry, Check};
use std::io::Write;

use crate::util::arguments::Cli;


pub fn get_log_level(args: &Cli) -> LevelFilter {
    let log_level: LevelFilter = if args.verbose == Some(true) {
        LevelFilter::Info
    } else if args.quiet == Some(true) {
        LevelFilter::Off
    } else if args.debug == Some(true) {
        LevelFilter::Debug
    } else {
        LevelFilter::Warn
    };
    log_level
}

pub fn check_version() {
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

pub fn setup_logging(args: &Cli) -> Result<(), Report> {
    let log_level = get_log_level(&args);

    let mut logger = pretty_env_logger::env_logger::builder();
    
    logger.filter_level(log_level);

    if log_level != LevelFilter::Debug {
        logger.format_module_path(false);
        logger.format(|buf, record| writeln!(buf, "{}", record.args()));
    } else {
        // logger.format_timestamp(Some(pretty_env_logger::env_logger::fmt::TimestampPrecision::Nanos));
        logger.format_timestamp_micros();
    }

    logger.try_init()?;

    Ok(())
}

pub fn set_wallpaper(source: &Source) -> Result<(), Report> {
    let path = match &source {
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
    wallpaper::windows::set(path)?;
    #[cfg(target_os = "macos")]
    wallpaper::macos::set(&path)?;
    #[cfg(any(target_os = "linux", target_os = "netbsd"))]
    wallpaper::unix::set(
        path,
        wallpaper_tool,
        &config.config.feh_options,
        &config.config.swww_options,
    )?;
    Ok(())
}