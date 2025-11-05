use chumsky::span::SimpleSpan;
use colorsys::{ColorAlpha, Hsl, Rgb};
use indexmap::IndexMap;

use crate::parser::{
    engine::{BinaryOperator, Expression, SpannedBinaryOperator, SpannedExpr, Template},
    BinaryOperatorError, Error, FilterError, FilterReturnType, IfError, KeywordError, LoopError,
    ParseErrorKind, SpannedValue, Value,
};

use crate::color::format::{
    format_hex, format_hex_stripped, format_hsl, format_hsla, format_rgb, format_rgba,
};

use super::Engine;

pub const FORMATS: &[&str] = &[
    "hex",
    "hex_stripped",
    "rgb",
    "rgba",
    "hsl",
    "hsla",
    "red",
    "green",
    "blue",
    "alpha",
    "hue",
    "saturation",
    "lightness",
];

pub fn get_str<'a>(source: &'a str, span: &SimpleSpan) -> &'a str {
    &source[span.start..span.end]
}

pub fn get_str_vec<'a>(source: &'a str, spans: &Vec<SimpleSpan>) -> Vec<&'a str> {
    spans
        .iter()
        .map(|s| get_str(source, s))
        .collect::<Vec<&str>>()
}

// TODO: Clean both of these up
pub fn format_color(base_color: Rgb, format: &str) -> Option<String> {
    let hsl_color = Hsl::from(&base_color);

    match format {
        f if FORMATS.contains(&f) => match f {
            "hex" => Some(format_hex(&base_color)),
            "hex_stripped" => Some(format_hex_stripped(&base_color)),
            "rgb" => Some(format_rgb(&base_color)),
            "rgba" => Some(format_rgba(&base_color, false)),
            "hsl" => Some(format_hsl(&hsl_color)),
            "hsla" => Some(format_hsla(&hsl_color, false)),
            "red" => Some(format!("{:?}", base_color.red() as u8)),
            "green" => Some(format!("{:?}", base_color.green() as u8)),
            "blue" => Some(format!("{:?}", base_color.blue() as u8)),
            "alpha" => Some(format!("{:?}", base_color.alpha() as u8)),
            "hue" => Some(format!("{:?}", &hsl_color.hue())),
            "saturation" => Some(format!("{:?}", &hsl_color.lightness())),
            "lightness" => Some(format!("{:?}", &hsl_color.saturation())),
            _ => unreachable!(),
        },
        _ => None,
    }
}

pub fn format_color_hsl(hsl_color: Hsl, format: &str) -> Option<String> {
    let base_color = Rgb::from(&hsl_color);

    match format {
        f if FORMATS.contains(&f) => match f {
            "hex" => Some(format_hex(&base_color)),
            "hex_stripped" => Some(format_hex_stripped(&base_color)),
            "rgb" => Some(format_rgb(&base_color)),
            "rgba" => Some(format_rgba(&base_color, true)),
            "hsl" => Some(format_hsl(&hsl_color)),
            "hsla" => Some(format_hsla(&hsl_color, true)),
            "red" => Some(format!("{:?}", base_color.red() as u8)),
            "green" => Some(format!("{:?}", base_color.green() as u8)),
            "blue" => Some(format!("{:?}", base_color.blue() as u8)),
            "alpha" => Some(format!("{:?}", base_color.alpha() as u8)),
            "hue" => Some(format!("{:?}", &hsl_color.hue())),
            "saturation" => Some(format!("{:?}", &hsl_color.lightness())),
            "lightness" => Some(format!("{:?}", &hsl_color.saturation())),
            _ => unreachable!(),
        },
        _ => None,
    }
}

