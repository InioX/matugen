use convert_case::{Case, Casing};

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
    _engine: &Engine,
) -> Result<FilterReturnType, FilterError> {
    let (find, replace) = expect_args!(args, String, String);

    match original {
        FilterReturnType::String(s) => Ok(FilterReturnType::String(s.replace(&find, &replace))),
        FilterReturnType::Rgb(color) => {
            let string = format_color(color, keywords.last().expect("Could not get format"));
            let modified: String = string.unwrap().to_string().replace(&find, &replace);
            Ok(FilterReturnType::String(modified))
        }
        FilterReturnType::Hsl(color) => {
            let string = format_color_hsl(color, keywords.last().expect("Could not get format"));
            let modified: String = string.unwrap().to_string().replace(&find, &replace);
            Ok(FilterReturnType::String(modified))
        }
        FilterReturnType::Bool(boolean) => match boolean {
            true => Ok(FilterReturnType::String("true".replace(&find, &replace))),
            false => Ok(FilterReturnType::String("false".replace(&find, &replace))),
        },
    }
}

pub(crate) fn lower_case(
    keywords: &[&str],
    _args: &[SpannedValue],
    original: FilterReturnType,
    _engine: &Engine,
) -> Result<FilterReturnType, FilterError> {
    match original {
        FilterReturnType::String(s) => Ok(FilterReturnType::String(s.to_case(Case::Lower))),
        FilterReturnType::Rgb(color) => {
            let string =
                format_color(color, keywords.last().expect("Could not get format")).unwrap();
            Ok(FilterReturnType::String(
                string.to_string().to_case(Case::Lower),
            ))
        }
        FilterReturnType::Hsl(color) => {
            let string =
                format_color_hsl(color, keywords.last().expect("Could not get format")).unwrap();
            Ok(FilterReturnType::String(
                string.to_string().to_case(Case::Lower),
            ))
        }
        FilterReturnType::Bool(boolean) => match boolean {
            true => Ok(FilterReturnType::String(
                "true".to_string().to_case(Case::Lower),
            )),
            false => Ok(FilterReturnType::String(
                "false".to_string().to_case(Case::Lower),
            )),
        },
    }
}

pub(crate) fn camel_case(
    keywords: &[&str],
    _args: &[SpannedValue],
    original: FilterReturnType,
    _engine: &Engine,
) -> Result<FilterReturnType, FilterError> {
    match original {
        FilterReturnType::String(s) => Ok(FilterReturnType::String(s.to_case(Case::Camel))),
        FilterReturnType::Rgb(color) => {
            let string =
                format_color(color, keywords.last().expect("Could not get format")).unwrap();
            Ok(FilterReturnType::String(
                string.to_string().to_case(Case::Camel),
            ))
        }
        FilterReturnType::Hsl(color) => {
            let string =
                format_color_hsl(color, keywords.last().expect("Could not get format")).unwrap();
            Ok(FilterReturnType::String(
                string.to_string().to_case(Case::Camel),
            ))
        }
        FilterReturnType::Bool(boolean) => match boolean {
            true => Ok(FilterReturnType::String(
                "true".to_string().to_case(Case::Camel),
            )),
            false => Ok(FilterReturnType::String(
                "false".to_string().to_case(Case::Camel),
            )),
        },
    }
}

pub(crate) fn pascal_case(
    keywords: &[&str],
    _args: &[SpannedValue],
    original: FilterReturnType,
    _engine: &Engine,
) -> Result<FilterReturnType, FilterError> {
    match original {
        FilterReturnType::String(s) => Ok(FilterReturnType::String(s.to_case(Case::Pascal))),
        FilterReturnType::Rgb(color) => {
            let string =
                format_color(color, keywords.last().expect("Could not get format")).unwrap();
            Ok(FilterReturnType::String(
                string.to_string().to_case(Case::Pascal),
            ))
        }
        FilterReturnType::Hsl(color) => {
            let string =
                format_color_hsl(color, keywords.last().expect("Could not get format")).unwrap();
            Ok(FilterReturnType::String(
                string.to_string().to_case(Case::Pascal),
            ))
        }
        FilterReturnType::Bool(boolean) => match boolean {
            true => Ok(FilterReturnType::String(
                "true".to_string().to_case(Case::Pascal),
            )),
            false => Ok(FilterReturnType::String(
                "false".to_string().to_case(Case::Pascal),
            )),
        },
    }
}

pub(crate) fn snake_case(
    keywords: &[&str],
    _args: &[SpannedValue],
    original: FilterReturnType,
    _engine: &Engine,
) -> Result<FilterReturnType, FilterError> {
    match original {
        FilterReturnType::String(s) => Ok(FilterReturnType::String(s.to_case(Case::Snake))),
        FilterReturnType::Rgb(color) => {
            let string =
                format_color(color, keywords.last().expect("Could not get format")).unwrap();
            Ok(FilterReturnType::String(
                string.to_string().to_case(Case::Snake),
            ))
        }
        FilterReturnType::Hsl(color) => {
            let string =
                format_color_hsl(color, keywords.last().expect("Could not get format")).unwrap();
            Ok(FilterReturnType::String(
                string.to_string().to_case(Case::Snake),
            ))
        }
        FilterReturnType::Bool(boolean) => match boolean {
            true => Ok(FilterReturnType::String(
                "true".to_string().to_case(Case::Snake),
            )),
            false => Ok(FilterReturnType::String(
                "false".to_string().to_case(Case::Snake),
            )),
        },
    }
}

pub(crate) fn kebab_case(
    keywords: &[&str],
    _args: &[SpannedValue],
    original: FilterReturnType,
    _engine: &Engine,
) -> Result<FilterReturnType, FilterError> {
    match original {
        FilterReturnType::String(s) => Ok(FilterReturnType::String(s.to_case(Case::Kebab))),
        FilterReturnType::Rgb(color) => {
            let string =
                format_color(color, keywords.last().expect("Could not get format")).unwrap();
            Ok(FilterReturnType::String(
                string.to_string().to_case(Case::Kebab),
            ))
        }
        FilterReturnType::Hsl(color) => {
            let string =
                format_color_hsl(color, keywords.last().expect("Could not get format")).unwrap();
            Ok(FilterReturnType::String(
                string.to_string().to_case(Case::Kebab),
            ))
        }
        FilterReturnType::Bool(boolean) => match boolean {
            true => Ok(FilterReturnType::String(
                "true".to_string().to_case(Case::Kebab),
            )),
            false => Ok(FilterReturnType::String(
                "false".to_string().to_case(Case::Kebab),
            )),
        },
    }
}
