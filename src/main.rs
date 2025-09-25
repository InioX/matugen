#![allow(clippy::too_many_arguments)]

extern crate pretty_env_logger;
#[macro_use]
extern crate paris_log;
use std::{fs::read_to_string, path::PathBuf};

use indexmap::IndexMap;
use material_colors::theme::ThemeBuilder;
use serde_json::{Map, Value};

mod helpers;
pub mod template;
mod util;
mod wallpaper;

use crate::{
    cache::ImageCache,
    color::color::{get_source_color, Source},
    helpers::{color_entry, merge_json},
    parser::engine::EngineSyntax,
    scheme::{get_custom_color_schemes, get_schemes, SchemeTypes},
    util::{
        arguments::{FilterType, Format},
        color::format_palettes,
    },
};
use helpers::{set_wallpaper, setup_logging};
use template::TemplateFile;

use crate::{
    template::Template,
    util::{arguments::Cli, color::show_color, config::ConfigFile},
};

use clap::Parser;
use color_eyre::Report;

pub mod cache;
pub mod color;
pub mod exec;
pub mod filters;
pub mod parser;
pub mod scheme;
pub mod template_util;

use crate::{
    parser::Engine,
    scheme::{Schemes, SchemesEnum},
};

use material_colors::{color::Argb, theme::Theme};

pub struct State {
    pub args: Cli,
    pub config_file: ConfigFile,
    pub config_path: Option<PathBuf>,
    pub source_color: Option<Argb>,
    pub theme: Option<Theme>,
    pub schemes: Option<Schemes>,
    pub default_scheme: SchemesEnum,
    pub image_hash: ImageCache,
}