pub fn format_color_all(base_color: Rgb) -> IndexMap<String, Value> {
    let hsl_color = Hsl::from(&base_color);

    let mut map = IndexMap::new();

    map.insert("hex".to_string(), Value::Ident(format_hex(&base_color)));
    map.insert(
        "hex_stripped".to_string(),
        Value::Ident(format_hex_stripped(&base_color)),
    );
    map.insert("rgb".to_string(), Value::Ident(format_rgb(&base_color)));
    map.insert(
        "rgba".to_string(),
        Value::Ident(format_rgba(&base_color, true)),
    );
    map.insert("hsl".to_string(), Value::Ident(format_hsl(&hsl_color)));
    map.insert(
        "hsla".to_string(),
        Value::Ident(format_hsla(&hsl_color, true)),
    );
    map.insert(
        "red".to_string(),
        Value::Ident(format!("{:?}", base_color.red() as u8)),
    );
    map.insert(
        "green".to_string(),
        Value::Ident(format!("{:?}", base_color.green() as u8)),
    );
    map.insert(
        "blue".to_string(),
        Value::Ident(format!("{:?}", base_color.blue() as u8)),
    );
    map.insert(
        "alpha".to_string(),
        Value::Ident(format!("{:?}", base_color.alpha() as u8)),
    );
    map.insert(
        "hue".to_string(),
        Value::Ident(format!("{:?}", &hsl_color.hue())),
    );
    map.insert(
        "saturation".to_string(),
        Value::Ident(format!("{:?}", &hsl_color.lightness())),
    );
    map.insert(
        "lightness".to_string(),
        Value::Ident(format!("{:?}", &hsl_color.saturation())),
    );

    map
}

impl Engine {
    pub fn generate_template(&self, template: &Template, name: String) -> String {
        self.build_string(&template.ast, &self.sources[template.source_id], &name)
    }

    fn build_string(&self, exprs: &[Box<SpannedExpr>], source: &String, name: &str) -> String {
        let src = &mut String::from("");

        for expr in exprs.iter() {
            let _range = expr.span.into_range();

            self.eval(src, expr, source, name);
        }

        src.to_string()
    }

    fn eval(&self, src: &mut String, expr: &SpannedExpr, source: &String, name: &str) {
        match &expr.expr {
            Expression::Keyword { keywords } => {
                src.push_str(&self.get_value(keywords, source, true, name).to_string());
            }
            Expression::KeywordWithFilters { keyword, filters } => {
                let value = self.get_value(keyword, source, false, name);
                let keywords = keyword.expr.as_keywords(source);

                src.push_str(
                    &self
                        .get_replacement_filter(
                            value.into(),
                            keywords.as_deref(),
                            filters,
                            source,
                            keyword.span,
                            name,
                        )
                        .to_string(),
                );
            }
            Expression::Raw { value } => {
                let str = get_str(source, value);
                src.push_str(str);
            }
            Expression::ForLoop { var, iter, body } => {
                let format_color = true;

                match &iter.expr {
                    //     Expression::KeywordWithFilters { keyword, filters } => todo!(),
                    Expression::Range { start, end } => {
                        for i in *start..*end {
                            self.runtime.borrow_mut().push_scope();

                            self.runtime.borrow_mut().insert("i", Value::Int(i));

                            src.push_str(&self.eval_loop_body(body.clone(), source, name));
                            self.runtime.borrow_mut().pop_scope();
                        }
                    }
                    Expression::Access { keywords: _ } => {
                        let values = match iter.expr.as_keywords(source) {
                            Some(v) => self.resolve_path(v, format_color, expr.span, name),
                            None => unreachable!(),
                        };

                        let Ok(values) = values else {
                            let spans = iter.expr.as_spans().unwrap();
                            let error = Error::ResolveError {
                                span: SimpleSpan::from(
                                    spans.first().unwrap().start..spans.last().unwrap().end,
                                ),
                                name: name.to_string(),
                            };
                            self.errors.add(error);
                            return;
                        };

                        match values {
                            Value::Map(map) => {
                                let res = self.eval_map(map, body, var, source, iter.span, name);
                                src.push_str(&res);
                            }
                            Value::LazyColor { color, scheme: _ } | Value::Color(color) => {
                                let formats = format_color_all(color);
                                let res =
                                    self.eval_map(formats, body, var, source, iter.span, name);
                                src.push_str(&res);
                            }
                            Value::Array(arr) => {
                                for item in arr {
                                    self.runtime.borrow_mut().push_scope();

                                    if var.len() == 1 {
                                        self.runtime
                                            .borrow_mut()
                                            .insert(var[0].value.to_string(), item.clone());
                                    } else {
                                        self.errors.add(Error::ParseError {
                                            kind: ParseErrorKind::Loop(
                                                LoopError::TooManyLoopVariablesArray,
                                            ),
                                            span: iter.span,
                                            name: name.to_string(),
                                        });
                                    }

                                    src.push_str(&self.eval_loop_body(body.clone(), source, name));
                                    self.runtime.borrow_mut().pop_scope();
                                }
                            }
                            _ => {
                                self.errors.add(Error::ParseError {
                                    kind: ParseErrorKind::Loop(
                                        crate::parser::LoopError::LoopOverNonIterableValue,
                                    ),
                                    span: iter.span,
                                    name: name.to_string(),
                                });
                            }
                        }
                    }
                    _ => {}
                }
            }
            Expression::Include { name: include_name } => match &include_name.value {
                Value::Ident(s) => {
                    let template = self.templates.get(s);
                    match template {
                        Some(v) => {
                            let res = self.build_string(&v.ast, &self.sources[v.source_id], s);
                            src.push_str(&res);
                        }
                        None => {
                            let error = Error::IncludeError {
                                span: include_name.span,
                                name: name.to_string(),
                            };
                            self.errors.add(error);
                            return;
                        }
                    };
                }
                _ => {}
            },
            Expression::If { .. } => {
                src.push_str(&self.get_value(expr, source, true, name).to_string());
            }
            Expression::Filter { name: _, args: _ } => unreachable!(),
            Expression::Range { start: _, end: _ } => unreachable!(),
            Expression::LiteralValue { value: _ } => unreachable!(),
            Expression::BinaryOp {
                lhs: _,
                op: _,
                rhs: _,
            } => unreachable!(),
            Expression::Access { keywords: _ } => unreachable!(),
        }
    }

