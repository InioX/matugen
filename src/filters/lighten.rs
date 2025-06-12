use colorsys::{ColorAlpha, ColorTransform, Rgb};
use material_colors::color::Argb;

use crate::{
    engine::{Engine, FilterError, FilterErrorKind, FilterReturnType, SpannedValue, Value},
    expect_args,
};

pub(crate) fn lighten(
    keywords: &Vec<&str>,
    args: Vec<SpannedValue>,
    original: FilterReturnType,
    engine: &Engine,
) -> Result<FilterReturnType, FilterError> {
    let amt = expect_args!(args, f64);

    match original {
        FilterReturnType::String(s) => Err(FilterError::new(FilterErrorKind::ColorFilterOnString)),
        FilterReturnType::Color(argb) => {
            let mut color = Rgb::from((argb.red, argb.green, argb.blue));
            color.lighten(amt);
            Ok(FilterReturnType::Color(Argb {
                alpha: color.alpha() as u8,
                red: color.red() as u8,
                green: color.green() as u8,
                blue: color.blue() as u8,
            }))
        }
    }
}
