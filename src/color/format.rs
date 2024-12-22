use colorsys::{ColorAlpha, Hsl, Rgb};
use material_colors::color::Argb;

pub fn rgb_from_argb(color: Argb) -> Rgb {
    Rgb::from([
        color.red as f64,
        color.green as f64,
        color.blue as f64,
        color.alpha as f64,
    ])
}

pub fn hsl_from_argb(color: Argb) -> Hsl {
    rgb_from_argb(color).as_ref().into()
}

pub fn hsl_from_rgb(color: Rgb) -> Hsl {
    color.as_ref().into()
}

pub fn format_hex(color: &Rgb) -> String {
    color.to_hex_string()
}

pub fn format_hex_stripped(color: &Rgb) -> String {
    color.to_hex_string()[1..].to_string()
}

pub fn format_rgb(color: &Rgb) -> String {
    format!(
        "rgb({:?}, {:?}, {:?})",
        color.red() as u8,
        color.green() as u8,
        color.blue() as u8,
    )
}

pub fn format_rgba(color: &Rgb, divide: bool) -> String {
    if divide {
        format!(
            "rgba({:?}, {:?}, {:?}, {:.1})",
            color.red() as u8,
            color.green() as u8,
            color.blue() as u8,
            color.alpha() / 255.
        )
    } else {
        format!(
            "rgba({:?}, {:?}, {:?}, {:.1})",
            color.red() as u8,
            color.green() as u8,
            color.blue() as u8,
            color.alpha()
        )
    }
}

pub fn format_hsl(color: &Hsl) -> String {
    format!(
        "hsl({:?}, {:?}%, {:?}%)",
        color.hue() as u8,
        color.saturation() as u8,
        color.lightness() as u8,
    )
}

pub fn format_hsla(color: &Hsl, divide: bool) -> String {
    if divide {
        format!(
            "hsla({:?}, {:?}%, {:?}%, {:.1})",
            color.hue() as u8,
            color.saturation() as u8,
            color.lightness() as u8,
            color.alpha() / 255.
        )
    } else {
        format!(
            "hsla({:?}, {:?}%, {:?}%, {:.1})",
            color.hue() as u8,
            color.saturation() as u8,
            color.lightness() as u8,
            color.alpha()
        )
    }
}