    fn eval_map(
        &self,
        map: IndexMap<String, Value>,
        body: &Vec<Box<SpannedExpr>>,
        var: &Vec<SpannedValue>,
        source: &String,
        span: SimpleSpan,
        name: &str,
    ) -> String {
        let mut output = String::from("");
        for (key, value) in map {
            self.runtime.borrow_mut().push_scope();

            if var.len() == 1 {
                self.runtime
                    .borrow_mut()
                    .insert(var[0].value.to_string(), Value::Ident(key.clone()));
            } else if var.len() == 2 {
                self.runtime
                    .borrow_mut()
                    .insert(var[0].value.to_string(), Value::Ident(key.clone()));
                self.runtime
                    .borrow_mut()
                    .insert(var[1].value.to_string(), value.clone());
            } else {
                self.errors.add(Error::ParseError {
                    kind: ParseErrorKind::Loop(LoopError::TooManyLoopVariables),
                    span: span,
                    name: name.to_string(),
                });
            }

            output.push_str(&self.eval_loop_body(body.clone(), source, name));

            self.runtime.borrow_mut().pop_scope();
        }
        output
    }

    fn eval_loop_body(&self, exprs: Vec<Box<SpannedExpr>>, source: &String, name: &str) -> String {
        let mut output = String::from("");

        for expr in exprs.into_iter() {
            let _range = expr.span.into_range();
            self.eval(&mut output, &expr, source, name);
        }

        output
    }

    fn get_replacement(
        &self,
        keywords: &[&str],
        span: SimpleSpan,
        format_value: bool,
        name: &str,
    ) -> Value {
        match self.resolve_path(keywords.iter().copied(), format_value, span, name) {
            Ok(v) => v,
            Err(e) => {
                self.errors.add(e);

                if keywords[0] == "colors" {
                    Value::Color(Rgb::from_hex_str("#ffffff").unwrap())
                } else {
                    Value::Ident(String::from(""))
                }
            }
        }
    }

