use std::path::PathBuf;

#[cfg(feature = "ui")]
use egui::Context;
#[cfg(feature = "ui")]
use indexmap::IndexMap;

use crate::{
    color::{
        color::Source,
        format::{format_hex, rgb_from_argb},
    },
    scheme::SchemeTypes,
    template::TemplateFile,
    util::arguments::Cli,
    State,
};
#[cfg(feature = "ui")]
use eframe::egui;
use egui::{Color32, Stroke, Ui, Vec2, Visuals};
use material_colors::color::Argb;
use serde::{Deserialize, Serialize};

use super::cache::{read_cache, save_cache};

#[cfg(feature = "ui")]
#[derive(PartialEq, Serialize, Deserialize, Clone)]
pub enum Tabs {
    Images,
    Settings,
    Colors,
}

const COLOR_RECT_SIZE: [f32; 2] = [20., 20.];

#[cfg(feature = "ui")]
pub struct MyApp {
    selected_file: Option<PathBuf>,
    cli: Box<Cli>,
    colors: ColorsMap,
    selected_tab: Tabs,
    app: State,
    images_vec: Vec<PathBuf>,
    image_folder: Option<PathBuf>,
    load_cache: bool,
    ran_once: bool,
}

#[cfg(feature = "ui")]
pub struct ColorsMap {
    pub light: IndexMap<String, Color32>,
    pub dark: IndexMap<String, Color32>,
}

impl Default for ColorsMap {
    fn default() -> Self {
        use material_colors::theme::ThemeBuilder;

        let theme = ThemeBuilder::with_source(Argb::new(255, 66, 133, 244)).build();

        let (scheme_dark, scheme_light) = (theme.schemes.dark, theme.schemes.light);

        let mut dark: IndexMap<String, Color32> = IndexMap::new();
        let mut light: IndexMap<String, Color32> = IndexMap::new();

        for (name, color) in scheme_dark {
            dark.insert(name.to_string(), argb_to_color32(&color));
        }
        for (name, color) in scheme_light {
            light.insert(name.to_string(), argb_to_color32(&color));
        }

        Self { light, dark }
    }
}

fn get_images_in_folder(folder_path: &PathBuf) -> Vec<PathBuf> {
    let valid_extensions = ["jpg", "jpeg", "png", "gif", "bmp", "tiff", "webp"];

    std::fs::read_dir(folder_path)
        .ok()
        .into_iter()
        .flatten()
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .filter(|path| {
            path.extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| valid_extensions.contains(&ext.to_lowercase().as_str()))
                .unwrap_or(false)
        })
        .collect()
}

#[cfg(feature = "ui")]
impl MyApp {
    pub fn new(_cc: &eframe::CreationContext, cli: Box<Cli>) -> Self {
        let mut is_cache = false;
        let mut selected_tab = Tabs::Settings;
        let mut image_folder = None;
        if let Some(c) = read_cache() {
            is_cache = true;
            selected_tab = c.selected_tab;
            image_folder = Some(c.image_folder);
        } else {
        };
        Self {
            selected_file: None,
            app: crate::State::new(*cli.clone()),
            cli,
            colors: ColorsMap::default(),
            selected_tab,
            images_vec: vec![],
            load_cache: if is_cache { true } else { false },
            image_folder,
            ran_once: false,
        }
    }
    fn body(&mut self, ui: &mut Ui, ctx: &Context) {
        if self.load_cache {
            self.update_images_tab(ui);
            self.load_cache = false;
        }
        match self.selected_tab {
            Tabs::Settings => self.settings(ui),
            Tabs::Colors => self.colors(ui),
            Tabs::Images => self.images(ui, ctx),
        }
    }

