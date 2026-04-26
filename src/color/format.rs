use colorsys::{Hsl, Rgb as ColorSysRgb};
use material_colors::color::Rgb;

pub fn argb_from_rgb(color: &ColorSysRgb) -> Rgb {
    Rgb {
        red: color.red() as u8,
        green: color.green() as u8,
        blue: color.blue() as u8,
        // alpha: (color.alpha() * 255.0) as u8,
    }
}

pub fn argb_from_hsl(color: &Hsl) -> Rgb {
    let color: ColorSysRgb = color.into();
    argb_from_rgb(&color)
}

pub fn rgb_from_argb(color: Rgb) -> ColorSysRgb {
    ColorSysRgb::from([
        color.red as f64,
        color.green as f64,
        color.blue as f64,
        // color.alpha as f64,
    ])
}

pub fn hsl_from_argb(color: Rgb) -> Hsl {
    rgb_from_argb(color).as_ref().into()
}

pub fn hsl_from_rgb(color: ColorSysRgb) -> Hsl {
    color.as_ref().into()
}
