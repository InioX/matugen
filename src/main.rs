#![allow(clippy::too_many_arguments)]

extern crate pretty_env_logger;
#[macro_use]
extern crate paris_log;
use std::path::PathBuf;

use material_colors::theme::ThemeBuilder;
use serde_json::Value;

mod helpers;
pub mod template;
mod util;
mod wallpaper;

use crate::{
    cache::ImageCache,
    color::{base16::Backend, color::Source},
    helpers::{generate_schemes_and_theme, get_syntax, json_from_file, merge_json},
    scheme::SchemeTypes,
    template::get_absolute_path,
    util::{
        arguments::{FilterType, Format},
        color::{format_palettes, format_schemes},
    },
};
use helpers::{set_wallpaper, setup_logging};
use template::TemplateFile;

use crate::{
    template::Template,
    util::{arguments::Cli, color::show_color, config::ConfigFile},
};

use clap::Parser;
use color_eyre::{eyre::Context, Report, Section};

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
    pub base16: Option<Schemes>,
}

impl State {
    pub fn new(args: Cli) -> Result<Self, Report> {
        let (config_file, config_path) =
            ConfigFile::read(&args).wrap_err("Failed to read config file.")?;

        let image_cache = ImageCache::new(&args.source);

        let mut loaded_cache = false;

        let caching_enabled = config_file.config.caching.unwrap_or(false) && args.source.is_image();

        let default_scheme = args
            .mode
            .ok_or_else(|| Report::msg("Something went wrong while parsing the mode"))?;

        let (schemes, source_color, theme, base16) = if caching_enabled {
            match image_cache.load() {
                Ok((schemes, base16)) => {
                    // Source color will be the same in both light and dark mode
                    let source_color = *schemes.dark.clone().get("source_color").unwrap();

                    let theme = ThemeBuilder::with_source(source_color).build();

                    loaded_cache = true;

                    (Some(schemes), Some(source_color), Some(theme), Some(base16))
                }
                Err(e) => {
                    if !image_cache.exists() {
                        warn!(
                            "<d>The cache in <yellow><b>{}</><d> doesn't exist.</>",
                            image_cache.get_path().display()
                        );
                        generate_schemes_and_theme(&args, &config_file)?
                    } else {
                        return Err(e.wrap_err("Couldn't load the cache file").suggestion("You may need to regenerate your cache if coming from v3.1.0 and lower."));
                    }
                }
            }
        } else {
            generate_schemes_and_theme(&args, &config_file)?
        };

        Ok(Self {
            args,
            config_file,
            config_path,
            source_color,
            theme,
            schemes,
            default_scheme,
            image_hash: image_cache,
            loaded_cache,
            base16,
        })
    }

    fn init_engine(&self) -> Result<(Engine, Value), Report> {
        let mut json = self
            .get_render_data()
            .wrap_err("Could not get render data")?;

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
                if let Some(schemes) = &self.schemes {
                    let colors_md3 =
                        format_schemes(&schemes, self.default_scheme, schemes.get_all_names());
                    let json_md3 = serde_json::json!({"colors": serde_json::to_value(colors_md3).wrap_err("Could not format md3 colors to JSON")?});

                    merge_json(&mut json, json_md3);
                }

                if let Some(base16) = &self.base16 {
                    let colors_base16 =
                        format_schemes(&base16, self.default_scheme, base16.get_all_names());

                    let json_base16 = serde_json::json!({"base16": serde_json::to_value(colors_base16).wrap_err("Could not format base16 colors to JSON")?});

                    merge_json(&mut json, json_base16);
                }

                if let Some(theme) = &self.theme {
                    let palettes = format_palettes(&theme.palettes, &Format::Hex);

                    let json_palettes = serde_json::json!({"palettes": serde_json::to_value(palettes).wrap_err("Could not format palettes to JSON")?});

                    merge_json(&mut json, json_palettes);
                }

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

        Ok((engine, json))
    }

    fn save_cache(&self, _json: &Value) -> Result<(), Report> {
        let json_modified = serde_json::json!({
            "colors": {
                "dark": cache::convert_argb_scheme(&self.schemes.as_ref().unwrap().dark),
                "light": cache::convert_argb_scheme(&self.schemes.as_ref().unwrap().light),
            },
            "base16": {
                "dark": cache::convert_argb_scheme(&self.base16.as_ref().unwrap().dark),
                "light": cache::convert_argb_scheme(&self.base16.as_ref().unwrap().light),
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
            "image": image, "mode": format!("{}", self.default_scheme), "is_dark_mode": is_dark_mode,
        }))
    }

