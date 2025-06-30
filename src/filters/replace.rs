use crate::{
    expect_args,
    parser::{
        engine::{format_color, format_color_hsl},
        Engine, FilterError, FilterReturnType, SpannedValue,
    },
};

pub(crate) fn replace(
    keywords: &[&str],
    args: &[SpannedValue],
    original: FilterReturnType,
    engine: &Engine,
) -> Result<FilterReturnType, FilterError> {
    let (find, replace) = expect_args!(args, String, String);

    match original {
        FilterReturnType::String(s) => Ok(FilterReturnType::String(s.replace(&find, &replace))),
        FilterReturnType::Rgb(color) => {
            let string = format_color(color, keywords[3]);
            let modified: String = string.unwrap().into().replace(&find, &replace);
            Ok(FilterReturnType::String(modified))
        }
        FilterReturnType::Hsl(color) => {
            let string = format_color_hsl(color, keywords[3]);
            let modified: String = string.unwrap().into().replace(&find, &replace);
            Ok(FilterReturnType::String(modified))
        }
    }
}
