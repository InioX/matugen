use std::collections::HashMap;

use chumsky::{span::SimpleSpan, Parser};
use colorsys::{ColorAlpha, Hsl, Rgb};

use crate::parser::{
    engine::{BinaryOperator, Expression, SpannedBinaryOperator, SpannedExpr, Template},
    Error, FilterError, FilterReturnType, KeywordError, ParseErrorKind, SpannedValue, Value,
};

use crate::color::format::{
    format_hex, format_hex_stripped, format_hsl, format_hsla, format_rgb, format_rgba,
    rgb_from_argb,
};

use super::Engine;

pub fn get_str<'a>(source: &'a str, span: &SimpleSpan) -> &'a str {
    &source[span.start..span.end]
}

pub fn get_str_vec<'a>(source: &'a str, spans: &Vec<SimpleSpan>) -> Vec<&'a str> {
    spans
        .iter()
        .map(|s| get_str(source, s))
        .collect::<Vec<&str>>()
}

pub fn format_color(base_color: Rgb, format: &str) -> Option<impl Into<String>> {
    let hsl_color = Hsl::from(&base_color);

    match format {
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
        _ => None,
    }
}

pub fn format_color_hsl(hsl_color: Hsl, format: &str) -> Option<impl Into<String>> {
    let base_color = Rgb::from(&hsl_color);

    match format {
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
        _ => None,
    }
}