impl State {
    pub fn new(args: Cli) -> Self {
        let (config_file, config_path) =
            ConfigFile::read(&args).expect("Failed to read config file.");

        let source_color = match &args.source {
            Source::Json { path: _ } => None,
            _ => Some(
                (get_source_color(&args.source, &args.resize_filter))
                    .expect("Failed to get source color."),
            ),
        };

        let default_scheme = args
            .mode
            .expect("Something went wrong while parsing the mode");

        let image_hash = ImageCache::new(&args.source);

        let (schemes, theme) = match source_color {
            Some(color) => {
                let theme = ThemeBuilder::with_source(color).build();
                let (scheme_dark, scheme_light) = get_schemes(color, &args.r#type, &args.contrast);

                let mut schemes = get_custom_color_schemes(
                    color,
                    scheme_dark,
                    scheme_light,
                    &config_file.config.custom_colors,
                    &args.r#type,
                    &args.contrast,
                );

                schemes.dark.insert("source_color".to_owned(), color);
                schemes.light.insert("source_color".to_owned(), color);
                (Some(schemes), Some(theme))
            }
            None => (None, None),
        };

        Self {
            args,
            config_file,
            config_path,
            source_color,
            theme,
            schemes,
            default_scheme,
            image_hash,
        }
    }

    fn init_engine(&self) -> (Engine, Value) {
        let (schemes, default_scheme, mut json, loaded_cache) =
            if self.config_file.config.caching.unwrap_or(false) && self.args.source.is_image() {
                match self.image_hash.load() {
                    Ok((schemes, default_scheme)) => {
                        let json = self.get_render_data().unwrap();

                        (Some(schemes), default_scheme, json, true)
                    }
                    Err(_) => {
                        let json = self.get_render_data().unwrap();
                        (self.schemes.clone(), self.default_scheme, json, false)
                    }
                }
            } else {
                let json = self.get_render_data().unwrap();
                (self.schemes.clone(), self.default_scheme, json, false)
            };

        let mut engine = Engine::new();

        engine.set_syntax(self.get_syntax());

        self.add_engine_filters(&mut engine);

        let json = match &self.args.source {
            Source::Json { path } => {
                let string = read_to_string(&path).unwrap();

                let mut json = serde_json::from_str(&string).unwrap();

                if let Some(path) = &self.args.import_json {
                    let string = read_to_string(path).unwrap();

                    let json_file = serde_json::from_str(&string).unwrap();
                    merge_json(&mut json, json_file);
                }

                json
            }
            _ => {
                let schemes = schemes.unwrap();
                let palettes = &self.theme.as_ref().unwrap().palettes;

                let mut modified = IndexMap::new();
                for name in schemes.get_all_names() {
                    let dark_hex = schemes.dark.get(name).unwrap().to_hex_with_pound();
                    let light_hex = schemes.light.get(name).unwrap().to_hex_with_pound();
                    let default_hex = match default_scheme {
                        SchemesEnum::Dark => dark_hex.clone(),
                        SchemesEnum::Light => light_hex.clone(),
                    };

                    let mut schemes = Map::new();
                    schemes.insert("dark".to_string(), color_entry(dark_hex));
                    schemes.insert("light".to_string(), color_entry(light_hex));
                    schemes.insert("default".to_string(), color_entry(default_hex));
                    modified.insert(name.to_string(), Value::Object(schemes));
                }

                let palettes = format_palettes(palettes, &Format::Hex);

                let modified = serde_json::json!({
                    "colors": modified,
                    "palettes": palettes
                });

                merge_json(&mut json, modified);

                if let Some(path) = &self.args.import_json {
                    let string = read_to_string(path).unwrap();

                    let json_file = serde_json::from_str(&string).unwrap();
                    merge_json(&mut json, json_file);
                }

                json
            }
        };

        if self.config_file.config.caching.unwrap_or(false)
            && self.args.source.is_image()
            && !loaded_cache
        {
            self.save_cache(&mut json.clone())
                .expect("Failed saving cache");
        }

        engine.add_context(json.clone());

        (engine, json)
    }

    fn save_cache(&self, json: &Value) -> Result<(), Report> {
        let json_modified = serde_json::json!({
            "colors": {
                "dark": cache::convert_argb_scheme(&self.schemes.as_ref().unwrap().dark),
                "light": cache::convert_argb_scheme(&self.schemes.as_ref().unwrap().light),
            },
            "mode": self.default_scheme
        });

        self.image_hash.save(&json_modified)
    }

    pub fn get_render_data(&self) -> Result<serde_json::Value, Report> {
        let image = match &self.args.source {
            Source::Image { path } => Some(path),
            #[cfg(feature = "web-image")]
            Source::WebImage { .. } => None,
            Source::Color { .. } => None,
            Source::Json { path: _ } => None,
        };

        let is_dark_mode = match self.default_scheme {
            SchemesEnum::Dark => true,
            SchemesEnum::Light => false,
        };

        Ok(serde_json::json!({
            "image": image, "mode": self.default_scheme, "is_dark_mode": is_dark_mode,
        }))
    }

    fn add_engine_filters(&self, engine: &mut Engine) {
        // Colors
        engine.add_filter("set_red", crate::filters::set_red);
        engine.add_filter("set_green", crate::filters::set_green);
        engine.add_filter("set_blue", crate::filters::set_blue);
        engine.add_filter("set_alpha", crate::filters::set_alpha);

        engine.add_filter("set_hue", crate::filters::set_hue);
        engine.add_filter("set_saturation", crate::filters::set_saturation);
        engine.add_filter("set_lightness", crate::filters::set_lightness);

        engine.add_filter("lighten", crate::filters::lighten);
        engine.add_filter("auto_lightness", crate::filters::auto_lighten);
        engine.add_filter("saturate", crate::filters::saturate);

        engine.add_filter("grayscale", crate::filters::grayscale);
        engine.add_filter("invert", crate::filters::invert);

        engine.add_filter("blend", crate::filters::blend);
        engine.add_filter("harmonize", crate::filters::harmonize);
        engine.add_filter("to_color", crate::filters::to_color);

        // String
        engine.add_filter("lower_case", crate::filters::lower_case);
        engine.add_filter("camel_case", crate::filters::camel_case);
        engine.add_filter("pascal_case", crate::filters::pascal_case);
        engine.add_filter("snake_case", crate::filters::snake_case);
        engine.add_filter("kebab_case", crate::filters::kebab_case);
        engine.add_filter("replace", crate::filters::replace);
    }

    fn get_syntax(&self) -> EngineSyntax {
        let mut syntax = EngineSyntax::default();

        if let Some(bprefix) = &self.config_file.config.block_prefix {
            syntax.block_left = bprefix.clone();
        }
        if let Some(bpostfix) = &self.config_file.config.block_postfix {
            syntax.block_right = bpostfix.clone();
        }
        if let Some(eprefix) = &self.config_file.config.expr_prefix {
            syntax.keyword_left = eprefix.clone();
        }
        if let Some(epostfix) = &self.config_file.config.expr_postfix {
            syntax.keyword_right = epostfix.clone();
        }

        syntax
    }

    fn init_in_term(&self) -> Result<(), Report> {
        setup_logging(&self.args)?;

        #[cfg(feature = "update-informer")]
        if self.config_file.config.version_check == Some(true) {
            use crate::helpers::check_version;
            check_version();
        }

        Ok(())
    }

    pub fn run_in_term(&self) -> Result<(), Report> {
        self.init_in_term()?;

        if self.args.show_colors == Some(true) && !self.args.source.is_json() {
            show_color(
                &self.schemes.as_ref().unwrap(),
                &self.source_color.as_ref().unwrap(),
            );
        }

        let (mut engine, mut json_value) = self.init_engine();
        let mut template = TemplateFile::new(self, &mut engine);

        #[cfg(feature = "dump-json")]
        if let Some(ref format) = self.args.json {
            use crate::util::color::dump_json;
            if !self.args.include_image_in_json.unwrap_or(true) {
                if let Some(obj) = json_value.as_object_mut() {
                    obj.remove("image");
                };
            };
            dump_json(&mut json_value, format);
        }

        if self.args.dry_run == Some(true) {
            return Ok(());
        }

        // self.run_other_generator();
        template.generate()?;

        if let Some(_wallpaper_cfg) = &self.config_file.config.wallpaper {
            if _wallpaper_cfg.set.unwrap_or(true) {
                set_wallpaper(&self.args.source, _wallpaper_cfg)?;
            }
        }

        Ok(())
    }
}

#[allow(unreachable_code)]
fn main() -> Result<(), Report> {
    color_eyre::install()?;

    #[allow(unused_variables)]
    let default_args = Cli {
        source: crate::Source::Color(crate::color::color::ColorFormat::Hex {
            string: String::from("#ffffff"),
        }),
        r#type: Some(SchemeTypes::SchemeContent),
        config: None,
        prefix: None,
        contrast: Some(0.0),
        verbose: Some(true),
        quiet: None,
        debug: Some(true),
        mode: Some(SchemesEnum::Dark),
        dry_run: None,
        show_colors: None,
        json: None,
        import_json: None,
        include_image_in_json: Some(true),
        resize_filter: Some(FilterType::Triangle),
    };

    let args = Cli::parse();
    let prog = State::new(args.clone());
    prog.run_in_term()?;

    Ok(())
}
