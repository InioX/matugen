use color_eyre::eyre::ContextCompat;
use color_eyre::eyre::WrapErr;
use color_eyre::Help;
use color_eyre::{eyre::Result, Report};

use crate::parser::Engine as NewEngine;
// use matugen::template_util::template::add_engine_filters;
use serde::{Deserialize, Serialize};

use std::path::Path;
use std::str;

use std::fs::create_dir_all;
use std::fs::read_to_string;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use resolve_path::PathResolveExt;

use crate::SchemesEnum;
use crate::State;

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

    pub fn generate_new(&mut self) -> Result<(), Report> {
        info!(
            "Loaded <b><cyan>{}</> templates.",
            &self.state.config_file.templates.len()
        );

        for (i, (name, template)) in self.state.config_file.templates.iter().enumerate() {
            // add_engine_filters(self.engine);

            let input_path = if let Some(input_path_mode) = &template.input_path_modes {
                match self.state.default_scheme {
                    SchemesEnum::Light => &input_path_mode.light,
                    SchemesEnum::Dark => &input_path_mode.dark,
                }
            } else {
                &template.input_path
            };

            let (input_path_absolute, output_path_absolute) = get_absolute_paths(
                &self.state.config_path,
                template,
                input_path,
                &template.output_path,
            )?;

            if !input_path_absolute.exists() {
                warn!("<d>The <yellow><b>{}</><d> template in <u>{}</><d> doesnt exist, skipping...</>", name, input_path_absolute.display());
                continue;
            }

            let data = read_to_string(&input_path_absolute)
                .wrap_err(format!("Could not read the {} template.", name))
                .suggestion("Try converting the file to use UTF-8 encoding.")?;

            self.engine.add_template(name.to_string(), data);

            debug!(
                "Trying to write the {} template from {} to {}",
                name,
                input_path.display(),
                output_path_absolute.display()
            );

            self.export_template(name, output_path_absolute, input_path_absolute, i)?;
        }
        Ok(())
    }

    // pub fn generate(&mut self) -> Result<(), Report> {
    //     info!(
    //         "Loaded <b><cyan>{}</> templates.",
    //         &self.state.config_file.templates.len()
    //     );

    //     for (i, (name, template)) in self.state.config_file.templates.iter().enumerate() {
    //         // add_engine_filters(self.engine);

    //         let input_path = if let Some(input_path_mode) = &template.input_path_modes {
    //             match self.state.default_scheme {
    //                 SchemesEnum::Light => &input_path_mode.light,
    //                 SchemesEnum::Dark => &input_path_mode.dark,
    //             }
    //         } else {
    //             &template.input_path
    //         };

    //         let (input_path_absolute, output_path_absolute) = get_absolute_paths(
    //             &self.state.config_path,
    //             template,
    //             input_path,
    //             &template.output_path,
    //         )?;

    //         if template.pre_hook.is_some() {
    //             format_hook(
    //                 self.engine,
    //                 self.render_data,
    //                 template.pre_hook.as_ref().unwrap(),
    //                 &template.colors_to_compare,
    //                 &template.compare_to,
    //             )
    //             .unwrap();
    //         }

    //         if !input_path_absolute.exists() {
    //             warn!("<d>The <yellow><b>{}</><d> template in <u>{}</><d> doesnt exist, skipping...</>", name, input_path_absolute.display());
    //             continue;
    //         }

    //         let data = read_to_string(&input_path_absolute)
    //             .wrap_err(format!("Could not read the {} template.", name))
    //             .suggestion("Try converting the file to use UTF-8 encoding.")?;

    //         self.engine.add_template(name, data).map_err(|error| {
    //             let message = format!(
    //                 "[{} - {}]\n{:#}",
    //                 name,
    //                 input_path_absolute.display(),
    //                 error
    //             );
    //             Report::new(error).wrap_err(message)
    //         })?;

    //         debug!(
    //             "Trying to write the {} template from {} to {}",
    //             name,
    //             input_path.display(),
    //             output_path_absolute.display()
    //         );

    //         self.export_template(
    //             name,
    //             self.render_data,
    //             output_path_absolute,
    //             input_path_absolute,
    //             i,
    //         )?;

    //         if template.post_hook.is_some() {
    //             format_hook(
    //                 self.engine,
    //                 self.render_data,
    //                 template.post_hook.as_ref().unwrap(),
    //                 &template.colors_to_compare,
    //                 &template.compare_to,
    //             )
    //             .unwrap();
    //         }
    //     }
    //     Ok(())
    // }

    fn export_template(
        &self,
        name: &String,
        output_path_absolute: PathBuf,
        input_path_absolute: PathBuf,
        i: usize,
    ) -> Result<(), Report> {
        let data = match self.engine.render(name) {
            Ok(v) => v,
            Err(errors) => {
                for err in errors {
                    err.emit(&self.engine.get_source(name));
                }
                std::process::exit(1);
            }
        };

        let out = if self.state.args.prefix.is_some() && !cfg!(windows) {
            let mut prefix_path = PathBuf::from(self.state.args.prefix.as_ref().unwrap());

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
            &self.state.config_file.templates.len(),
            name,
            output_path_absolute.display()
        );

        Ok(())
    }
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
