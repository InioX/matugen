use colorsys::{ColorAlpha, Hsl, Rgb};
use material_colors::color::Argb;

pub fn argb_from_rgb(color: &Rgb) -> Argb {
    Argb {
        red: color.red() as u8,
        green: color.green() as u8,
        blue: color.blue() as u8,
        alpha: (color.alpha() * 255.0) as u8,
    }
}

pub fn argb_from_hsl(color: &Hsl) -> Argb {
    let color: Rgb = color.into();
    argb_from_rgb(&color)
}

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

pub fn format_hex_alpha(color: &Rgb) -> String {
    let alpha = alpha_u8(color.alpha());
    format!(
        "#{:02X}{:02X}{:02X}{:02X}",
        color.red() as u8,
        color.green() as u8,
        color.blue() as u8,
        alpha
    )
}

pub fn format_hex_alpha_stripped(color: &Rgb) -> String {
    let alpha = alpha_u8(color.alpha());
    format!(
        "{:02X}{:02X}{:02X}{:02X}",
        color.red() as u8,
        color.green() as u8,
        color.blue() as u8,
        alpha
    )
}

// alpha can be 0..1 (CSS parsing / set_alpha) or 0..255 (ARGB conversions).
// Normalize to 0..255 so hex output is consistent.
fn alpha_u8(alpha: f64) -> u8 {
    if alpha <= 1.0 {
        (alpha.clamp(0.0, 1.0) * 255.0).round() as u8
    } else {
        alpha.clamp(0.0, 255.0).round() as u8
    }
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
            "rgba({:.0}, {:.0}, {:.0}, {})",
            color.red(),
            color.green(),
            color.blue(),
            color.alpha() / 255.
        )
    } else {
        format!(
            "rgba({:.0}, {:.0}, {:.0}, {})",
            color.red(),
            color.green(),
            color.blue(),
            color.alpha()
        )
    }
}

pub fn format_hsl(color: &Hsl) -> String {
    format!(
        "hsl({:.0}, {:.0}%, {:.0}%)",
        color.hue(),
        color.saturation(),
        color.lightness(),
    )
}

pub fn format_hsla(color: &Hsl, divide: bool) -> String {
    if divide {
        format!(
            "hsla({:.0}, {:.0}%, {:.0}%, {})",
            color.hue(),
            color.saturation(),
            color.lightness(),
            color.alpha() / 255.
        )
    } else {
        format!(
            "hsla({:.0}, {:.0}%, {:.0}%, {})",
            color.hue(),
            color.saturation(),
            color.lightness(),
            color.alpha()
        )
    }
}
