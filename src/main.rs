#![allow(clippy::too_many_arguments)]

extern crate pretty_env_logger;
#[macro_use]
extern crate paris_log;
use std::path::PathBuf;

use material_colors::theme::ThemeBuilder;

mod helpers;
pub mod template;
mod util;
mod wallpaper;

use helpers::{set_wallpaper, setup_logging};
use matugen::{
    color::color::{get_source_color, Source},
    scheme::{get_custom_color_schemes, get_schemes, SchemeTypes},
    template_util::template::get_render_data,
};
use template::{build_engine_syntax, TemplateFile};

use crate::template::Template;
use crate::util::{arguments::Cli, color::show_color, config::ConfigFile};

use clap::Parser;
use color_eyre::Report;

use matugen::scheme::{Schemes, SchemesEnum};

use material_colors::{color::Argb, theme::Theme};
use matugen::engine::Engine;
use upon::Value;

pub struct State {
    pub args: Cli,
    pub config_file: ConfigFile,
    pub config_path: Option<PathBuf>,
    pub source_color: Argb,
    pub theme: Theme,
    pub schemes: Schemes,
    pub default_scheme: SchemesEnum,
}

impl State {
    pub fn new(args: Cli) -> Self {
        let (config_file, config_path) = ConfigFile::read(&args).unwrap();

        let source_color = get_source_color(&args.source).unwrap();
        let theme = ThemeBuilder::with_source(source_color).build();
        let (scheme_dark, scheme_light) = get_schemes(source_color, &args.r#type, &args.contrast);

        let default_scheme = args
            .mode
            .expect("Something went wrong while parsing the mode");

        let schemes = get_custom_color_schemes(
            source_color,
            scheme_dark,
            scheme_light,
            &config_file.config.custom_colors,
            &args.r#type,
            &args.contrast,
        );

        Self {
            args,
            config_file,
            config_path,
            source_color,
            theme,
            schemes,
            default_scheme,
        }
    }

    fn run_other_generator(&self) {
        let src = std::fs::read_to_string("test.test").unwrap();

        let engine = Engine::new(src, self.schemes.clone(), self.default_scheme);

        engine.generate_templates();
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
        let mut render_data = self.init_render_data()?;
        let template = TemplateFile::new(self, &mut engine, &mut render_data);

        self.run_other_generator();
        // template.generate()?;

        if let Some(_wallpaper_cfg) = &self.config_file.config.wallpaper {
            set_wallpaper(&self.args.source, _wallpaper_cfg)?;
        }

        Ok(())
    }

    fn init_engine(&self) -> upon::Engine {
        let syntax = build_engine_syntax(self);
        upon::Engine::with_syntax(syntax)
    }

    fn init_render_data(&self) -> Result<Value, Report> {
        let image = match &self.args.source {
            Source::Image { path } => Some(path),
            #[cfg(feature = "web-image")]
            Source::WebImage { .. } => None,
            Source::Color { .. } => None,
        };

        get_render_data(
            &self.schemes,
            &self.source_color,
            &self.default_scheme,
            &self.config_file.config.custom_keywords,
            image,
        )
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
        source: crate::Source::Color(matugen::color::color::ColorFormat::Hex {
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
