use color_eyre::{
    eyre::{ContextCompat, Result, WrapErr},
    Help, Report,
};
use execute::{shell, Execute};
use serde_json::json;

use crate::{color::color::get_closest_color, helpers::get_syntax, parser::Engine};
use serde::{Deserialize, Serialize};

use std::{collections::HashMap, path::Path, process::Stdio, str};

use std::{
    fs::{create_dir_all, read_to_string, OpenOptions},
    io::Write,
    path::PathBuf,
};

use resolve_path::PathResolveExt;

use crate::{SchemesEnum, State};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Template {
    pub input_path: PathBuf,
    pub output_path: Option<OutputPath>,
    pub mode: Option<SchemesEnum>,
    pub colors_to_compare: Option<Vec<crate::color::color::ColorDefinition>>,
    pub compare_to: Option<String>,
    pub pre_hook: Option<String>,
    pub post_hook: Option<String>,
    pub input_path_modes: Option<InputPathModes>,
    pub expr_prefix: Option<String>,
    pub expr_postfix: Option<String>,
    pub block_prefix: Option<String>,
    pub block_postfix: Option<String>,
    pub index: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum OutputPath {
    Single(PathBuf),
    Multiple(Vec<PathBuf>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputPathModes {
    pub light: PathBuf,
    pub dark: PathBuf,
}

pub struct TemplateFile<'a> {
    state: &'a State,
    engine: &'a mut Engine,
}

impl TemplateFile<'_> {
    pub fn new<'a>(state: &'a State, engine: &'a mut Engine) -> TemplateFile<'a> {
        TemplateFile { state, engine }
    }

    pub fn generate(&mut self) -> Result<(), Report> {
        info!(
            "Loaded <b><cyan>{}</> templates.",
            &self.state.config_file.templates.len()
        );

        let mut paths_hashmap = HashMap::new();

        for (_i, (name, template)) in self.state.config_file.templates.iter().enumerate() {
            let input_path = if let Some(input_path_mode) = &template.input_path_modes {
                match self.state.default_scheme {
                    SchemesEnum::Light => &input_path_mode.light,
                    SchemesEnum::Dark => &input_path_mode.dark,
                }
            } else {
                &template.input_path
            };

            let (input_path_absolute, output_paths_absolute) =
                get_absolute_paths(&self.state.config_path, input_path, &template.output_path)?;

            if !input_path_absolute.exists() {
                warn!("<d>The <yellow><b>{}</><d> template in <u>{}</><d> doesn't exist, skipping...</>", name, input_path_absolute.display());
                continue;
            }

            let old_syntax = match (
                &template.block_prefix,
                &template.block_postfix,
                &template.expr_prefix,
                &template.expr_postfix,
            ) {
                (None, None, None, None) => None,
                _ => {
                    let old_syntax = self.engine.set_syntax(get_syntax(
                        template.block_prefix.as_ref(),
                        template.block_postfix.as_ref(),
                        template.expr_prefix.as_ref(),
                        template.expr_postfix.as_ref(),
                    ));
                    Some(old_syntax)
                }
            };

            let data = read_to_string(&input_path_absolute)
                .wrap_err(format!("Could not read the {} template.", name))
                .suggestion("Try converting the file to use UTF-8 encoding.")?;

            self.engine.add_template(name.to_string(), data);

            for output_path in output_paths_absolute {
                paths_hashmap.insert(
                    name.to_string(),
                    (input_path_absolute.to_path_buf(), output_path),
                );
            }

            if let Some(old) = old_syntax {
                self.engine.set_syntax(old);
            };
        }

        // Iterate over sorted templates when running command hooks
        let mut templates: Vec<(&String, &Template)> =
            self.state.config_file.templates.iter().collect();
        // Templates with an unspecified `index` default to 0
        templates.sort_by_key(|(_, Template { index, .. })| index.unwrap_or(0));

        for (name, template) in templates {
            if let Some(hook) = &template.pre_hook {
                info!("Running pre_hook for the <b><cyan>{}</> template.", &name);
                format_hook(
                    self.engine,
                    &hook,
                    &template.colors_to_compare,
                    &template.compare_to,
                )
                .wrap_err(format!("Failed to format the following hook:\n{}", hook))?;
            }

            if template.output_path.is_some() {
                let (input_path_absolute, output_path_absolute) = paths_hashmap
                    .get(name)
                    .wrap_err("Failed to get the input and output paths from hashmap")?;

                debug!(
                    "Trying to write the {} template from {} to {}",
                    name,
                    input_path_absolute.display(),
                    output_path_absolute.display()
                );

                self.export_template(name, output_path_absolute)?;
            }

            if let Some(hook) = &template.post_hook {
                info!("Running post_hook for the <b><cyan>{}</> template.", &name);
                format_hook(
                    self.engine,
                    &hook,
                    &template.colors_to_compare,
                    &template.compare_to,
                )
                .wrap_err(format!("Failed to format the following hook:\n{}", hook))?;
            }
        }

        Ok(())
    }

    fn export_template(&self, name: &String, output_path_absolute: &PathBuf) -> Result<(), Report> {
        let data = match self.engine.render(name) {
            Ok(v) => v,
            Err(errors) => {
                for err in errors {
                    err.emit(&self.engine)?;
                }

                if self.state.args.continue_on_error.unwrap_or(false) {
                    return Ok(());
                }

                std::process::exit(1);
            }
        };

        let out = if self.state.args.prefix.is_some() && !cfg!(windows) {
            let mut prefix_path = PathBuf::from(
                self.state
                    .args
                    .prefix
                    .as_ref()
                    .ok_or_else(|| Report::msg("Couldn't get the prefix path"))?,
            );

            let output_path = match output_path_absolute.strip_prefix("/") {
                Ok(v) => v,
                Err(e) => {
                    return Err(Report::msg(format!(
                        "Output path is not an absolute path: {}",
                        e
                    )))
                }
            };

            prefix_path.push(output_path);

            prefix_path
        } else {
            output_path_absolute.to_path_buf()
        };

        create_missing_folders(&out).wrap_err(format!(
            "Failed to create the missing folders for {}",
            &out.display()
        ))?;

        debug!("out: {:?}", out);

        if out.exists() {
            let meta = std::fs::metadata(&out).wrap_err(format!(
                "Failed to get file metadata for {}",
                &out.display()
            ))?;

            if meta.permissions().readonly() {
                error!(
                    "The <b><red>{}</> file is <b><red>Read-Only</>, not writing to it.",
                    &output_path_absolute.display()
                );
                return Ok(());
            }
        }

        let mut output_file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(out)?;

        output_file.write_all(data.as_bytes())?;

        success!(
            "Exported the <b><green>{}</> template to <d><u>{}</>",
            name,
            output_path_absolute.display()
        );

        Ok(())
    }
}

pub fn format_hook(
    engine: &mut Engine,
    hook: &String,
    colors_to_compare: &Option<Vec<crate::color::color::ColorDefinition>>,
    compare_to: &Option<String>,
) -> Result<(), Report> {
    if let (Some(compare), Some(to)) = (colors_to_compare, compare_to) {
        let res = match engine.compile(to.to_string()) {
            Ok(v) => v,
            Err(errors) => {
                eprintln!("Error when formatting hook:\n{}", &hook);
                for err in errors {
                    err.emit(&engine)?;
                }
                std::process::exit(1);
            }
        };
        let closest_color = get_closest_color(compare, &res)?;
        engine.add_context(json!({
            "closest_color": closest_color
        }));
    }

    let res = match engine.compile((&hook).to_string()) {
        Ok(v) => v,
        Err(errors) => {
            eprintln!("Error when formatting hook:\n{}", &hook);
            for err in errors {
                err.emit(&engine)?;
            }
            std::process::exit(1);
        }
    };

    let mut command = shell(&res);

    command.stdout(Stdio::inherit());

    let output = command.execute_output()?;

    if let Some(exit_code) = output.status.code() {
        if exit_code != 0 {
            error!("Failed executing command: {:?}", &res)
        }
    } else {
        eprintln!("Interrupted!");
    }

    Ok(())
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
        create_dir_all(parent_folder)?;
    };
    Ok(())
}

