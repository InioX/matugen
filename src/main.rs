#![allow(clippy::too_many_arguments)]

extern crate pretty_env_logger;
#[macro_use]
extern crate paris_log;
use std::path::PathBuf;

use indexmap::IndexMap;
use material_colors::theme::ThemeBuilder;
use serde_json::{Map, Value};

mod helpers;
pub mod template;
mod util;
mod wallpaper;

use crate::{
    cache::ImageCache,
    color::color::Source,
    helpers::{color_entry, generate_schemes_and_theme, get_syntax, json_from_file, merge_json},
    scheme::SchemeTypes,
    template::get_absolute_path,
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
use color_eyre::{eyre::Context, Report};

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
    pub loaded_cache: bool,
}

impl State {
    pub fn new(args: Cli) -> Result<Self, Report> {
        let (config_file, config_path) =
            ConfigFile::read(&args).wrap_err("Failed to read config file.")?;

        let image_hash = ImageCache::new(&args.source);

        let mut loaded_cache = false;

        let caching_enabled = config_file.config.caching.unwrap_or(false) && args.source.is_image();

        let default_scheme = args
            .mode
            .ok_or_else(|| Report::msg("Something went wrong while parsing the mode"))?;

        let (schemes, source_color, theme) = if caching_enabled {
            match image_hash.load() {
                Ok(schemes) => {
                    // Source color will be the same in both light and dark mode
                    let source_color = *schemes.dark.clone().get("source_color").unwrap();

                    let theme = ThemeBuilder::with_source(source_color).build();

                    loaded_cache = true;

                    (Some(schemes), Some(source_color), Some(theme))
                }
                Err(e) => return Err(e.wrap_err("Couldn't load the cache file")),
            }
        } else {
            generate_schemes_and_theme(&args, &config_file)
        };

        Ok(Self {
            args,
            config_file,
            config_path,
            source_color,
            theme,
            schemes,
            default_scheme,
            image_hash,
            loaded_cache,
        })
    }

    fn init_engine(&self) -> (Engine, Value) {
        let mut json = self.get_render_data().unwrap();

        let mut engine = Engine::new();

        engine.set_syntax(get_syntax(
            self.config_file.config.block_prefix.as_ref(),
            self.config_file.config.block_postfix.as_ref(),
            self.config_file.config.expr_prefix.as_ref(),
            self.config_file.config.expr_postfix.as_ref(),
        ));

        self.add_engine_filters(&mut engine);

        let mut json = match &self.args.source {
            Source::Json { path } => json_from_file(&PathBuf::from(path)).unwrap(),
            _ => {
                let schemes = self.schemes.as_ref().unwrap();
                let palettes = &self.theme.as_ref().unwrap().palettes;

                let mut modified = IndexMap::new();
                for name in schemes.get_all_names() {
                    let dark_hex = schemes.dark.get(name).unwrap().to_hex_with_pound();
                    let light_hex = schemes.light.get(name).unwrap().to_hex_with_pound();
                    let default_hex = match self.default_scheme {
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

                json
            }
        };

        if let Some(paths) = &self.args.import_json {
            for path in paths {
                let json2 = json_from_file(&PathBuf::from(path)).unwrap();
                merge_json(&mut json, json2);
            }
        }

        if let Some(strings) = &self.args.import_json_string {
            for string in strings {
                let json2 =
                    serde_json::from_str(&string).expect("Failed to parse JSON from string.");
                merge_json(&mut json, json2);
            }
        }

        if let (Some(paths), Some(config_path)) = (
            &self.config_file.config.import_json_files,
            &self.config_path,
        ) {
            for path in paths {
                let absolute = get_absolute_path(config_path, path).unwrap();

                let json2 = json_from_file(&absolute).unwrap();

                merge_json(&mut json, json2);
            }
        }

        if self.config_file.config.caching.unwrap_or(false)
            && self.args.source.is_image()
            && !self.loaded_cache
        {
            self.save_cache(&mut json.clone())
                .expect("Failed saving cache");
        }

        engine.add_context(json.clone());

        (engine, json)
    }

    fn save_cache(&self, _json: &Value) -> Result<(), Report> {
        let json_modified = serde_json::json!({
            "colors": {
                "dark": cache::convert_argb_scheme(&self.schemes.as_ref().unwrap().dark),
                "light": cache::convert_argb_scheme(&self.schemes.as_ref().unwrap().light),
            },
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

    fn init_in_term(&self) -> Result<(), Report> {
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
        import_json_string: None,
        include_image_in_json: Some(true),
        resize_filter: Some(FilterType::Triangle),
        continue_on_error: Some(false),
    };

    let args = Cli::parse();

    setup_logging(&args)?;

    let state = State::new(args.clone())?;
    state.run_in_term()?;

    Ok(())
}
