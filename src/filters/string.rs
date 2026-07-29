use convert_case::{Case, Casing};

use crate::{
    expect_args,
    parser::{engine::format_color, Engine, FilterError, FilterReturnType, SpannedValue},
};

fn last_keyword<'a>(keywords: &'a [&'a str]) -> Result<&'a str, FilterError> {
    keywords
        .last()
        .copied()
        .ok_or(FilterError::NotEnoughArguments)
}

fn apply_to_string<F>(
    keywords: &[&str],
    original: FilterReturnType,
    op: F,
) -> Result<FilterReturnType, FilterError>
where
    F: Fn(String) -> String,
{
    match original {
        FilterReturnType::String(s) => Ok(FilterReturnType::String(op(s))),
        FilterReturnType::Rgb(color) => {
            let fmt = last_keyword(keywords)?;
            let s = format_color(color, fmt).ok_or(FilterError::NotEnoughArguments)?;
            Ok(FilterReturnType::String(op(s.to_string())))
        }
        FilterReturnType::Hsl(color) => {
            let fmt = last_keyword(keywords)?;
            let s = format_color(color.into(), fmt).ok_or(FilterError::NotEnoughArguments)?;
            Ok(FilterReturnType::String(op(s.to_string())))
        }
        FilterReturnType::Bool(b) => {
            let s = if b { "true" } else { "false" };
            Ok(FilterReturnType::String(op(s.to_string())))
        }
    }
}

pub(crate) fn replace(
    keywords: &[&str],
    args: &[SpannedValue],
    original: FilterReturnType,
    _engine: &Engine,
) -> Result<FilterReturnType, FilterError> {
    let (find, replace) = expect_args!(args, String, String);
    apply_to_string(keywords, original, |s| s.replace(&find, &replace))
}

pub(crate) fn lower_case(
    keywords: &[&str],
    _args: &[SpannedValue],
    original: FilterReturnType,
    _engine: &Engine,
) -> Result<FilterReturnType, FilterError> {
    apply_to_string(keywords, original, |s| s.to_case(Case::Lower))
}

pub(crate) fn camel_case(
    keywords: &[&str],
    _args: &[SpannedValue],
    original: FilterReturnType,
    _engine: &Engine,
) -> Result<FilterReturnType, FilterError> {
    apply_to_string(keywords, original, |s| s.to_case(Case::Camel))
}

pub(crate) fn pascal_case(
    keywords: &[&str],
    _args: &[SpannedValue],
    original: FilterReturnType,
    _engine: &Engine,
) -> Result<FilterReturnType, FilterError> {
    apply_to_string(keywords, original, |s| s.to_case(Case::Pascal))
}

pub(crate) fn snake_case(
    keywords: &[&str],
    _args: &[SpannedValue],
    original: FilterReturnType,
    _engine: &Engine,
) -> Result<FilterReturnType, FilterError> {
    apply_to_string(keywords, original, |s| s.to_case(Case::Snake))
}

pub(crate) fn kebab_case(
    keywords: &[&str],
    _args: &[SpannedValue],
    original: FilterReturnType,
    _engine: &Engine,
) -> Result<FilterReturnType, FilterError> {
    apply_to_string(keywords, original, |s| s.to_case(Case::Kebab))
}