pub fn get_absolute_path(base_path: &PathBuf, relative_path: &PathBuf) -> Result<PathBuf, Report> {
    let base = std::fs::canonicalize(base_path)?;
    let absolute = relative_path
        .try_resolve_in(&base)?
        .to_path_buf()
        .strip_canonicalization();
    Ok(absolute)
}

fn get_absolute_paths(
    config_path: &Option<PathBuf>,
    input_path: &PathBuf,
    output_paths: &Option<OutputPath>,
) -> Result<(PathBuf, Vec<PathBuf>), Report> {
    let output_paths = match output_paths {
        Some(OutputPath::Single(path)) => &Vec::from([path.to_path_buf()]),
        Some(OutputPath::Multiple(paths)) => paths,
        None => &Vec::new(),
    };

    let (input_path_absolute, output_paths_absolute) = if let Some(p) = config_path {
        let base = std::fs::canonicalize(p)?;
        let mut paths = Vec::new();

        for output_path in output_paths {
            paths.push(
                output_path
                    .try_resolve_in(&base)?
                    .to_path_buf()
                    .strip_canonicalization(),
            );
        }

        (
            input_path
                .try_resolve_in(&base)?
                .to_path_buf()
                .strip_canonicalization(),
            paths,
        )
    } else {
        let mut paths = Vec::new();

        for output_path in output_paths {
            paths.push(output_path.try_resolve()?.to_path_buf());
        }

        (input_path.try_resolve()?.to_path_buf(), paths)
    };
    Ok((input_path_absolute, output_paths_absolute))
}
