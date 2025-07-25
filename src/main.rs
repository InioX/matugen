#![allow(clippy::too_many_arguments)]

extern crate pretty_env_logger;
#[macro_use]
extern crate paris_log;
use std::{collections::HashMap, path::PathBuf};

use material_colors::theme::ThemeBuilder;
use serde_json::Value;

mod helpers;
pub mod template;
mod util;
mod wallpaper;

use crate::{
    cache::{convert_argb_scheme, ImageCache},
    color::color::{get_source_color, Source},
    parser::engine::EngineSyntax,
    scheme::{get_custom_color_schemes, get_schemes, SchemeTypes},
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
    pub source_color: Argb,
    pub theme: Theme,
    pub schemes: Schemes,
    pub default_scheme: SchemesEnum,
    pub image_hash: ImageCache,
}

impl State {
    pub fn new(args: Cli) -> Self {
        let (config_file, config_path) =
            ConfigFile::read(&args).expect("Failed to read config file.");

        let source_color = get_source_color(&args.source).expect("Failed to get source color.");
        let theme = ThemeBuilder::with_source(source_color).build();
        let (scheme_dark, scheme_light) = get_schemes(source_color, &args.r#type, &args.contrast);

        let default_scheme = args
            .mode
            .expect("Something went wrong while parsing the mode");

        let mut schemes = get_custom_color_schemes(
            source_color,
            scheme_dark,
            scheme_light,
            &config_file.config.custom_colors,
            &args.r#type,
            &args.contrast,
        );

        let image_hash = ImageCache::new(&args.source);

        schemes.dark.insert("source_color".to_owned(), source_color);
        schemes
            .light
            .insert("source_color".to_owned(), source_color);

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

    fn init_engine(&self) -> Engine {
        let (schemes, default_scheme, json, loaded_cache) =
            if self.config_file.config.caching.unwrap_or(false) && self.args.source.is_image() {
                match self.image_hash.load() {
                    Ok((schemes, default_scheme, json)) => (schemes, default_scheme, json, true),
                    Err(_) => {
                        let json = self.get_render_data().unwrap();
                        (self.schemes.clone(), self.default_scheme, json, false)
                    }
                }
            } else {
                let json = self.get_render_data().unwrap();
                (self.schemes.clone(), self.default_scheme, json, false)
            };

        let mut engine = Engine::new(schemes, default_scheme);

        engine.set_syntax(self.get_syntax());

        self.add_engine_filters(&mut engine);

        if self.config_file.config.caching.unwrap_or(false)
            && self.args.source.is_image()
            && !loaded_cache
        {
            self.save_cache(&json).expect("Failed saving cache");
        }

        engine.add_context(json);

        engine
    }

    fn save_cache(&self, json: &Value) -> Result<(), Report> {
        let json_modified = serde_json::json!({
            "colors": {
                "dark": convert_argb_scheme(&self.schemes.dark),
                "light": convert_argb_scheme(&self.schemes.light),
            }
        });

        let mut merged_json = json.clone();

        if let Value::Object(ref mut map) = merged_json {
            if let Value::Object(extra) = json_modified {
                for (k, v) in extra {
                    map.insert(k, v);
                }
            }
        }

        self.image_hash.save(&merged_json)
    }

    pub fn get_render_data(&self) -> Result<serde_json::Value, Report> {
        let image = match &self.args.source {
            Source::Image { path } => Some(path),
            #[cfg(feature = "web-image")]
            Source::WebImage { .. } => None,
            Source::Color { .. } => None,
        };

        let mut custom: HashMap<String, String> = Default::default();
        for entry in self.config_file.config.custom_keywords.iter() {
            for (name, value) in entry {
                custom.insert(name.to_string(), value.to_string());
            }
        }

        Ok(serde_json::json!({
            "image": image, "custom": &custom, "mode": self.default_scheme,
        }))
    }

    fn add_engine_filters(&self, engine: &mut Engine) {
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

        engine.add_filter("replace", crate::filters::replace);
    }

    fn get_syntax(&self) -> EngineSyntax {
        match (
            self.config_file.config.block_prefix,
            self.config_file.config.block_postfix,
            self.config_file.config.expr_prefix,
            self.config_file.config.expr_postfix,
        ) {
            (Some(bprefix), Some(pbostfix), Some(eprefix), Some(epostfix)) => {
                EngineSyntax::new(bprefix, pbostfix, eprefix, epostfix)
            }
            _ => EngineSyntax::default(),
        }
    }

    fn update_themes(&mut self) {
        let source_color = get_source_color(&self.args.source).unwrap();
        let _theme = ThemeBuilder::with_source(source_color).build();
        let (scheme_dark, scheme_light) =
            get_schemes(source_color, &self.args.r#type, &self.args.contrast);
        self.schemes = get_custom_color_schemes(
            source_color,
            scheme_dark,
            scheme_light,
            &self.config_file.config.custom_colors,
            &self.args.r#type,
            &self.args.contrast,
        );
    }

    #[cfg(feature = "ui")]
    fn run_gui(self) -> Result<(), Report> {
        setup_logging(&self.args)?;
        eframe::run_native(
            "Matugen",
            eframe::NativeOptions::default(),
            Box::new(|ctx| Ok(Box::new(MyApp::new(ctx, Box::new(self.args))))),
        )
        .unwrap();
        Ok(())
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

        if self.args.show_colors == Some(true) {
            show_color(&self.schemes, &self.source_color);
        }

        #[cfg(feature = "dump-json")]
        if let Some(ref format) = self.args.json {
            use crate::util::color::dump_json;
            dump_json(
                &self.schemes,
                &self.source_color,
                format,
                &self.theme.palettes,
            );
        }

        if self.args.dry_run == Some(true) {
            return Ok(());
        }

        let mut engine = self.init_engine();
        let mut template = TemplateFile::new(self, &mut engine);

        // self.run_other_generator();
        template.generate()?;

        if let Some(_wallpaper_cfg) = &self.config_file.config.wallpaper {
            set_wallpaper(&self.args.source, _wallpaper_cfg)?;
        }

        Ok(())
    }
}

#[cfg(feature = "ui")]
mod gui;

#[cfg(feature = "ui")]
use gui::init::MyApp;

#[allow(unreachable_code)]
fn main() -> Result<(), Report> {
    color_eyre::install()?;

    let args_unparsed: Vec<String> = std::env::args().collect();

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
    };

    if args_unparsed.len() > 1 && args_unparsed[1] == "ui" {
        #[cfg(feature = "ui")]
        {
            let prog = State::new(default_args);
            prog.run_gui()?;
            return Ok(());
        }

        error!("Tried to run gui mode without the <red>--ui</> compilation flag")
    } else {
        let args = Cli::parse();
        let prog = State::new(args.clone());
        prog.run_in_term()?;
    }

    Ok(())
}
