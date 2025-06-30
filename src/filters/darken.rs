use colorsys::{ColorAlpha, ColorTransform, Rgb};
use material_colors::color::Argb;

use crate::{
    expect_args,
    parser::{Engine, FilterError, FilterReturnType, SpannedValue},
};

pub(crate) fn darken(
    keywords: &[&str],
    args: &[SpannedValue],
    original: FilterReturnType,
    engine: &Engine,
) -> Result<FilterReturnType, FilterError> {
    let amt = expect_args!(args, f64);

    match original {
        FilterReturnType::String(s) => Err(FilterError::ColorFilterOnString),
        FilterReturnType::Color(mut color) => {
            color.lighten(-amt);
            Ok(FilterReturnType::Color(color))
        }
    }
}