    fn settings(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Contrast");
            ui.add(egui::Slider::new(
                &mut self.app.args.contrast.unwrap(),
                -1.0..=1.0,
            ));
        });
        ui.label("Scheme type");
        egui::ComboBox::from_label("")
            .selected_text(format!("{:?}", self.app.args.r#type.unwrap()))
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut self.app.args.r#type,
                    Some(SchemeTypes::SchemeContent),
                    "SchemeContent",
                );
                ui.selectable_value(
                    &mut self.app.args.r#type,
                    Some(SchemeTypes::SchemeExpressive),
                    "SchemeExpressive",
                );
                ui.selectable_value(
                    &mut self.app.args.r#type,
                    Some(SchemeTypes::SchemeFidelity),
                    "SchemeFidelity",
                );
                ui.selectable_value(
                    &mut self.app.args.r#type,
                    Some(SchemeTypes::SchemeFruitSalad),
                    "SchemeFruitSalad",
                );
                ui.selectable_value(
                    &mut self.app.args.r#type,
                    Some(SchemeTypes::SchemeMonochrome),
                    "SchemeMonochrome",
                );
                ui.selectable_value(
                    &mut self.app.args.r#type,
                    Some(SchemeTypes::SchemeNeutral),
                    "SchemeNeutral",
                );
                ui.selectable_value(
                    &mut self.app.args.r#type,
                    Some(SchemeTypes::SchemeRainbow),
                    "SchemeRainbow",
                );
                ui.selectable_value(
                    &mut self.app.args.r#type,
                    Some(SchemeTypes::SchemeTonalSpot),
                    "SchemeTonalSpot",
                );
                ui.selectable_value(
                    &mut self.app.args.r#type,
                    Some(SchemeTypes::SchemeVibrant),
                    "SchemeVibrant",
                );
            });
    }

    fn show_single_color_scrollable(
        &self,
        ui: &mut Ui,
        text: &str,
        colors: &IndexMap<String, Color32>,
    ) {
        ui.vertical(|ui| {
            ui.label(egui::RichText::new(text).strong());
            for (name, color) in colors {
                let hex_label = format_hex(&rgb_from_argb(Argb {
                    alpha: color.a(),
                    red: color.r(),
                    green: color.g(),
                    blue: color.b(),
                }));
                ui.horizontal(|ui| {
                    ui.label(name);
                    egui::widgets::color_picker::show_color(
                        ui,
                        *color,
                        Vec2::new(COLOR_RECT_SIZE[0], COLOR_RECT_SIZE[1]),
                    );
                    ui.label(hex_label);
                });
            }
        });
    }

    fn show_colors(&mut self, ui: &mut Ui) {
        self.show_single_color_scrollable(ui, "DARK", &self.colors.dark);

        self.show_single_color_scrollable(ui, "LIGHT", &self.colors.light);
    }

    fn colors(&mut self, ui: &mut Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.horizontal(|ui| {
                self.show_colors(ui);
            });
        });
    }

    fn images(&mut self, ui: &mut Ui, ctx: &Context) {
        if self.images_vec.is_empty() {
            ui.label("No image folder selected or no images to show.");
        } else {
            ui.with_layout(
                egui::Layout::left_to_right(egui::Align::Max).with_cross_justify(true),
                |ui| {
                    self.show_images(ui, ctx);
                },
            );
        }
    }

    fn show_images(&mut self, ui: &mut Ui, ctx: &Context) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("image_grid")
                .num_columns(3)
                .spacing([10.0, 10.0])
                .show(ui, |ui| {
                    for (i, path) in self.images_vec.clone().iter().enumerate() {
                        let path_str = path.clone().into_os_string().into_string().unwrap();
                        let img_widget = egui::Image::from_uri(format!("file://{}", &path_str))
                            .max_height(200.0)
                            .max_width(200.0)
                            .maintain_aspect_ratio(true)
                            .fit_to_original_size(1.0);

                        ui.vertical_centered(|ui| {
                            if ui.add(img_widget.sense(egui::Sense::click())).clicked() {
                                self.selected_file = Some(path.to_path_buf());
                                self.run();
                                self.apply_app_theme(ctx);
                            }

                            ui.label(format!("{}", path_str));
                        });

                        if (i + 1) % 3 == 0 {
                            ui.end_row();
                        }
                    }
                });
        });
    }

    fn update_images_tab(&mut self, ui: &mut Ui) {
        if self.image_folder == Some("".into()) || self.image_folder.is_none() {
            return;
        }
        save_cache(
            self.image_folder.clone().unwrap(),
            self.selected_tab.clone(),
        );
        let images = get_images_in_folder(&self.image_folder.as_ref().unwrap());
        self.images_vec = images;
    }

    fn top_buttons(&mut self, ui: &mut Ui, ctx: &Context) {
        if ui.button("Image Folder").clicked() {
            if let Some(path) = rfd::FileDialog::new().pick_folder() {
                self.image_folder = Some(path);
                self.update_images_tab(ui);
            }
        }
        if ui.button("Select image").clicked() {
            if let Some(path) = rfd::FileDialog::new().pick_file() {
                self.selected_file = Some(path);
            }
        }
        if ui.button("Run").clicked() {
            self.run();
            self.apply_app_theme(ctx);
        }
    }

    fn run(&mut self) {
        if self.selected_file == Some("".into()) || self.selected_file.is_none() {
            return;
        };
        self.generate_tempalates();
        self.update_colors_tab();
        self.ran_once = true;
    }

    fn generate_tempalates(&mut self) {
        let mut engine = self.app.init_engine();
        let mut template = TemplateFile::new(&self.app, &mut engine);
        template.generate().unwrap();
    }

    fn update_colors_tab(&mut self) {
        self.app.args.source = Source::Image {
            path: self
                .selected_file
                .clone()
                .unwrap()
                .into_os_string()
                .into_string()
                .unwrap(),
        };
        self.app.update_themes();
        let mut dark: IndexMap<String, Color32> = IndexMap::new();
        let mut light: IndexMap<String, Color32> = IndexMap::new();
        for (name, color) in &self.app.schemes.dark {
            dark.insert(name.to_string(), argb_to_color32(color));
        }
        for (name, color) in &self.app.schemes.light {
            light.insert(name.to_string(), argb_to_color32(color));
        }
        self.colors.dark = dark;
        self.colors.light = light;
    }

    #[cfg(feature = "ui")]
    pub fn apply_app_theme(&self, ctx: &Context) {
        use egui::{
            style::{Selection, WidgetVisuals, Widgets},
            Shadow, Style,
        };

        let is_dark_mode = ctx.style().visuals.dark_mode;

        let colors = if is_dark_mode {
            &self.colors.dark
        } else {
            &self.colors.light
        };

        macro_rules! get {
            ($key:expr) => {
                colors.get($key).copied().unwrap_or(Color32::DEBUG_COLOR)
            };
        }

        fn widget_style(fill: Color32, on_fill: Color32) -> WidgetVisuals {
            WidgetVisuals {
                bg_fill: fill,
                weak_bg_fill: fill,
                bg_stroke: Stroke {
                    color: fill,
                    width: 1.0,
                },
                fg_stroke: Stroke {
                    color: on_fill,
                    width: 1.0,
                },
                expansion: 0.0,
                rounding: 5.0.into(),
            }
        }

        let widgets = Widgets {
            noninteractive: widget_style(get!("surface"), get!("on_surface")),
            inactive: widget_style(get!("primary_container"), get!("on_primary_container")),
            hovered: widget_style(get!("tertiary_container"), get!("on_tertiary_container")),
            active: widget_style(get!("tertiary"), get!("on_tertiary")),
            open: widget_style(get!("primary_container"), get!("on_primary_container")),
        };

        let visuals = Visuals {
            override_text_color: Some(get!("on_surface")),
            hyperlink_color: get!("on_primary"),
            faint_bg_color: get!("surface_container"),
            extreme_bg_color: get!("surface_variant"),
            code_bg_color: get!("surface_dim"),
            window_fill: get!("surface_container_highest"),
            panel_fill: get!("surface"),
            warn_fg_color: get!("error_container"),
            error_fg_color: get!("error"),

            selection: Selection {
                bg_fill: get!("secondary"),
                stroke: Stroke {
                    width: 1.5,
                    color: get!("on_secondary"),
                },
            },

            widgets,
            window_shadow: Shadow {
                color: get!("shadow"),
                ..Default::default()
            },
            popup_shadow: Shadow {
                color: get!("shadow"),
                ..Default::default()
            },
            collapsing_header_frame: true,
            window_highlight_topmost: false,
            ..if is_dark_mode {
                Visuals::dark()
            } else {
                Visuals::light()
            }
        };

        ctx.set_style(Style {
            visuals,
            ..Default::default()
        });
    }
}

