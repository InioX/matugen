use crate::{
    color::color::Source,
    parser::engine::EngineSyntax,
    wallpaper::{self, Wallpaper},
};
use color_eyre::{eyre::Result, Report};
use log::LevelFilter;
use serde_json::{Map, Value};
use std::{fs::read_to_string, io::Write, path::PathBuf};

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

pub fn get_syntax(
    bprefix: Option<&String>,
    bpostfix: Option<&String>,
    eprefix: Option<&String>,
    epostfix: Option<&String>,
) -> EngineSyntax {
    let mut syntax = EngineSyntax::default();

    if let Some(bprefix) = bprefix {
        syntax.block_left = bprefix.clone();
    }
    if let Some(bpostfix) = bpostfix {
        syntax.block_right = bpostfix.clone();
    }
    if let Some(eprefix) = eprefix {
        syntax.keyword_left = eprefix.clone();
    }
    if let Some(epostfix) = epostfix {
        syntax.keyword_right = epostfix.clone();
    }

    syntax
}

pub fn json_from_file(path: &PathBuf) -> Result<serde_json::Value, Report> {
    if !path.exists() {
        error!(
            "<d>The path <red><b>{}</><d> doesnt exist.</>",
            path.display()
        );
    }
    let json_string = read_to_string(path)?;
    let json = serde_json::from_str(&json_string)?;
    Ok(json)
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
        Source::Json { path: _ } => unreachable!(),
    };

    #[cfg(target_os = "windows")]
    wallpaper::windows::set(path)?;
    #[cfg(target_os = "macos")]
    wallpaper::macos::set(&path)?;
    #[cfg(any(target_os = "linux", target_os = "netbsd"))]
    wallpaper::unix::set(path, _wallpaper_cfg)?;
    Ok(())
}

pub fn color_entry(hex: String) -> Value {
    let mut m = Map::new();
    m.insert("color".to_string(), Value::String(hex));
    Value::Object(m)
}

pub fn merge_json(a: &mut Value, b: Value) {
    match (a, b) {
        (Value::Object(a_map), Value::Object(b_map)) => {
            for (k, v_b) in b_map {
                match a_map.get_mut(&k) {
                    Some(v_a) => merge_json(v_a, v_b),
                    None => {
                        a_map.insert(k, v_b);
                    }
                }
            }
        }
        // Arrays: append `b`'s items to `a`
        (Value::Array(a_arr), Value::Array(b_arr)) => {
            a_arr.extend(b_arr);
        }
        // For all other cases: replace `a` with `b`
        (a_slot, b_val) => {
            *a_slot = b_val;
        }
    }
}
