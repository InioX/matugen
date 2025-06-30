use colorsys::ColorTransform;

use crate::parser::{Engine, FilterError, FilterReturnType, SpannedValue};

pub(crate) fn invert(
    keywords: &[&str],
    args: &[SpannedValue],
    original: FilterReturnType,
    engine: &Engine,
) -> Result<FilterReturnType, FilterError> {
    match original {
        FilterReturnType::String(s) => Err(FilterError::ColorFilterOnString),
        FilterReturnType::Color(mut color) => {
            color.invert();
            Ok(FilterReturnType::Color(color))
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
        FilterReturnType::Color(mut color) => {
            color.grayscale_simple();
            Ok(FilterReturnType::Color(color))
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
