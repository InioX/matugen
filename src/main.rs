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
    scheme::{get_custom_color_schemes, get_schemes},
    template_util::template::get_render_data,
};
use template::{build_engine_syntax, TemplateFile};

use crate::template::Template;
use crate::util::{arguments::Cli, color::show_color, config::ConfigFile};

use clap::Parser;
use color_eyre::{eyre::Result, Report};

use matugen::scheme::{Schemes, SchemesEnum};

use material_colors::{color::Argb, theme::Theme};
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

    fn init_in_term(&self) -> Result<(), Report> {
        color_eyre::install()?;
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
        let mut template = TemplateFile::new(self, &mut engine, &mut render_data);

        template.generate()?;

        // Template::generate(
        //     &self.schemes,
        //     &self.config_file.templates,
        //     &self.args.source,
        //     &self.source_color,
        //     &self.default_scheme,
        //     &self.config_file.config.custom_keywords,
        //     &self.args.prefix,
        //     &self.config_path,
        // )?;

        if let Some(_wallpaper_cfg) = &self.config_file.config.wallpaper {
            set_wallpaper(&self.args.source)?;
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

fn main() -> Result<(), Report> {
    let args = Cli::parse();

    let prog = State::new(args.clone());

    prog.run_in_term()
}