pub fn format_color_all(base_color: Rgb) -> HashMap<String, Value> {
    let hsl_color = Hsl::from(&base_color);

    let mut map = HashMap::new();

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
    pub fn generate_template(&self, template: &Template) -> String {
        self.build_string(&template.ast, &self.sources[template.source_id])
    }

    fn build_string(&self, exprs: &[Box<SpannedExpr>], source: &String) -> String {
        let src = &mut String::from("");

        for expr in exprs.iter() {
            let _range = expr.span.into_range();

            self.eval(src, expr, source);
        }

        src.to_string()
    }

    fn eval(&self, src: &mut String, expr: &SpannedExpr, source: &String) {
        match &expr.expr {
            Expression::Keyword { keywords } => {
                let format_value = true;
                let value: String = self
                    .get_replacement(&get_str_vec(source, keywords), expr.span, format_value)
                    .into();
                src.push_str(&value);
            }
            Expression::KeywordWithFilters { keyword, filters } => {
                let keywords = match &keyword.expr {
                    Expression::Keyword { keywords } => &get_str_vec(source, keywords),
                    _ => panic!(""),
                };

                src.push_str(
                    &self
                        .get_replacement_filter(keywords, filters, source, keyword.span)
                        .to_string(),
                );
            }
            Expression::Raw { value } => {
                src.push_str(get_str(source, value));
            }
            Expression::ForLoop { var, iter, body } => {
                let format_color = true;

                match &iter.expr {
                    //     Expression::KeywordWithFilters { keyword, filters } => todo!(),
                    Expression::Range { start, end } => {
                        for i in *start..*end {
                            self.runtime.borrow_mut().push_scope();

                            self.runtime.borrow_mut().insert("i", Value::Int(i));

                            src.push_str(&self.eval_loop_body(body.clone(), source));
                            self.runtime.borrow_mut().pop_scope();
                        }
                    }
                    Expression::Keyword { keywords } => {
                        let values = match iter.expr.as_keywords(source) {
                            Some(v) => self.resolve_path(v, format_color),
                            None => unreachable!(),
                        };

                        let Some(values) = values else {
                            let spans = iter.expr.as_spans().unwrap();
                            let error = Error::ResolveError {
                                span: SimpleSpan::from(
                                    spans.first().unwrap().start..spans.last().unwrap().end,
                                ),
                            };
                            self.errors.add(error);
                            return;
                        };

                        match values {
                            Value::Map(map) => {
                                for (key, value) in map {
                                    self.runtime.borrow_mut().push_scope();

                                    if var.len() == 1 {
                                        self.runtime.borrow_mut().insert(
                                            var[0].value.clone(),
                                            Value::Ident(key.clone()),
                                        );
                                    } else if var.len() == 2 {
                                        self.runtime.borrow_mut().insert(
                                            var[0].value.clone(),
                                            Value::Ident(key.clone()),
                                        );
                                        self.runtime
                                            .borrow_mut()
                                            .insert(var[1].value.clone(), value.clone());
                                    } else {
                                        panic!("for-loop supports only one or two variables");
                                    }

                                    // Evaluate the body with these bindings
                                    src.push_str(&self.eval_loop_body(body.clone(), source));

                                    self.runtime.borrow_mut().pop_scope();
                                }
                            }
                            Value::Array(arr) => {
                                for item in arr {
                                    self.runtime.borrow_mut().push_scope();

                                    if var.len() == 1 {
                                        self.runtime
                                            .borrow_mut()
                                            .insert(var[0].value.clone(), item.clone());
                                    } else {
                                        panic!("for-loop over list supports only one variable");
                                    }

                                    src.push_str(&self.eval_loop_body(body.clone(), source));
                                    self.runtime.borrow_mut().pop_scope();
                                }
                            }
                            _ => {
                                panic!("Cannot loop over non-iterable value");
                            }
                        }
                    }
                    _ => {}
                }
            }
            Expression::Include { name } => match &name.value {
                Value::Ident(s) => {
                    let template = self.templates.get(s);
                    match template {
                        Some(v) => {
                            let res = self.build_string(&v.ast, &self.sources[v.source_id]);
                            src.push_str(&res);
                        }
                        None => {
                            let error = Error::IncludeError { span: name.span };
                            self.errors.add(error);
                            return;
                        }
                    };
                }
                _ => {}
            },
            Expression::If {
                condition,
                then_branch,
                else_branch,
            } => {
                match &condition.expr {
                    Expression::Keyword { keywords } => {
                        let res = self.get_replacement_filter(
                            &get_str_vec(source, keywords),
                            &vec![],
                            source,
                            condition.span,
                        );
                        match res {
                            FilterReturnType::Bool(boolean) => {
                                if boolean {
                                    let str = self.build_string(&then_branch, source);
                                    src.push_str(&str);
                                } else {
                                    if let Some(else_branch) = else_branch {
                                        let str = self.build_string(&else_branch, source);
                                        src.push_str(&str);
                                    }
                                }
                            }
                            _ => {
                                println!("You can only use if with booleans1");
                                return;
                            }
                        }
                    }
                    _ => {
                        // TODO: ERROR
                        println!("You can only use if with booleans2")
                    }
                }
            }
            Expression::Filter { name: _, args: _ } => unreachable!(),
            Expression::Range { start: _, end: _ } => unreachable!(),
            Expression::LiteralValue { value: _ } => unreachable!(),
            Expression::BinaryOp { lhs, op, rhs } => todo!(),
        }
    }

    fn eval_loop_body(&self, exprs: Vec<Box<SpannedExpr>>, source: &String) -> String {
        let mut output = String::from("");

        for expr in exprs.into_iter() {
            let _range = expr.span.into_range();
            self.eval(&mut output, &expr, source);
        }

        output
    }

    fn get_replacement(&self, keywords: &[&str], span: SimpleSpan, format_value: bool) -> Value {
        if keywords[0] == "colors" {
            let (r#type, name, colorscheme, format) = self.get_color_parts(keywords, span);
            let color = rgb_from_argb(*self.get_from_map(r#type, name, colorscheme, span));

            if !format_value {
                return Value::Color(color);
            }

            match format_color(color, self.get_format(keywords)) {
                Some(v) => Value::Ident(v.into()),
                None => {
                    let error = Error::ParseError {
                        kind: ParseErrorKind::Keyword(KeywordError::InvalidFormat),
                        span,
                    };
                    self.errors.add(error);
                    Value::Ident(String::from(""))
                }
            }
        } else {
            match self.resolve_path(keywords.iter().copied(), format_value) {
                Some(v) => {
                    if format_value {
                        Value::Ident(String::from(v))
                    } else {
                        v
                    }
                }
                None => {
                    self.errors.add(Error::ResolveError { span });
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
    ) -> Value {
        let left = self.get_value(lhs, source).get_int();
        let right = self.get_value(rhs, source).get_int();

        match (left, right) {
            (Some(l), Some(r)) => self.apply_binary_op(l, r, op.op),
            _ => {
                panic!("TODO")
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

    fn get_value(&self, expr: &SpannedExpr, source: &String) -> Value {
        match &expr.expr {
            Expression::Keyword { keywords } => {
                let format_value = false;
                self.get_replacement(&get_str_vec(source, keywords), expr.span, format_value)
            }
            Expression::KeywordWithFilters { keyword, filters } => {
                let keywords = match &keyword.expr {
                    Expression::Keyword { keywords } => &get_str_vec(source, keywords),
                    _ => panic!(""),
                };
                Value::from(self.get_replacement_filter(keywords, filters, source, keyword.span))
            }
            Expression::LiteralValue { value } => value.value.clone(),
            _ => {
                panic!("");
            }
        }
    }

    fn get_replacement_filter(
        &self,
        keywords: &[&str],
        filters: &[SpannedExpr],
        source: &String,
        span: SimpleSpan,
    ) -> FilterReturnType {
        let format_color_string = false;
        let mut current_value = if keywords[0] == "colors" {
            match self.resolve_path(keywords.iter().copied(), format_color_string) {
                Some(v) => FilterReturnType::from(v),
                None => {
                    self.errors.add(Error::ResolveError { span });
                    FilterReturnType::from(Rgb::from_hex_str("#ffffff").unwrap())
                }
            }
        } else {
            match self.resolve_path(keywords.iter().copied(), format_color_string) {
                Some(v) => FilterReturnType::from(v),
                None => {
                    self.errors.add(Error::ResolveError { span });
                    FilterReturnType::from(String::from(""))
                }
            }
        };

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
                let format_value = false;
                let mut args_resolved = vec![];
                for arg in args {
                    match &arg.expr {
                        Expression::Keyword { keywords } => args_resolved.push(SpannedValue {
                            value: self.get_replacement(
                                &get_str_vec(source, keywords),
                                span,
                                false,
                            ),
                            span: arg.span,
                        }),
                        Expression::KeywordWithFilters { keyword, filters } => {
                            let keywords = match &keyword.expr {
                                Expression::Keyword { keywords } => &get_str_vec(source, keywords),
                                _ => panic!(""),
                            };
                            args_resolved.push(SpannedValue {
                                value: Value::from(
                                    self.get_replacement_filter(keywords, filters, source, span),
                                ),
                                span: arg.span,
                            });
                        }
                        Expression::LiteralValue { value } => args_resolved.push(value.clone()),
                        Expression::BinaryOp { lhs, op, rhs } => {
                            args_resolved.push(SpannedValue {
                                value: self.replace_binary_op(lhs, *op, rhs, source),
                                span: arg.span,
                            });
                        }
                        _ => {
                            panic!("Unsupported filter arg")
                        }
                    }
                }
                current_value = match self.apply_filter(
                    get_str(source, filter_name),
                    &args_resolved,
                    keywords,
                    current_value,
                    filter.span,
                ) {
                    Ok(val) => val,
                    Err(e) => {
                        let error = Error::ParseError {
                            kind: ParseErrorKind::Filter(e),
                            span: filter.span,
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

        match current_value {
            FilterReturnType::String(_) => current_value,
            FilterReturnType::Rgb(argb) => match format_color(argb, self.get_format(keywords)) {
                Some(v) => FilterReturnType::String(v.into()),
                None => {
                    let error = Error::ParseError {
                        kind: ParseErrorKind::Keyword(KeywordError::InvalidFormat),
                        span,
                    };
                    self.errors.add(error);
                    FilterReturnType::String(String::from(""))
                }
            },
            FilterReturnType::Hsl(hsl) => match format_color_hsl(hsl, self.get_format(keywords)) {
                Some(v) => FilterReturnType::String(v.into()),
                None => {
                    let error = Error::ParseError {
                        kind: ParseErrorKind::Keyword(KeywordError::InvalidFormat),
                        span,
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
    ) -> Result<FilterReturnType, FilterError> {
        match self.filters.get(filtername) {
            Some(f) => f(keywords, args, input, self),
            None => {
                let error = Error::ParseError {
                    kind: ParseErrorKind::Filter(FilterError::FilterNotFound {
                        filter: filtername.to_owned(),
                    }),
                    span,
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