    fn replace_binary_op(
        &self,
        lhs: &Box<SpannedExpr>,
        op: SpannedBinaryOperator,
        rhs: &Box<SpannedExpr>,
        source: &String,
        span: SimpleSpan,
        name: &str,
    ) -> Value {
        let left = self.get_value(lhs, source, false, name);
        let right = self.get_value(rhs, source, false, name);

        let left_val = left.get_int();
        let right_val = right.get_int();

        match (left_val, right_val) {
            (Some(l), Some(r)) => self.apply_binary_op(l, r, op.op),
            (l, r) => {
                if l.is_none() | r.is_none() {
                    self.errors.add(Error::ParseError {
                        kind: ParseErrorKind::BinOp(
                            BinaryOperatorError::InvalidBinaryOperatorType {
                                lhs: left.to_string(),
                                op: op.op.to_string(),
                                rhs: right.to_string(),
                            },
                        ),
                        span,
                        name: name.to_string(),
                    });
                }

                Value::Int(0)
            }
        }
    }

    fn apply_binary_op(&self, left: i64, right: i64, op: BinaryOperator) -> Value {
        match op {
            BinaryOperator::Add => left + right,
            BinaryOperator::Sub => left - right,
            BinaryOperator::Mul => left * right,
            BinaryOperator::Div => left / right,
        }
        .into()
    }

