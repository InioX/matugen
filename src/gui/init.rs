use std::{ffi::OsStr, path::PathBuf};

#[cfg(feature = "ui")]
use indexmap::IndexMap;

#[cfg(feature = "ui")]
use eframe::egui;
use egui::{Color32, Ui, Vec2};
use material_colors::{color::Argb, scheme::variant::SchemeFidelity};
use matugen::{
    color::{
        color::Source,
        format::{format_hex, rgb_from_argb},
    },
    scheme::{SchemeTypes, Schemes},
};
use serde::{Deserialize, Serialize};

use crate::{template::TemplateFile, util::arguments::Cli, State};

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
}

#[cfg(feature = "ui")]
pub struct ColorsMap {
    pub light: Option<IndexMap<String, Color32>>,
    pub dark: Option<IndexMap<String, Color32>>,
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

// FIXME: Cleanup code, reorganize stuff into its own functions

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
            colors: ColorsMap {
                light: None,
                dark: None,
            },
            selected_tab,
            images_vec: vec![],
            load_cache: if is_cache { true } else { false },
            image_folder,
        }
    }
    fn body(&mut self, ui: &mut Ui) {
        if self.load_cache {
            self.update_images_tab(ui);
            self.load_cache = false;
        }
        match self.selected_tab {
            Tabs::Settings => self.settings(ui),
            Tabs::Colors => self.colors(ui),
            Tabs::Images => self.images(ui),
        }
    }

    fn settings(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Contrast");
            ui.add(egui::Slider::new(
                &mut self.app.args.contrast.unwrap(),
                -1.0..=1.0,
            ));
            ui.label("Lightness");
            ui.add(egui::Slider::new(
                &mut self.app.args.lightness.unwrap(),
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
        colors: &Option<IndexMap<String, Color32>>,
    ) {
        ui.vertical(|ui| {
            ui.label(egui::RichText::new(text).strong());
            if let Some(colors) = &colors {
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
            }
        });
    }

    fn show_colors(&mut self, ui: &mut Ui) {
        self.show_single_color_scrollable(ui, "DARK", &self.colors.dark);

        self.show_single_color_scrollable(ui, "LIGHT", &self.colors.light);
    }

    fn colors(&mut self, ui: &mut Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            if self.colors.dark != None || self.colors.light != None {
                ui.horizontal(|ui| {
                    self.show_colors(ui);
                });
            } else {
                ui.label("No colors generated yet");
            }
        });
    }

    fn images(&mut self, ui: &mut Ui) {
        if self.images_vec.is_empty() {
            ui.label("No image folder selected or no images to show.");
        } else {
            ui.with_layout(
                egui::Layout::left_to_right(egui::Align::Max).with_cross_justify(true),
                |ui| {
                    self.show_images(ui);
                },
            );
        }
    }

    fn show_images(&mut self, ui: &mut Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("image_grid")
                .num_columns(3) // Set number of columns
                .spacing([10.0, 10.0]) // Spacing between items
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

    fn top_buttons(&mut self, ui: &mut Ui) {
        if ui.button("Image Folder").clicked() {
            if let Some(path) = rfd::FileDialog::new().pick_folder() {
                self.image_folder = Some(path);
                self.update_images_tab(ui)
            }
        }
        if ui.button("Select image").clicked() {
            if let Some(path) = rfd::FileDialog::new().pick_file() {
                self.selected_file = Some(path);
            }
        }
        if ui.button("Run").clicked() {
            self.run()
        }
    }

    fn run(&mut self) {
        if self.selected_file == Some("".into()) || self.selected_file.is_none() {
            return;
        };
        self.generate_tempalates();
        self.update_colors_tab();
    }

    fn generate_tempalates(&mut self) {
        let mut engine = self.app.init_engine();
        let mut render_data = self.app.init_render_data().unwrap();
        let mut template = TemplateFile::new(&self.app, &mut engine, &mut render_data);
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
        self.colors.dark = Some(dark);
        self.colors.light = Some(light);
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
        egui::TopBottomPanel::top("my_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.horizontal(|ui| {
                    if ui
                        .selectable_value(&mut self.selected_tab, Tabs::Images, "Images")
                        .clicked()
                    {
                        save_cache(
                            self.image_folder.clone().unwrap(),
                            self.selected_tab.clone(),
                        );
                    };
                    if ui
                        .selectable_value(&mut self.selected_tab, Tabs::Settings, "Settings")
                        .clicked()
                    {
                        save_cache(
                            self.image_folder.clone().unwrap(),
                            self.selected_tab.clone(),
                        )
                    };
                    if ui
                        .selectable_value(&mut self.selected_tab, Tabs::Colors, "Colors")
                        .clicked()
                    {
                        save_cache(
                            self.image_folder.clone().unwrap(),
                            self.selected_tab.clone(),
                        )
                    };
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    self.top_buttons(ui);
                });
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| self.body(ui));
    }
}
