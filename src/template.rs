use color_eyre::eyre::ContextCompat;
use color_eyre::eyre::WrapErr;
use color_eyre::Help;
use color_eyre::{eyre::Result, Report};

use material_colors::color::Argb;
use matugen::template_util::template::add_engine_filters;
use matugen::template_util::template::get_render_data;
use matugen::template_util::template::render_template;
use serde::{Deserialize, Serialize};

use upon::Value;

use matugen::exec::hook::format_hook;

use std::path::Path;
use std::str;

use std::collections::HashMap;
use std::fs::create_dir_all;
use std::fs::read_to_string;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

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
    pub expr_prefix: Option<String>,
    pub expr_postfix: Option<String>,
    pub block_prefix: Option<String>,
    pub block_postfix: Option<String>,
}

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

        let image = match &source {
            Source::Image { path } => Some(path),
            #[cfg(feature = "web-image")]
            Source::WebImage { .. } => None,
            Source::Color { .. } => None,
        };

        let mut render_data = get_render_data(
            schemes,
            source_color,
            default_scheme,
            custom_keywords,
            image,
        )?;

        for (i, (name, template)) in templates.iter().enumerate() {
            let expr_prefix = template.expr_prefix.as_deref().unwrap_or("{{");
            let expr_postfix = template.expr_postfix.as_deref().unwrap_or("}}");
            let block_prefix = template.block_prefix.as_deref().unwrap_or("<*");
            let block_postfix = template.block_postfix.as_deref().unwrap_or("*>");

            let syntax = Syntax::builder()
                .expr(expr_prefix, expr_postfix)
                .block(block_prefix, block_postfix)
                .build();
            let mut engine = Engine::with_syntax(syntax);

            add_engine_filters(&mut engine);

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

fn create_missing_folders(output_path_absolute: &Path) -> Result<(), Report> {
    let parent_folder = &output_path_absolute
        .parent()
        .wrap_err("Could not get the parent of the output path.")?;
    if !parent_folder.exists() {
        error!(
            "The <b><yellow>{}</> folder doesnt exist, trying to create...",
            &parent_folder.display()
        );
        debug!("{}", parent_folder.display());
        let _ = create_dir_all(parent_folder).wrap_err(format!(
            "Failed to create the {} folders.",
            &output_path_absolute.display()
        ));
    };
    Ok(())
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
    let data = render_template(engine, name, render_data, input_path_absolute.to_str())?;

    let out = if path_prefix.is_some() && !cfg!(windows) {
        let mut prefix_path = PathBuf::from(path_prefix.as_ref().unwrap());

        // remove the root from the output_path so that we can push it onto the prefix
        let output_path = output_path_absolute
            .strip_prefix("/")
            .expect("output_path_absolute is not an absolute path.");

        prefix_path.push(output_path);

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