#[cfg(feature = "ui")]
fn argb_to_color32(color: &Argb) -> Color32 {
    Color32::from_rgba_premultiplied(color.red, color.green, color.blue, color.alpha)
}

#[cfg(feature = "ui")]
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);

        if !self.ran_once {
            self.apply_app_theme(ctx);
        }

        egui::TopBottomPanel::top("my_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.horizontal(|ui| {
                    if ui
                        .selectable_value(&mut self.selected_tab, Tabs::Images, "Images")
                        .clicked()
                        && self.image_folder.is_some()
                    {
                        save_cache(
                            self.image_folder.clone().unwrap(),
                            self.selected_tab.clone(),
                        );
                    };
                    if ui
                        .selectable_value(&mut self.selected_tab, Tabs::Settings, "Settings")
                        .clicked()
                        && self.image_folder.is_some()
                    {
                        save_cache(
                            self.image_folder.clone().unwrap(),
                            self.selected_tab.clone(),
                        )
                    };
                    if ui
                        .selectable_value(&mut self.selected_tab, Tabs::Colors, "Colors")
                        .clicked()
                        && self.image_folder.is_some()
                    {
                        save_cache(
                            self.image_folder.clone().unwrap(),
                            self.selected_tab.clone(),
                        )
                    };
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    self.top_buttons(ui, ctx);
                });
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| self.body(ui, ctx));
    }
}