    fn add_engine_filters(&self, engine: &mut Engine) {
        register_filters!((engine) {
            "Colors" => {
                /// <p>Sets the red channel of a color</p>
                ///
                /// <p><strong>Arguments:</strong></p>
                ///
                /// <ul>
                ///     <li><code>Int</code> - the red channel value (0-255)</li>
                /// </ul>

                /// <p><strong>Example:</strong></p>
                /// <md-card class="code-card">
                ///     <pre class="code-block"><code class="language-bash">{{ "#000000" | to_color | set_red: 255 }}</code></pre>
                /// </md-card>
                "set_red" => crate::filters::set_red,

                /// <p>Sets the blue channel of a color</p>
                ///
                /// <p><strong>Arguments:</strong></p>
                ///
                /// <ul>
                ///     <li><code>Int</code> - the blue channel value</li>
                /// </ul>

                /// <p><strong>Example:</strong></p>
                /// <md-card class="code-card">
                ///     <pre class="code-block"><code class="language-bash">{{ "#000000" | to_color | set_blue: 255 }}</code></pre>
                /// </md-card>
                "set_blue" => crate::filters::set_blue,

                /// <p>Sets the green channel of a color</p>
                ///
                /// <p><strong>Arguments:</strong></p>
                ///
                /// <ul>
                ///     <li><code>Int</code> - the green channel value (0-255)</li>
                /// </ul>

                /// <p><strong>Example:</strong></p>
                /// <md-card class="code-card">
                ///     <pre class="code-block"><code class="language-bash">{{ "#000000" | to_color | set_green: 255 }}</code></pre>
                /// </md-card>
                "set_green" => crate::filters::set_green,

                /// <p>Sets the blue channel of a color</p>
                ///
                /// <p><strong>Arguments:</strong></p>
                ///
                /// <ul>
                ///     <li><code>Int</code> - the blue channel value (0-255)</li>
                /// </ul>

                /// <p><strong>Example:</strong></p>
                /// <md-card class="code-card">
                ///     <pre class="code-block"><code class="language-bash">{{ "#000000" | to_color | set_blue: 255 }}</code></pre>
                /// </md-card>
                "set_blue" => crate::filters::set_blue,

                /// <p>Sets the alpha channel of a color</p>
                ///
                /// <p><strong>Arguments:</strong></p>
                ///
                /// <ul>
                ///     <li><code>Float</code> - the alpha channel value (0.0-1.0)</li>
                /// </ul>

                /// <p><strong>Example:</strong></p>
                /// <md-card class="code-card">
                ///     <pre class="code-block"><code class="language-bash">{{ "#000000" | to_color | set_alpha: 0.1 }}</code></pre>
                /// </md-card>
                "set_alpha" => crate::filters::set_alpha,

                /// <p>Sets the hue channel of a color</p>
                ///
                /// <p><strong>Arguments:</strong></p>
                ///
                /// <ul>
                ///     <li><code>Int</code> - the hue value (0-360)</li>
                /// </ul>

                /// <p><strong>Example:</strong></p>
                /// <md-card class="code-card">
                ///     <pre class="code-block"><code class="language-bash">{{ "#000000" | to_color | set_hue: 360 }}</code></pre>
                /// </md-card>
                "set_hue" => crate::filters::set_hue,

                /// <p>Sets the saturation of a color</p>
                ///
                /// <p><strong>Arguments:</strong></p>
                ///
                /// <ul>
                ///     <li><code>Float</code> - the saturation value (0-100)</li>
                /// </ul>

                /// <p><strong>Example:</strong></p>
                /// <md-card class="code-card">
                ///     <pre class="code-block"><code class="language-bash">{{ "#000000" | to_color | set_saturation: 100.0 }}</code></pre>
                /// </md-card>
                "set_saturation" => crate::filters::set_saturation,

                /// <p>Sets the lightness of a color</p>
                ///
                /// <p><strong>Arguments:</strong></p>
                ///
                /// <ul>
                ///     <li><code>Int</code> - the lightness value (0-100)</li>
                /// </ul>

                /// <p><strong>Example:</strong></p>
                /// <md-card class="code-card">
                ///     <pre class="code-block"><code class="language-bash">{{ "#000000" | to_color | set_lightness: 100 }}</code></pre>
                /// </md-card>
                "set_lightness" => crate::filters::set_lightness,

                /// <p>Lightens a color</p>
                ///
                /// <p><strong>Arguments:</strong></p>
                ///
                /// <ul>
                ///     <li><code>Float</code> - amount to adjust lightness by</li>
                /// </ul>
                ///
                /// <p><strong>Example:</strong></p>
                /// <md-card class="code-card">
                ///     <pre class="code-block"><code class="language-bash">{{ "#ffffff" | to_color | lighten: 20.0 }}</code></pre>
                /// </md-card>
                "lighten" => crate::filters::lighten,

                /// <p>Parses a CSS color string into a color</p>
                ///
                /// <p><strong>Arguments:</strong></p>
                ///
                /// <ul>
                ///     <li>None</li>
                /// </ul>
                ///
                /// <p><strong>Example:</strong></p>
                /// <md-card class="code-card">
                ///     <pre class="code-block"><code class="language-bash">{{ "#ff00ff" | to_color }}</code></pre>
                /// </md-card>
                "to_color" => crate::filters::to_color,

                /// <p>Inverts a color</p>
                ///
                /// <p><strong>Arguments:</strong></p>
                ///
                /// <ul>
                ///     <li>None</li>
                /// </ul>
                ///
                /// <p><strong>Example:</strong></p>
                /// <md-card class="code-card">
                ///     <pre class="code-block"><code class="language-bash">{{ "#ffffff" | to_color | invert }}</code></pre>
                /// </md-card>
                "invert" => crate::filters::invert,

                /// <p>Converts a color to grayscale</p>
                ///
                /// <p><strong>Arguments:</strong></p>
                ///
                /// <ul>
                ///     <li>None</li>
                /// </ul>
                ///
                /// <p><strong>Example:</strong></p>
                /// <md-card class="code-card">
                ///     <pre class="code-block"><code class="language-bash">{{ "#ff0000" | to_color | grayscale }}</code></pre>
                /// </md-card>
                "grayscale" => crate::filters::grayscale,

                /// <p>Automatically lightens or darkens a color based on its current lightness</p>
                ///
                /// <p>If the color is dark, it will be lightened. If it is light, it will be darkened.</p>
                ///
                /// <p><strong>Arguments:</strong></p>
                ///
                /// <ul>
                ///     <li><code>Float</code> - amount to adjust lightness by</li>
                /// </ul>
                ///
                /// <p><strong>Example:</strong></p>
                /// <md-card class="code-card">
                ///     <pre class="code-block"><code class="language-bash">{{ "#222222" | to_color | auto_lightness: 10.0 }}</code></pre>
                /// </md-card>
                "auto_lightness" => crate::filters::auto_lighten,

                /// <p>Adjusts the saturation of a color</p>
                ///
                /// <p><strong>Arguments:</strong></p>
                ///
                /// <ul>
                ///     <li><code>Float</code> - saturation amount</li>
                ///     <li><code>String</code> - color space (<code>hsl</code> or <code>hsv</code>)</li>
                /// </ul>
                ///
                /// <p><strong>Example:</strong></p>
                /// <md-card class="code-card">
                ///     <pre class="code-block"><code class="language-bash">{{ "#336699" | to_color | saturate: 20.0, "hsl" }}</code></pre>
                /// </md-card>
                "saturate" => crate::filters::saturate,

                /// <p>Blends two colors together using hue blending</p>
                ///
                /// <p><strong>Arguments:</strong></p>
                ///
                /// <ul>
                ///     <li><code>Color</code> - color to blend with</li>
                ///     <li><code>Float</code> - blend amount (0.0 - 1.0)</li>
                /// </ul>
                ///
                /// <p><strong>Example:</strong></p>
                /// <md-card class="code-card">
                ///     <pre class="code-block"><code class="language-bash">{{ "#ff0000" | to_color | blend: {{ "#0000ff" | to_color }}, 0.5 }}</code></pre>
                /// </md-card>
                "blend" => crate::filters::blend,

                /// <p>Harmonizes a color with another using harmonization</p>
                ///
                /// <p>This shifts the hue of the original color toward the target color.</p>
                ///
                /// <p><strong>Arguments:</strong></p>
                ///
                /// <ul>
                ///     <li><code>Color</code> - color to harmonize with</li>
                /// </ul>
                ///
                /// <p><strong>Example:</strong></p>
                /// <md-card class="code-card">
                ///     <pre class="code-block"><code class="language-bash">{{ "#ff0000" | to_color | harmonize: {{ "#00ff00" | to_color }}</code></pre>
                /// </md-card>
                "harmonize" => crate::filters::harmonize,
            },

            "String" => {
                /// <p>Converts a value to snake_case</p>
                ///
                /// <p><strong>Arguments:</strong></p>
                ///
                /// <ul>
                ///     <li>None</li>
                /// </ul>
                ///
                /// <p><strong>Example:</strong></p>
                /// <md-card class="code-card">
                ///     <pre class="code-block"><code class="language-bash">{{ "Hello World" | snake_case }}</code></pre>
                /// </md-card>
                "snake_case" => crate::filters::snake_case,

                /// <p>Converts a value to lowercase</p>
                ///
                /// <p><strong>Arguments:</strong></p>
                ///
                /// <ul>
                ///     <li>None</li>
                /// </ul>
                ///
                /// <p><strong>Example:</strong></p>
                /// <md-card class="code-card">
                ///     <pre class="code-block"><code class="language-bash">{{ "Hello World" | lower_case }}</code></pre>
                /// </md-card>
                "lower_case" => crate::filters::lower_case,

                /// <p>Converts a value to camelCase</p>
                ///
                /// <p><strong>Arguments:</strong></p>
                ///
                /// <ul>
                ///     <li>None</li>
                /// </ul>
                ///
                /// <p><strong>Example:</strong></p>
                /// <md-card class="code-card">
                ///     <pre class="code-block"><code class="language-bash">{{ "hello world" | camel_case }}</code></pre>
                /// </md-card>
                "camel_case" => crate::filters::camel_case,

                /// <p>Converts a value to PascalCase</p>
                ///
                /// <p><strong>Arguments:</strong></p>
                ///
                /// <ul>
                ///     <li>None</li>
                /// </ul>
                ///
                /// <p><strong>Example:</strong></p>
                /// <md-card class="code-card">
                ///     <pre class="code-block"><code class="language-bash">{{ "hello world" | pascal_case }}</code></pre>
                /// </md-card>
                "pascal_case" => crate::filters::pascal_case,

                /// <p>Converts a value to kebab-case</p>
                ///
                /// <p><strong>Arguments:</strong></p>
                ///
                /// <ul>
                ///     <li>None</li>
                /// </ul>
                ///
                /// <p><strong>Example:</strong></p>
                /// <md-card class="code-card">
                ///     <pre class="code-block"><code class="language-bash">{{ "hello world" | kebab_case }}</code></pre>
                /// </md-card>
                "kebab_case" => crate::filters::kebab_case,

                /// <p>Replaces all occurrences of a substring</p>
                ///
                /// <p><strong>Arguments:</strong></p>
                ///
                /// <ul>
                ///     <li><code>String</code> - text to find</li>
                ///     <li><code>String</code> - replacement text</li>
                /// </ul>
                ///
                /// <p><strong>Example:</strong></p>
                /// <md-card class="code-card">
                ///     <pre class="code-block"><code class="language-bash">{{ "hello world" | replace: "world", "there" }}</code></pre>
                /// </md-card>
                "replace" => crate::filters::replace,

                /// <p>Formats a color into a certain format just like what using the .<format> on a color keyword would. This is useful for colors that are defined in the templates as there is no way to format them otherwise.</p>
                /// <md-card class="code-card">
                ///     <pre class="code-block"><code class="language-bash">{{ "#ff00ff" | format: "hex" }}</code></pre>
                /// </md-card>
                ///
                /// <p><strong>Arguments:</strong></p>
                ///
                /// <ul>
                ///     <li><code>String</code> - what to format the color into</li>
                /// </ul>
                ///
                /// <p><strong>Example:</strong></p>
                /// <md-card class="code-card">
                ///     <pre class="code-block"><code class="language-bash">{{ "#ff00ff" | to_color }}</code></pre>
                /// </md-card>
                "format" => crate::filters::format,
            },
        });
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
                self.schemes.as_ref(),
                self.source_color.as_ref(),
                self.base16.as_ref(),
            );
        }

        let (mut engine, mut json_value) = self
            .init_engine()
            .wrap_err("Something went wrong while initializing the engine")?;
        let mut template = TemplateFile::new(self, &mut engine);

        #[cfg(feature = "filter-docs")]
        {
            if self.args.filter_docs_html == Some(true) {
                {
                    use crate::parser::helpers::filters_to_html;
                    println!("{}", filters_to_html());
                    return Ok(());
                }
            }
        }

        #[cfg(feature = "dump-json")]
        if let Some(ref format) = self.args.json {
            use crate::util::color::dump_json;
            if !self.args.include_image_in_json.unwrap_or(true) {
                if let Some(obj) = json_value.as_object_mut() {
                    obj.remove("image");
                };
            };
            dump_json(&mut json_value, format, self.args.old_json_output);
        }

        if self.args.dry_run == Some(true) {
            return Ok(());
        }

        // self.run_other_generator();
        template.generate()?;

        if let Some(_wallpaper_cfg) = &self.config_file.config.wallpaper {
            if _wallpaper_cfg.set.unwrap_or(true) {
                set_wallpaper(&self.args.source, _wallpaper_cfg, &mut engine)?;
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
        fallback_color: None,
        prefer: None,
        old_json_output: Some(false),
        base16_backend: Some(Backend::Wal),
        #[cfg(feature = "filter-docs")]
        filter_docs_html: Some(false),
        lightness_dark: Some(0.0),
        lightness_light: Some(0.0),
        source_color_index: None,
    };

    let args = Cli::parse();

    setup_logging(&args)?;

    let state = State::new(args.clone())?;
    state.run_in_term()?;

    Ok(())
}
