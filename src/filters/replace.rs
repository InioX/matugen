use crate::{
    expect_args,
    parser::{Engine, FilterError, FilterReturnType, SpannedValue},
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
        FilterReturnType::Color(color) => {
            let string = format_color(&color, keywords[3]);
            let modified: String = string.into().replace(&find, &replace);
            Ok(FilterReturnType::String(modified))
        }
    }
}
