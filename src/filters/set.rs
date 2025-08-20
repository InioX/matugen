use colorsys::{ColorAlpha, Hsl, Rgb};

use crate::{
    expect_args,
    parser::{Engine, FilterError, FilterReturnType, SpannedValue},
};

pub(crate) fn set_red(
    _keywords: &[&str],
    args: &[SpannedValue],
    original: FilterReturnType,
    _engine: &Engine,
) -> Result<FilterReturnType, FilterError> {
    let amt = expect_args!(args, f64);

    match original {
        FilterReturnType::String(_) => Err(FilterError::ColorFilterOnString),
        FilterReturnType::Rgb(mut color) => {
            color.set_red(amt);
            Ok(FilterReturnType::Rgb(color))
        }
        FilterReturnType::Hsl(color) => {
            let mut rgb: Rgb = color.into();
            rgb.set_red(amt);
            Ok(FilterReturnType::Rgb(rgb))
        }
        FilterReturnType::Bool(_) => Err(FilterError::ColorFilterOnBool),
    }
}

pub(crate) fn set_green(
    _keywords: &[&str],
    args: &[SpannedValue],
    original: FilterReturnType,
    _engine: &Engine,
) -> Result<FilterReturnType, FilterError> {
    let amt = expect_args!(args, f64);

    match original {
        FilterReturnType::String(_) => Err(FilterError::ColorFilterOnString),
        FilterReturnType::Rgb(mut color) => {
            color.set_green(amt);
            Ok(FilterReturnType::Rgb(color))
        }
        FilterReturnType::Hsl(color) => {
            let mut rgb: Rgb = color.into();
            rgb.set_green(amt);
            Ok(FilterReturnType::Rgb(rgb))
        }
        FilterReturnType::Bool(_) => Err(FilterError::ColorFilterOnBool),
    }
}

pub(crate) fn set_blue(
    _keywords: &[&str],
    args: &[SpannedValue],
    original: FilterReturnType,
    _engine: &Engine,
) -> Result<FilterReturnType, FilterError> {
    let amt = expect_args!(args, f64);

    match original {
        FilterReturnType::String(_) => Err(FilterError::ColorFilterOnString),
        FilterReturnType::Rgb(mut color) => {
            color.set_blue(amt);
            Ok(FilterReturnType::Rgb(color))
        }
        FilterReturnType::Hsl(color) => {
            let mut rgb: Rgb = color.into();
            rgb.set_blue(amt);
            Ok(FilterReturnType::Rgb(rgb))
        }
        FilterReturnType::Bool(_) => Err(FilterError::ColorFilterOnBool),
    }
}

pub(crate) fn set_alpha(
    _keywords: &[&str],
    args: &[SpannedValue],
    original: FilterReturnType,
    _engine: &Engine,
) -> Result<FilterReturnType, FilterError> {
    let amt = expect_args!(args, f64);

    match original {
        FilterReturnType::String(_) => Err(FilterError::ColorFilterOnString),
        FilterReturnType::Rgb(mut color) => {
            color.set_alpha(amt);
            Ok(FilterReturnType::Rgb(color))
        }
        FilterReturnType::Hsl(mut color) => {
            color.set_alpha(amt);
            Ok(FilterReturnType::Hsl(color))
        }
        FilterReturnType::Bool(_) => Err(FilterError::ColorFilterOnBool),
    }
}

pub(crate) fn set_hue(
    _keywords: &[&str],
    args: &[SpannedValue],
    original: FilterReturnType,
    _engine: &Engine,
) -> Result<FilterReturnType, FilterError> {
    let amt = expect_args!(args, f64);

    match original {
        FilterReturnType::String(_) => Err(FilterError::ColorFilterOnString),
        FilterReturnType::Rgb(color) => {
            let mut hsl: Hsl = color.into();
            hsl.set_hue(amt);
            Ok(FilterReturnType::Hsl(hsl))
        }
        FilterReturnType::Hsl(mut color) => {
            color.set_hue(amt);
            Ok(FilterReturnType::Hsl(color))
        }
        FilterReturnType::Bool(_) => Err(FilterError::ColorFilterOnBool),
    }
}

pub(crate) fn set_saturation(
    _keywords: &[&str],
    args: &[SpannedValue],
    original: FilterReturnType,
    _engine: &Engine,
) -> Result<FilterReturnType, FilterError> {
    let amt = expect_args!(args, f64);

    match original {
        FilterReturnType::String(_) => Err(FilterError::ColorFilterOnString),
        FilterReturnType::Rgb(color) => {
            let mut hsl: Hsl = color.into();
            hsl.set_saturation(amt);
            Ok(FilterReturnType::Hsl(hsl))
        }
        FilterReturnType::Hsl(mut color) => {
            color.set_saturation(amt);
            Ok(FilterReturnType::Hsl(color))
        }
        FilterReturnType::Bool(_) => Err(FilterError::ColorFilterOnBool),
    }
}

pub(crate) fn set_lightness(
    _keywords: &[&str],
    args: &[SpannedValue],
    original: FilterReturnType,
    _engine: &Engine,
) -> Result<FilterReturnType, FilterError> {
    let amt = expect_args!(args, f64);

    match original {
        FilterReturnType::String(_) => Err(FilterError::ColorFilterOnString),
        FilterReturnType::Rgb(color) => {
            let mut hsl: Hsl = color.into();
            hsl.set_lightness(amt);
            Ok(FilterReturnType::Hsl(hsl))
        }
        FilterReturnType::Hsl(mut color) => {
            color.set_lightness(amt);
            Ok(FilterReturnType::Hsl(color))
        }
        FilterReturnType::Bool(_) => Err(FilterError::ColorFilterOnBool),
    }
}
