use colorsys::{ColorTransform, SaturationInSpace};

use crate::{
    expect_args,
    parser::{Engine, FilterError, FilterReturnType, SpannedValue},
};

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
    }
}

// pub(crate) fn blend(
//     keywords: &[&str],
//     args: &[SpannedValue],
//     original: FilterReturnType,
//     engine: &Engine,
// ) -> Result<FilterReturnType, FilterError> {
// }
