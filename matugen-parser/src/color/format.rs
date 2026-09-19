use colorsys::{ColorAlpha, Hsl, Rgb as ColorSysRgb};

pub fn format_hex(color: &ColorSysRgb) -> String {
    color.to_hex_string()
}

pub fn format_hex_stripped(color: &ColorSysRgb) -> String {
    color.to_hex_string()[1..].to_string()
}

pub fn format_hex_alpha(color: &ColorSysRgb) -> String {
    let alpha = alpha_u8(color.alpha());
    format!(
        "#{:02X}{:02X}{:02X}{:02X}",
        color.red() as u8,
        color.green() as u8,
        color.blue() as u8,
        alpha
    )
}

pub fn format_hex_alpha_stripped(color: &ColorSysRgb) -> String {
    let alpha = alpha_u8(color.alpha());
    format!(
        "{:02X}{:02X}{:02X}{:02X}",
        color.red() as u8,
        color.green() as u8,
        color.blue() as u8,
        alpha
    )
}

pub fn format_alpha_hex(color: &ColorSysRgb) -> String {
    let alpha = alpha_u8(color.alpha());
    format!(
        "#{:02X}{:02X}{:02X}{:02X}",
        alpha,
        color.red() as u8,
        color.green() as u8,
        color.blue() as u8
    )
}

pub fn format_alpha_hex_stripped(color: &ColorSysRgb) -> String {
    let alpha = alpha_u8(color.alpha());
    format!(
        "{:02X}{:02X}{:02X}{:02X}",
        alpha,
        color.red() as u8,
        color.green() as u8,
        color.blue() as u8
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

pub fn format_rgb(color: &ColorSysRgb) -> String {
    format!(
        "rgb({:?}, {:?}, {:?})",
        color.red() as u8,
        color.green() as u8,
        color.blue() as u8,
    )
}

pub fn format_rgba(color: &ColorSysRgb) -> String {
    let alpha = (color.alpha() * 100.0).round() / 100.0;

    format!(
        "rgba({:.0}, {:.0}, {:.0}, {})",
        color.red(),
        color.green(),
        color.blue(),
        alpha
    )
}

pub fn format_hsl(color: &Hsl) -> String {
    format!(
        "hsl({:.0}, {:.0}%, {:.0}%)",
        color.hue(),
        color.saturation(),
        color.lightness(),
    )
}

pub fn format_hsla(color: &Hsl) -> String {
    let alpha = (color.alpha() * 100.0).round() / 100.0;

    format!(
        "hsla({:.0}, {:.0}%, {:.0}%, {})",
        color.hue(),
        color.saturation(),
        color.lightness(),
        alpha
    )
}
