use colorsys::{ColorTransform, Hsl, Rgb, SaturationInSpace};
use material_colors::blend::{harmonize as md3_harmonize, hct_hue};

use crate::{
    color::{
        format::{argb_from_hsl, argb_from_rgb, hsl_from_argb, rgb_from_argb},
        parse::parse_css_color,
    },
    expect_args,
    parser::{Engine, FilterError, FilterReturnType, SpannedValue},
};

fn adjust_rgb_lightness(color: &mut Rgb, amount: f64, threshold: f64) {
    let hsl = Hsl::from(color.clone());

    if hsl.lightness() < threshold {
        color.lighten(amount);
    } else {
        color.lighten(-amount);
    }
}

fn adjust_hsl_lightness(color: &mut Hsl, amount: f64, threshold: f64) {
    if color.lightness() < threshold {
        color.lighten(amount);
    } else {
        color.lighten(-amount);
    }
}

pub(crate) fn invert(
    _keywords: &[&str],
    _args: &[SpannedValue],
    original: FilterReturnType,
    _engine: &Engine,
) -> Result<FilterReturnType, FilterError> {
    match original {
        FilterReturnType::String(_) => Err(FilterError::ColorFilterOnString),
        FilterReturnType::Rgb(mut color) => {
            color.invert();
            Ok(FilterReturnType::Rgb(color))
        }
        FilterReturnType::Hsl(mut color) => {
            color.invert();
            Ok(FilterReturnType::Hsl(color))
        }
        FilterReturnType::Bool(_) => Err(FilterError::ColorFilterOnBool),
    }
}

pub(crate) fn grayscale(
    _keywords: &[&str],
    _args: &[SpannedValue],
    original: FilterReturnType,
    _engine: &Engine,
) -> Result<FilterReturnType, FilterError> {
    match original {
        FilterReturnType::String(_) => Err(FilterError::ColorFilterOnString),
        FilterReturnType::Rgb(mut color) => {
            color.grayscale_simple();
            Ok(FilterReturnType::Rgb(color))
        }
        FilterReturnType::Hsl(mut color) => {
            color.invert();
            Ok(FilterReturnType::Hsl(color))
        }
        FilterReturnType::Bool(_) => Err(FilterError::ColorFilterOnBool),
    }
}

// Different name so that set_lightness isnt weird
pub(crate) fn lighten(
    _keywords: &[&str],
    args: &[SpannedValue],
    original: FilterReturnType,
    _engine: &Engine,
) -> Result<FilterReturnType, FilterError> {
    let amt = expect_args!(args, f64);

    match original {
        FilterReturnType::String(_) => Err(FilterError::ColorFilterOnString),
        FilterReturnType::Rgb(mut color) => {
            color.lighten(amt);
            Ok(FilterReturnType::Rgb(color))
        }
        FilterReturnType::Hsl(mut color) => {
            color.lighten(amt);
            Ok(FilterReturnType::Hsl(color))
        }
        FilterReturnType::Bool(_) => Err(FilterError::ColorFilterOnBool),
    }
}

// Different name so that set_lightness isnt weird
pub(crate) fn auto_lighten(
    _keywords: &[&str],
    args: &[SpannedValue],
    original: FilterReturnType,
    _engine: &Engine,
) -> Result<FilterReturnType, FilterError> {
    let threshold = 50.0;
    let amt = expect_args!(args, f64);

    match original {
        FilterReturnType::String(_) => Err(FilterError::ColorFilterOnString),
        FilterReturnType::Rgb(mut color) => {
            adjust_rgb_lightness(&mut color, amt, threshold);
            Ok(FilterReturnType::Rgb(color))
        }
        FilterReturnType::Hsl(mut color) => {
            adjust_hsl_lightness(&mut color, amt, threshold);
            Ok(FilterReturnType::Hsl(color))
        }
        FilterReturnType::Bool(_) => Err(FilterError::ColorFilterOnBool),
    }
}

pub(crate) fn saturate(
    _keywords: &[&str],
    args: &[SpannedValue],
    original: FilterReturnType,
    _engine: &Engine,
) -> Result<FilterReturnType, FilterError> {
    let (amt, space) = expect_args!(args, f64, String);

    let saturation = match space.as_str() {
        a if a.to_lowercase() == "hsl" => SaturationInSpace::Hsl(amt),
        a if a.to_lowercase() == "hsv" => SaturationInSpace::Hsv(amt),
        _ => {
            return Err(FilterError::UnexpectedStringValue {
                expected: "hsv, hsl".to_owned(),
                span: args[1].span,
            })
        }
    };

    match original {
        FilterReturnType::String(_) => Err(FilterError::ColorFilterOnString),
        FilterReturnType::Rgb(mut color) => {
            color.saturate(saturation);
            Ok(FilterReturnType::Rgb(color))
        }
        FilterReturnType::Hsl(mut color) => {
            color.saturate(saturation);
            Ok(FilterReturnType::Hsl(color))
        }
        FilterReturnType::Bool(_) => Err(FilterError::ColorFilterOnBool),
    }
}

pub(crate) fn to_color(
    _keywords: &[&str],
    _args: &[SpannedValue],
    original: FilterReturnType,
    _engine: &Engine,
) -> Result<FilterReturnType, FilterError> {
    match original {
        FilterReturnType::String(s) => Ok(FilterReturnType::Rgb(parse_css_color(&s).unwrap())),
        FilterReturnType::Rgb(color) => Ok(FilterReturnType::Rgb(color)),
        FilterReturnType::Hsl(color) => Ok(FilterReturnType::Hsl(color)),
        // TODO: Add proper error here
        FilterReturnType::Bool(_) => Err(FilterError::ColorFilterOnBool),
    }
}

pub(crate) fn blend(
    _keywords: &[&str],
    args: &[SpannedValue],
    original: FilterReturnType,
    _engine: &Engine,
) -> Result<FilterReturnType, FilterError> {
    let (blend_with, amount) = expect_args!(args, Rgb, f64);

    match original {
        FilterReturnType::String(_) => Err(FilterError::ColorFilterOnString),
        FilterReturnType::Rgb(color) => {
            let res = hct_hue(argb_from_rgb(&color), argb_from_rgb(&blend_with), amount);
            Ok(FilterReturnType::Rgb(rgb_from_argb(res)))
        }
        FilterReturnType::Hsl(color) => {
            let res = hct_hue(argb_from_hsl(&color), argb_from_rgb(&blend_with), amount);
            Ok(FilterReturnType::Hsl(hsl_from_argb(res)))
        }
        FilterReturnType::Bool(_) => Err(FilterError::ColorFilterOnBool),
    }
}

pub(crate) fn harmonize(
    _keywords: &[&str],
    args: &[SpannedValue],
    original: FilterReturnType,
    _engine: &Engine,
) -> Result<FilterReturnType, FilterError> {
    let blend_with = expect_args!(args, Rgb);

    match original {
        FilterReturnType::String(_) => Err(FilterError::ColorFilterOnString),
        FilterReturnType::Rgb(color) => {
            let res = md3_harmonize(argb_from_rgb(&color), argb_from_rgb(&blend_with));
            Ok(FilterReturnType::Rgb(rgb_from_argb(res)))
        }
        FilterReturnType::Hsl(color) => {
            let res = md3_harmonize(argb_from_hsl(&color), argb_from_rgb(&blend_with));
            Ok(FilterReturnType::Hsl(hsl_from_argb(res)))
        }
        FilterReturnType::Bool(_) => Err(FilterError::ColorFilterOnBool),
    }
}