    fn get_value(
        &self,
        expr: &SpannedExpr,
        source: &String,
        format_value: bool,
        name: &str,
    ) -> Value {
        match &expr.expr {
            Expression::Keyword { keywords } => {
                self.get_value(&keywords, source, format_value, name)
            }
            Expression::KeywordWithFilters { keyword, filters } => {
                let value = self.get_value(&keyword, source, false, name);
                let keywords = keyword.expr.as_keywords(source);
                Value::from(self.get_replacement_filter(
                    value.into(),
                    keywords.as_deref(),
                    filters,
                    source,
                    keyword.span,
                    name,
                ))
            }
            Expression::LiteralValue { value } => value.value.clone(),
            Expression::Access { keywords } => self.get_replacement(
                &get_str_vec(source, keywords),
                expr.span,
                format_value,
                name,
            ),
            Expression::BinaryOp { lhs, op, rhs } => {
                self.replace_binary_op(lhs, *op, rhs, source, expr.span, name)
            }
            Expression::Raw { value } => Value::Ident(get_str(source, value).to_string()),
            Expression::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let bool = match self.get_value(condition, source, false, name) {
                    Value::Bool(b) => b,
                    _ => {
                        self.errors.add(Error::ParseError {
                            kind: ParseErrorKind::If(IfError::InvalidIfCondition),
                            span: expr.span,
                            name: name.to_string(),
                        });
                        true
                    }
                };
                let mut values = vec![];

                if bool {
                    if format_value {
                        let str = self.build_string(&then_branch, source, name);
                        return Value::Ident(str);
                    } else {
                        for expr in then_branch {
                            values.push(self.get_value(expr, source, format_value, name))
                        }
                    }

                    return Value::Array(values);
                } else {
                    if let Some(exprs) = else_branch {
                        if format_value {
                            let str = self.build_string(&exprs, source, name);
                            return Value::Ident(str);
                        } else {
                            for expr in exprs {
                                values.push(self.get_value(expr, source, format_value, name))
                            }
                        }
                        return Value::Array(values);
                    } else {
                        return Value::Null;
                    };
                }
            }
            _ => {
                dbg!(&expr);
                panic!("");
            }
        }
    }

    fn get_replacement_filter(
        &self,
        mut current_value: FilterReturnType,
        keywords: Option<&[&str]>,
        filters: &[SpannedExpr],
        source: &String,
        span: SimpleSpan,
        name: &str,
    ) -> FilterReturnType {
        let is_color = match &current_value {
            FilterReturnType::Rgb(_) => true,
            FilterReturnType::Hsl(_) => true,
            FilterReturnType::String(_) => false,
            FilterReturnType::Bool(_) => false,
        };

        for filter in filters {
            if let Expression::Filter {
                name: filter_name,
                args,
            } = &filter.expr
            {
                let mut args_resolved = vec![];
                for arg in args {
                    match &arg.expr {
                        Expression::Keyword { keywords } => args_resolved.push(SpannedValue {
                            value: self.get_value(keywords, source, false, name),
                            span: arg.span,
                        }),
                        Expression::KeywordWithFilters { keyword, filters } => {
                            let value = self.get_value(&keyword, source, false, name);
                            let keywords = keyword.expr.as_keywords(source);
                            args_resolved.push(SpannedValue {
                                value: self
                                    .get_replacement_filter(
                                        value.into(),
                                        keywords.as_deref(),
                                        filters,
                                        source,
                                        span,
                                        name,
                                    )
                                    .into(),
                                span: arg.span,
                            });
                        }
                        Expression::LiteralValue { value } => args_resolved.push(value.clone()),
                        Expression::BinaryOp { lhs, op, rhs } => {
                            args_resolved.push(SpannedValue {
                                value: self
                                    .replace_binary_op(lhs, *op, rhs, source, arg.span, name),
                                span: arg.span,
                            });
                        }
                        Expression::If { .. } => {
                            let val = self.get_value(arg, source, false, name);
                            match val {
                                Value::Array(array) => {
                                    for value in array {
                                        args_resolved.push(SpannedValue {
                                            value,
                                            span: arg.span,
                                        })
                                    }
                                }
                                v => {
                                    args_resolved.push(SpannedValue {
                                        value: v,
                                        span: arg.span,
                                    });
                                }
                            }
                        }
                        _ => {
                            panic!("Unsupported filter arg")
                        }
                    }
                }
                current_value = match self.apply_filter(
                    get_str(source, filter_name),
                    &args_resolved,
                    keywords.unwrap_or(&vec![]),
                    current_value,
                    filter.span,
                    name,
                ) {
                    Ok(val) => val,
                    Err(e) => {
                        let error = Error::ParseError {
                            kind: ParseErrorKind::Filter(e),
                            span: filter.span,
                            name: name.to_string(),
                        };
                        self.errors.add(error);

                        match &is_color {
                            false => FilterReturnType::from(String::from("")),
                            true => FilterReturnType::from(Rgb::from_hex_str("#ffffff").unwrap()),
                        }
                    }
                };
            }
        }

        let format = match keywords {
            Some(v) => self.get_format(v),
            None => "hex",
        };

        match current_value {
            FilterReturnType::String(_) => current_value,
            FilterReturnType::Rgb(argb) => match format_color(argb, format) {
                Some(v) => FilterReturnType::String(v.into()),
                None => {
                    let error = Error::ParseError {
                        kind: ParseErrorKind::Keyword(KeywordError::InvalidFormat {
                            formats: FORMATS,
                        }),
                        span,
                        name: name.to_string(),
                    };
                    self.errors.add(error);
                    FilterReturnType::String(String::from(""))
                }
            },
            FilterReturnType::Hsl(hsl) => match format_color_hsl(hsl, format) {
                Some(v) => FilterReturnType::String(v.into()),
                None => {
                    let error = Error::ParseError {
                        kind: ParseErrorKind::Keyword(KeywordError::InvalidFormat {
                            formats: FORMATS,
                        }),
                        span,
                        name: name.to_string(),
                    };
                    self.errors.add(error);
                    FilterReturnType::String(String::from(""))
                }
            },
            FilterReturnType::Bool(_) => current_value,
        }
    }

    fn apply_filter(
        &self,
        filtername: &str,
        args: &[SpannedValue],
        keywords: &[&str],
        input: FilterReturnType,
        span: SimpleSpan,
        name: &str,
    ) -> Result<FilterReturnType, FilterError> {
        match self.filters.get(filtername) {
            Some(f) => f(keywords, args, input, self),
            None => {
                let error = Error::ParseError {
                    kind: ParseErrorKind::Filter(FilterError::FilterNotFound {
                        filter: filtername.to_owned(),
                    }),
                    span,
                    name: name.to_string(),
                };
                self.errors.add(error);
                Ok(FilterReturnType::from(
                    // This is a color so that when you chain filters that aren't found, it wont return FilterError::ColorFilterOnString
                    Rgb::from_hex_str("#ffffff").unwrap(),
                ))
            }
        }
    }
}
