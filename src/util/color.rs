use material_color_utilities_rs::{scheme::Scheme, util::color::format_argb_as_rgb};
use owo_colors::{OwoColorize, Style};

// TODO Fix this monstrosity

#[derive(Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Color {
    pub fn new(colors: [u8; 4]) -> Color {
        Color {
            red: colors[1],
            green: colors[2],
            blue: colors[3],
            alpha: colors[0],
        }
    }
}

pub trait SchemeExt {
    fn get_value(&self, field: &str) -> &[u8; 4];
}
impl SchemeExt for Scheme {
    fn get_value(&self, field: &str) -> &[u8; 4] {
        match field {
            "primary" => &self.primary,
            "on_primary" => &self.on_primary,
            "primary_container" => &self.primary_container,
            "on_primary_container" => &self.on_primary_container,
            "secondary" => &self.secondary,
            "on_secondary" => &self.on_secondary,
            "secondary_container" => &self.secondary_container,
            "on_secondary_container" => &self.on_secondary_container,
            "tertiary" => &self.tertiary,
            "on_tertiary" => &self.on_tertiary,
            "tertiary_container" => &self.tertiary_container,
            "on_tertiary_container" => &self.on_tertiary_container,
            "error" => &self.error,
            "on_error" => &self.on_error,
            "error_container" => &self.error_container,
            "on_error_container" => &self.on_error_container,
            "background" => &self.background,
            "on_background" => &self.on_background,
            "surface" => &self.surface,
            "on_surface" => &self.on_surface,
            "surface_variant" => &self.surface_variant,
            "on_surface_variant" => &self.on_surface_variant,
            "outline" => &self.outline,
            "outline_variant" => &self.outline_variant,
            "shadow" => &self.shadow,
            "scrim" => &self.scrim,
            "inverse_surface" => &self.inverse_surface,
            "inverse_on_surface" => &self.inverse_on_surface,
            "inverse_primary" => &self.inverse_primary,
            _ => panic!(),
        }
    }
}

pub fn show_color(scheme: &Scheme, colors: &Vec<&str>) {
    for field in colors {
        let color: Color = Color::new(*Scheme::get_value(scheme, field));

        let luma = color.red as u16 + color.blue as u16 + color.green as u16;

        let formatstr = format!("#{:x}{:x}{:x}", color.red, color.green, color.blue);
        let owo_color: owo_colors::Rgb = owo_colors::Rgb(color.red, color.green, color.blue);

        let style = if luma > 500 {
            Style::new().black().on_color(owo_color)
        } else {
            Style::new().white().on_color(owo_color)
        };

        let color_str = formatstr.style(style);

        print!("{}", color_str);
    }
    println!("\n");
}
