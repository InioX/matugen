use colorsys::{ColorTransform, Hsl, Rgb, SaturationInSpace};

use crate::{
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
