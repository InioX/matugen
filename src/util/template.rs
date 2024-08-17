use color_eyre::eyre::ContextCompat;
use color_eyre::eyre::WrapErr;
use color_eyre::Help;
use color_eyre::{eyre::Result, Report};

use colorsys::{ColorAlpha, Hsl};
use material_colors::color::Argb;
use matugen::filters::grayscale::grayscale;
use matugen::filters::hue::set_hue;
use matugen::filters::invert::invert;
use serde::{Deserialize, Serialize};

use upon::Value;

use matugen::exec::hook::format_hook;
use matugen::filters::{alpha::set_alpha, lightness::set_lightness};

use std::path::Path;
use std::str;

use std::collections::HashMap;
use std::fs::create_dir_all;
use std::fs::read_to_string;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use matugen::color::format::{
    format_hex, format_hex_stripped, format_hsl, format_hsla, format_rgb, format_rgba,
};

use matugen::color::color::Source;
use resolve_path::PathResolveExt;

use crate::{Schemes, SchemesEnum};

use upon::{Engine, Syntax};

#[derive(Serialize, Deserialize, Debug)]
pub struct Template {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub mode: Option<SchemesEnum>,
    pub colors_to_compare: Option<Vec<matugen::color::color::ColorDefinition>>,
    pub compare_to: Option<String>,
    pub pre_hook: Option<String>,
    pub post_hook: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Colora {
    hex: String,
    hex_stripped: String,
    rgb: String,
    rgba: String,
    hsl: String,
    hsla: String,
    red: String,
    green: String,
    blue: String,
    alpha: String,
    hue: String,
    saturation: String,
    lightness: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ColorVariants {
    pub light: Colora,
    pub dark: Colora,
    pub default: Colora,
}

use matugen::color::format::rgb_from_argb;

#[allow(clippy::manual_strip)]
pub trait StripCanonicalization
where
    Self: AsRef<Path>,
{
    #[cfg(not(target_os = "windows"))]
    fn strip_canonicalization(&self) -> PathBuf {
        self.as_ref().to_path_buf()
    }

    #[cfg(target_os = "windows")]
    fn strip_canonicalization(&self) -> PathBuf {
        const VERBATIM_PREFIX: &str = r#"\\?\"#;
        let p = self.as_ref().display().to_string();
        if p.starts_with(VERBATIM_PREFIX) {
            PathBuf::from(&p[VERBATIM_PREFIX.len()..])
        } else {
            self.as_ref().to_path_buf()
        }
    }
}

impl StripCanonicalization for PathBuf {}

impl Template {
    pub fn generate(
        schemes: &Schemes,
        templates: &HashMap<String, Template>,
        source: &Source,
        source_color: &Argb,
        default_scheme: &SchemesEnum,
        custom_keywords: &Option<HashMap<String, String>>,
        path_prefix: &Option<PathBuf>,
        config_path: Option<PathBuf>,
    ) -> Result<(), Report> {
        info!("Loaded <b><cyan>{}</> templates.", &templates.len());

        let syntax = Syntax::builder().expr("{{", "}}").block("<*", "*>").build();
        let mut engine = Engine::with_syntax(syntax);

        add_engine_filters(&mut engine);

        let image = match &source {
            Source::Image { path } => Some(path),
            Source::WebImage { .. } => None,
            Source::Color { .. } => None,
        };

        let colors = generate_colors(schemes, source_color, default_scheme)?;

        let mut custom: HashMap<String, String> = Default::default();

        for entry in custom_keywords.iter() {
            for (name, value) in entry {
                custom.insert(name.to_string(), value.to_string());
            }
        }

        let mut render_data = upon::value! {
            colors: &colors, image: image, custom: &custom,
        };

        for (i, (name, template)) in templates.iter().enumerate() {
            let (input_path_absolute, output_path_absolute) =
                get_absolute_paths(&config_path, template)?;

            if template.pre_hook.is_some() {
                format_hook(
                    &engine,
                    &mut render_data,
                    template.pre_hook.as_ref().unwrap(),
                    &template.colors_to_compare,
                    &template.compare_to,
                )
                .unwrap();
            }

            if !input_path_absolute.exists() {
                warn!("<d>The <yellow><b>{}</><d> template in <u>{}</><d> doesnt exist, skipping...</>", name, input_path_absolute.display());
                continue;
            }

            let data = read_to_string(&input_path_absolute)
                .wrap_err(format!("Could not read the {} template.", name))
                .suggestion("Try converting the file to use UTF-8 encoding.")?;

            engine.add_template(name, data).map_err(|error| {
                let message = format!(
                    "[{} - {}]\n{:#}",
                    name,
                    input_path_absolute.display(),
                    error
                );
                Report::new(error).wrap_err(message)
            })?;

            debug!(
                "Trying to write the {} template to {}",
                name,
                output_path_absolute.display()
            );

            export_template(
                &engine,
                name,
                &render_data,
                path_prefix,
                output_path_absolute,
                input_path_absolute,
                i,
                templates,
            )?;

            if template.post_hook.is_some() {
                format_hook(
                    &engine,
                    &mut render_data,
                    template.post_hook.as_ref().unwrap(),
                    &template.colors_to_compare,
                    &template.compare_to,
                )
                .unwrap();
            }
        }
        Ok(())
    }
}

fn create_missing_folders(output_path_absolute: &PathBuf) -> Result<(), Report> {
    let parent_folder = &output_path_absolute
        .parent()
        .wrap_err("Could not get the parent of the output path.")?;
    Ok(if !parent_folder.exists() {
        error!(
            "The <b><yellow>{}</> folder doesnt exist, trying to create...",
            &parent_folder.display()
        );
        debug!("{}", parent_folder.display());
        let _ = create_dir_all(parent_folder).wrap_err(format!(
            "Failed to create the {} folders.",
            &output_path_absolute.display()
        ));
    })
}

fn get_absolute_paths(
    config_path: &Option<PathBuf>,
    template: &Template,
) -> Result<(PathBuf, PathBuf), Report> {
    let (input_path_absolute, output_path_absolute) = if config_path.is_some() {
        let base = std::fs::canonicalize(config_path.as_ref().unwrap())?;
        (
            template
                .input_path
                .try_resolve_in(&base)?
                .to_path_buf()
                .strip_canonicalization(),
            template
                .output_path
                .try_resolve_in(&base)?
                .to_path_buf()
                .strip_canonicalization(),
        )
    } else {
        (
            template.input_path.try_resolve()?.to_path_buf(),
            template.output_path.try_resolve()?.to_path_buf(),
        )
    };
    Ok((input_path_absolute, output_path_absolute))
}

fn export_template(
    engine: &Engine,
    name: &String,
    render_data: &Value,
    path_prefix: &Option<PathBuf>,
    output_path_absolute: PathBuf,
    input_path_absolute: PathBuf,
    i: usize,
    templates: &HashMap<String, Template>,
) -> Result<(), Report> {
    let data = engine
        .template(name)
        .render(render_data)
        .to_string()
        .map_err(|error| {
            let message = format!(
                "[{} - {}]\n{:#}",
                name,
                input_path_absolute.display(),
                &error
            );

            Report::new(error).wrap_err(message)
        })?;

    let out = if path_prefix.is_some() && !cfg!(windows) {
        let mut prefix_path = PathBuf::from(path_prefix.as_ref().unwrap());

        // remove the root from the output_path so that we can push it onto the prefix
        let output_path = output_path_absolute
            .strip_prefix("/")
            .expect("output_path_absolute is not an absolute path.");

        prefix_path.push(&output_path);

        prefix_path
    } else {
        output_path_absolute.to_path_buf()
    };

    create_missing_folders(&out)?;

    debug!("out: {:?}", out);
    let mut output_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(out)?;

    if output_file.metadata()?.permissions().readonly() {
        error!(
            "The <b><red>{}</> file is Read-Only",
            &output_path_absolute.display()
        );
    }

    output_file.write_all(data.as_bytes())?;
    success!(
        "[{}/{}] Exported the <b><green>{}</> template to <d><u>{}</>",
        i + 1,
        &templates.len(),
        name,
        output_path_absolute.display()
    );

    Ok(())
}

fn add_engine_filters(engine: &mut Engine) {
    // Color manipulation
    engine.add_filter("set_lightness", set_lightness);
    engine.add_filter("set_alpha", set_alpha);
    engine.add_filter("set_hue", set_hue);
    engine.add_filter("grayscale", grayscale);
    engine.add_filter("invert", invert);

    // String manipulation
    engine.add_filter("to_upper", str::to_uppercase);
    engine.add_filter("to_lower", str::to_lowercase);
    engine.add_filter("replace", |s: String, from: String, to: String| {
        s.replace(&from, &to)
    });
}

fn generate_colors(
    schemes: &Schemes,
    source_color: &Argb,
    default_scheme: &SchemesEnum,
) -> Result<HashMap<String, ColorVariants>, Report> {
    let mut hashmap: HashMap<String, ColorVariants> = Default::default();
    for (field, _color) in &schemes.dark {
        hashmap.insert(
            field.to_string(),
            generate_single_color(field, schemes, source_color, default_scheme)?,
        );
    }
    hashmap.insert(
        String::from("source_color"),
        generate_single_color("source_color", schemes, source_color, default_scheme)?,
    );
    Ok(hashmap)
}

fn generate_single_color(
    field: &str,
    schemes: &Schemes,
    source_color: &Argb,
    default_scheme: &SchemesEnum,
) -> Result<ColorVariants, Report> {
    let scheme = match default_scheme {
        SchemesEnum::Light => &schemes.light,
        SchemesEnum::Dark => &schemes.dark,
    };

    if field == "source_color" {
        return Ok(ColorVariants {
            default: generate_color_strings(*source_color),
            light: generate_color_strings(*source_color),
            dark: generate_color_strings(*source_color),
        });
    }

    Ok(ColorVariants {
        default: generate_color_strings(scheme[field]),
        light: generate_color_strings(schemes.light[field]),
        dark: generate_color_strings(schemes.dark[field]),
    })
}

fn generate_color_strings(color: Argb) -> Colora {
    let base_color = rgb_from_argb(color);
    let hsl_color = Hsl::from(&base_color);
    Colora {
        hex: format_hex(&base_color),
        hex_stripped: format_hex_stripped(&base_color),
        rgb: format_rgb(&base_color),
        rgba: format_rgba(&base_color, true),
        hsl: format_hsl(&hsl_color),
        hsla: format_hsla(&hsl_color, true),
        red: format!("{:?}", base_color.red() as u8),
        green: format!("{:?}", base_color.green() as u8),
        blue: format!("{:?}", base_color.blue() as u8),
        alpha: format!("{:?}", base_color.alpha() as u8),
        hue: format!("{:?}", &hsl_color.hue()),
        lightness: format!("{:?}", &hsl_color.lightness()),
        saturation: format!("{:?}", &hsl_color.saturation()),
    }
}
