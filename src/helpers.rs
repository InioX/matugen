use crate::{
    color::color::Source,
    wallpaper::{self, Wallpaper},
};
use color_eyre::{eyre::Result, Report};
use log::LevelFilter;
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

#[cfg(feature = "update-informer")]
pub fn check_version() {
    use update_informer::{registry, Check};

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

pub fn setup_logging(args: &Cli) -> Result<(), Report> {
    let log_level = get_log_level(args);

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

pub fn set_wallpaper(source: &Source, _wallpaper_cfg: &Wallpaper) -> Result<(), Report> {
    let path = match &source {
        Source::Image { path } => path,
        Source::Color { .. } => return Ok(()),
        #[cfg(feature = "web-image")]
        Source::WebImage { .. } => return Ok(()),
        Source::Json { path } => unreachable!(),
    };

    #[cfg(target_os = "windows")]
    wallpaper::windows::set(path)?;
    #[cfg(target_os = "macos")]
    wallpaper::macos::set(&path)?;
    #[cfg(any(target_os = "linux", target_os = "netbsd"))]
    wallpaper::unix::set(path, _wallpaper_cfg)?;
    Ok(())
}
