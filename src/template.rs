use color_eyre::{
    eyre::{ContextCompat, Result, WrapErr},
    Help, Report,
};
use execute::{shell, Execute};
use serde_json::json;

use crate::{color::color::get_closest_color, parser::Engine as NewEngine};
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
    pub output_path: PathBuf,
    pub mode: Option<SchemesEnum>,
    pub colors_to_compare: Option<Vec<crate::color::color::ColorDefinition>>,
    pub compare_to: Option<String>,
    pub pre_hook: Option<String>,
    pub post_hook: Option<String>,
    pub input_path_modes: Option<InputPathModes>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputPathModes {
    pub light: PathBuf,
    pub dark: PathBuf,
}

pub struct TemplateFile<'a> {
    state: &'a State,
    engine: &'a mut NewEngine,
}

impl TemplateFile<'_> {
    pub fn new<'a>(state: &'a State, engine: &'a mut NewEngine) -> TemplateFile<'a> {
        TemplateFile { state, engine }
    }

    pub fn generate(&mut self) -> Result<(), Report> {
        info!(
            "Loaded <b><cyan>{}</> templates.",
            &self.state.config_file.templates.len()
        );

        let mut paths_hashmap: HashMap<String, (PathBuf, PathBuf)> = HashMap::new();

        for (_i, (name, template)) in self.state.config_file.templates.iter().enumerate() {
            let input_path = if let Some(input_path_mode) = &template.input_path_modes {
                match self.state.default_scheme {
                    SchemesEnum::Light => &input_path_mode.light,
                    SchemesEnum::Dark => &input_path_mode.dark,
                }
            } else {
                &template.input_path
            };

            let (input_path_absolute, output_path_absolute) =
                get_absolute_paths(&self.state.config_path, input_path, &template.output_path)?;

            if !input_path_absolute.exists() {
                warn!("<d>The <yellow><b>{}</><d> template in <u>{}</><d> doesnt exist, skipping...</>", name, input_path_absolute.display());
                continue;
            }

            let data = read_to_string(&input_path_absolute)
                .wrap_err(format!("Could not read the {} template.", name))
                .suggestion("Try converting the file to use UTF-8 encoding.")?;

            self.engine.add_template(name.to_string(), data);
            paths_hashmap.insert(
                name.to_string(),
                (input_path_absolute, output_path_absolute),
            );
        }

        for (i, (name, template)) in self.state.config_file.templates.iter().enumerate() {
            if let Some(hook) = &template.pre_hook {
                info!("Running pre_hook for the <b><cyan>{}</> template.", &name);
                format_hook(
                    self.engine,
                    name,
                    &hook,
                    &template.colors_to_compare,
                    &template.compare_to,
                )
                .wrap_err(format!("Failed to format the following hook:\n{}", hook))?;
            }

            let (input_path_absolute, output_path_absolute) = paths_hashmap
                .get(name)
                .wrap_err("Failed to get the input and output paths from hashmap")?;

            debug!(
                "Trying to write the {} template from {} to {}",
                name,
                input_path_absolute.display(),
                output_path_absolute.display()
            );

            self.export_template(name, output_path_absolute, input_path_absolute, i)?;

            if let Some(hook) = &template.post_hook {
                info!("Running post_hook for the <b><cyan>{}</> template.", &name);
                format_hook(
                    self.engine,
                    name,
                    &hook,
                    &template.colors_to_compare,
                    &template.compare_to,
                )
                .wrap_err(format!("Failed to format the following hook:\n{}", hook))?;
            }
        }
        Ok(())
    }

    fn export_template(
        &self,
        name: &String,
        output_path_absolute: &PathBuf,
        input_path_absolute: &PathBuf,
        i: usize,
    ) -> Result<(), Report> {
        let data = match self.engine.render(name) {
            Ok(v) => v,
            Err(errors) => {
                for err in errors {
                    err.emit(
                        self.engine.get_source(name),
                        &format!("{}", input_path_absolute.display()),
                    );
                }
                std::process::exit(1);
            }
        };

        let out = if self.state.args.prefix.is_some() && !cfg!(windows) {
            let mut prefix_path = PathBuf::from(self.state.args.prefix.as_ref().unwrap());

            let output_path = output_path_absolute
                .strip_prefix("/")
                .expect("output_path_absolute is not an absolute path.");

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
            "[{}/{}] Exported the <b><green>{}</> template to <d><u>{}</>",
            i + 1,
            &self.state.config_file.templates.len(),
            name,
            output_path_absolute.display()
        );

        Ok(())
    }
}

fn format_hook(
    engine: &mut NewEngine,
    template_name: &String,
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
                    err.emit(hook, &format!("{}-hook", template_name));
                }
                std::process::exit(1);
            }
        };
        let closest_color = get_closest_color(compare, &res);
        engine.add_context(json!({
            "closest_color": closest_color
        }));
    }

    let res = match engine.compile((&hook).to_string()) {
        Ok(v) => v,
        Err(errors) => {
            eprintln!("Error when formatting hook:\n{}", &hook);
            for err in errors {
                err.emit(hook, &format!("{}-hook", template_name));
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

fn get_absolute_paths(
    config_path: &Option<PathBuf>,
    input_path: &PathBuf,
    output_path: &PathBuf,
) -> Result<(PathBuf, PathBuf), Report> {
    let (input_path_absolute, output_path_absolute) = if config_path.is_some() {
        let base = std::fs::canonicalize(config_path.as_ref().unwrap())?;
        (
            input_path
                .try_resolve_in(&base)?
                .to_path_buf()
                .strip_canonicalization(),
            output_path
                .try_resolve_in(&base)?
                .to_path_buf()
                .strip_canonicalization(),
        )
    } else {
        (
            input_path.try_resolve()?.to_path_buf(),
            output_path.try_resolve()?.to_path_buf(),
        )
    };
    Ok((input_path_absolute, output_path_absolute))
}
