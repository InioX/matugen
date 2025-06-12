use colorsys::{ColorAlpha, ColorTransform, Rgb};
use material_colors::color::Argb;

use crate::{
    engine::{
        engine::format_color, Engine, FilterError, FilterErrorKind, FilterReturnType, SpannedValue,
        Value,
    },
    expect_args,
};

pub(crate) fn replace(
    keywords: &Vec<&str>,
    args: Vec<SpannedValue>,
    original: FilterReturnType,
    engine: &Engine,
) -> Result<FilterReturnType, FilterError> {
    let (find, replace) = expect_args!(args, String, String);

    match original {
        FilterReturnType::String(s) => Ok(FilterReturnType::String(s.replace(&find, &replace))),
        FilterReturnType::Color(color) => {
            let string = format_color(&color, keywords[3]);
            let modified: String = string.into().replace(&find, &replace);
            Ok(FilterReturnType::String(modified.into()))
        }
    }
}
