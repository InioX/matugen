use std::path::PathBuf;

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

use crate::{template::TemplateFile, util::arguments::Cli, State};

#[cfg(feature = "ui")]
#[derive(PartialEq)]
pub enum Tabs {
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
}

#[cfg(feature = "ui")]
pub struct ColorsMap {
    pub light: Option<IndexMap<String, Color32>>,
    pub dark: Option<IndexMap<String, Color32>>,
}

// FIXME: Cleanup code, reorganize stuff into its own functions

#[cfg(feature = "ui")]
impl MyApp {
    pub fn new(_cc: &eframe::CreationContext, cli: Box<Cli>) -> Self {
        Self {
            selected_file: None,
            app: crate::State::new(*cli.clone()),
            cli,
            colors: ColorsMap {
                light: None,
                dark: None,
            },
            selected_tab: Tabs::Settings,
        }
    }
    fn body(&mut self, ui: &mut Ui) {
        match self.selected_tab {
            Tabs::Settings => self.settings(ui),
            Tabs::Colors => self.colors(ui),
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
        egui::ComboBox::from_label("Select one!")
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

    fn top_buttons(&mut self, ui: &mut Ui) {
        if ui.button("Select image").clicked() {
            if let Some(path) = rfd::FileDialog::new().pick_file() {
                self.selected_file = Some(path);
            }
        }
        if ui.button("Run").clicked() {
            if self.selected_file != Some("".into()) || self.selected_file.is_some() {
                self.update_colors_tab();
                self.generate_tempalates();
            };
        }
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
        egui::TopBottomPanel::top("my_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.selected_tab, Tabs::Settings, "Settings");
                    ui.selectable_value(&mut self.selected_tab, Tabs::Colors, "Colors");
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    self.top_buttons(ui);
                });
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| self.body(ui));
    }
}
