use colorsys::{ColorTransform, Hsl, Rgb, SaturationInSpace};

use crate::{
    expect_args,
    parser::{Engine, FilterError, FilterReturnType, SpannedValue},
};

fn adjust_rgb_lightness(color: &mut Rgb, amount: f64, threshold: f64) {
    let hsl = Hsl::from(color.clone()); // Convert RGB to HSL

    // Adjust lightness based on the threshold
    if hsl.lightness() < threshold {
        color.lighten(amount); // Increase lightness
    } else {
        color.lighten(-amount); // Decrease lightness
    }
}

fn adjust_hsl_lightness(color: &mut Hsl, amount: f64, threshold: f64) {
    // Adjust lightness based on the threshold
    if color.lightness() < threshold {
        color.lighten(amount); // Increase lightness
    } else {
        color.lighten(-amount); // Decrease lightness
    }
}

pub(crate) fn invert(
    keywords: &[&str],
    args: &[SpannedValue],
    original: FilterReturnType,
    engine: &Engine,
) -> Result<FilterReturnType, FilterError> {
    match original {
        FilterReturnType::String(s) => Err(FilterError::ColorFilterOnString),
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
    keywords: &[&str],
    args: &[SpannedValue],
    original: FilterReturnType,
    engine: &Engine,
) -> Result<FilterReturnType, FilterError> {
    match original {
        FilterReturnType::String(s) => Err(FilterError::ColorFilterOnString),
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
    keywords: &[&str],
    args: &[SpannedValue],
    original: FilterReturnType,
    engine: &Engine,
) -> Result<FilterReturnType, FilterError> {
    let amt = expect_args!(args, f64);

    match original {
        FilterReturnType::String(s) => Err(FilterError::ColorFilterOnString),
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
    keywords: &[&str],
    args: &[SpannedValue],
    original: FilterReturnType,
    engine: &Engine,
) -> Result<FilterReturnType, FilterError> {
    let threshold = 50.0;
    let amt = expect_args!(args, f64);

    match original {
        FilterReturnType::String(s) => Err(FilterError::ColorFilterOnString),
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
    keywords: &[&str],
    args: &[SpannedValue],
    original: FilterReturnType,
    engine: &Engine,
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
        FilterReturnType::String(s) => Err(FilterError::ColorFilterOnString),
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

// pub(crate) fn blend(
//     keywords: &[&str],
//     args: &[SpannedValue],
//     original: FilterReturnType,
//     engine: &Engine,
// ) -> Result<FilterReturnType, FilterError> {
// }
