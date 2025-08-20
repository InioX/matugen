use chumsky::{error::Rich, prelude::*, span::SimpleSpan};

use crate::parser::{
    engine::{BinaryOperator, EngineSyntax, Expression, SpannedBinaryOperator, SpannedExpr},
    Engine, SpannedValue, Value,
};

impl Engine {
    pub fn parser<'src>(
        syntax: &'src EngineSyntax,
    ) -> impl Parser<'src, &'src str, Vec<Box<SpannedExpr>>, extra::Err<Rich<'src, char>>> {
        recursive(|expr| {
            // Dotted identifier as a sequence of spans
            let numeric_key_prefixed = just('_').ignore_then(text::int(10));

            let dotted_ident = text::ident()
                .or(numeric_key_prefixed)
                .map_with(|_, e| e.span())
                .separated_by(just('.').padded())
                .at_least(1)
                .collect::<Vec<SimpleSpan>>()
                .map_with(|spans, e| {
                    Box::new(SpannedExpr {
                        expr: Expression::Access { keywords: spans },
                        span: e.span(),
                    })
                });

            let plain_ident = text::ident().map(|s: &str| Value::Ident(s.to_string()));

            let escape = just('\\').ignore_then(just('"').or(just('\\')));

            let inner = escape
                .or(none_of("\"\\"))
                .repeated()
                .collect::<String>()
                .map(Value::Ident);

            let quoted_ident = inner.delimited_by(just('"'), just('"'));

            let ident = quoted_ident.or(plain_ident);
            // let ident = quoted_ident;

            let sign = just('-').or(just('+')).or_not();

            let int = sign
                .then(text::int(10))
                .map(|(sign, digits): (Option<char>, &str)| {
                    let number = format!("{}{}", sign.unwrap_or('+'), digits);
                    Value::Int(number.parse::<i64>().unwrap())
                });

            let float = sign
                .then(text::int(10)) // int part
                .then_ignore(just('.'))
                .then(text::int(10)) // frac part
                .map(
                    |((sign, int_part), frac_part): ((Option<char>, &str), &str)| {
                        let number = format!("{}{}.{}", sign.unwrap_or('+'), int_part, frac_part);
                        Value::Float(number.parse::<f64>().unwrap())
                    },
                );

            let range = int
                .then_ignore(just(".."))
                .then(int)
                .map_with(|(start, end), e| {
                    Box::new(SpannedExpr {
                        expr: Expression::Range {
                            start: start.get_int().expect("Failed to get int from range"),
                            end: end.get_int().expect("Failed to get int from range"),
                        },
                        span: e.span(),
                    })
                });

            let boolean = just("true")
                .to(Value::Bool(true))
                .or(just("false").to(Value::Bool(false)));

            let spanned_ident = ident.map_with(|value, e| SpannedValue::new(value, e.span()));

            let literal = float.or(int).or(ident).or(boolean).map_with(|value, e| {
                Box::new(SpannedExpr {
                    expr: Expression::LiteralValue {
                        value: SpannedValue {
                            value,
                            span: e.span(),
                        },
                    },
                    span: e.span(),
                })
            });

            let op = just('+')
                .to(BinaryOperator::Add)
                .or(just('-').to(BinaryOperator::Sub))
                .or(just('*').to(BinaryOperator::Mul))
                .or(just('/').to(BinaryOperator::Div))
                .map_with(|op, e| SpannedBinaryOperator { op, span: e.span() });

            let arg = literal
                .clone()
                .or(expr.clone())
                .padded()
                .clone()
                .then(op.padded())
                .then(literal.clone().or(expr.clone()).padded())
                .map_with(|((a, b), c), e| {
                    Box::new(SpannedExpr {
                        expr: Expression::BinaryOp {
                            lhs: a,
                            op: b,
                            rhs: c,
                        },
                        span: e.span(),
                    })
                });

            let filter = text::ident()
                .map_with(|_, e| e.span())
                .then(
                    just(':')
                        .padded()
                        .ignore_then(
                            literal
                                .clone()
                                .or(arg.clone())
                                .or(expr.clone())
                                .padded()
                                .separated_by(just(',').padded())
                                .collect::<Vec<Box<SpannedExpr>>>(),
                        )
                        .or_not(),
                )
                .map_with(
                    |(name, args), e: &mut chumsky::input::MapExtra<'_, '_, _, _>| SpannedExpr {
                        expr: Expression::Filter {
                            name,
                            args: args.unwrap_or_default(),
                        },
                        span: e.span(),
                    },
                );

            let filters = just('|')
                .padded()
                .ignore_then(filter.padded())
                .repeated()
                .collect::<Vec<_>>();

            let full_expr = dotted_ident
                .or(literal.clone())
                .or(arg)
                .or(expr.clone())
                .padded()
                .then(filters)
                .map(|(access, filters)| {
                    let keyword = SpannedExpr {
                        span: access.span.clone(),
                        expr: Expression::Keyword { keywords: access },
                    };
                    if filters.is_empty() {
                        keyword
                    } else {
                        let span = SimpleSpan::new(
                            (),
                            keyword.span.start
                                ..filters
                                    .last()
                                    .map(|f| f.span.end)
                                    .unwrap_or(keyword.span.end),
                        );
                        SpannedExpr {
                            expr: Expression::KeywordWithFilters {
                                keyword: Box::new(keyword),
                                filters,
                            },
                            span,
                        }
                    }
                });

            let keyword_full = full_expr
                .padded()
                .delimited_by(just(syntax.keyword_left), just(syntax.keyword_right))
                .map_with(|expr, e| {
                    Box::new(SpannedExpr {
                        expr: expr.expr,
                        span: e.span(),
                    })
                });

            let raw = any()
                .and_is(just(syntax.keyword_left).or(just(syntax.block_left)).not())
                .repeated()
                .at_least(1)
                .collect::<String>()
                .map_with(|_, span| {
                    Box::new(SpannedExpr {
                        expr: Expression::Raw { value: span.span() },
                        span: span.span(),
                    })
                });

            let include = just("include")
                .padded()
                .ignore_then(spanned_ident.padded())
                .delimited_by(just(syntax.block_left), just(syntax.block_right))
                .map_with(|name, e| {
                    Box::new(SpannedExpr {
                        expr: Expression::Include { name },
                        span: e.span(),
                    })
                });

            let if_statement = just("if")
                .padded()
                .ignore_then(keyword_full.clone().padded())
                .then_ignore(just(syntax.block_right).padded())
                .then(raw.or(expr.clone()).repeated().collect())
                .then(
                    just(syntax.block_left)
                        .padded()
                        .ignore_then(just("else").padded())
                        .ignore_then(just(syntax.block_right).padded())
                        .ignore_then(raw.or(expr.clone()).repeated().collect())
                        .or_not(),
                )
                .delimited_by(
                    just(syntax.block_left),
                    just("endif")
                        .padded()
                        .delimited_by(just(syntax.block_left), just(syntax.block_right)),
                )
                .map_with(|((condition, then_branch), else_branch), e| {
                    Box::new(SpannedExpr {
                        expr: Expression::If {
                            condition: condition,
                            then_branch: then_branch,
                            else_branch: else_branch,
                        },
                        span: e.span(),
                    })
                });

            let for_loop = just("for")
                .padded()
                .ignore_then(
                    spanned_ident
                        .separated_by(just(',').padded())
                        .at_least(1)
                        .collect::<Vec<SpannedValue>>(),
                )
                .padded()
                .then_ignore(just("in").padded())
                .then(dotted_ident.or(range).padded())
                .then_ignore(just(syntax.block_right))
                .then(raw.or(expr).repeated().collect())
                .delimited_by(
                    just(syntax.block_left),
                    just("endfor")
                        .padded()
                        .delimited_by(just(syntax.block_left), just(syntax.block_right)),
                )
                .map_with(|((var, iter), body), e| {
                    Box::new(SpannedExpr {
                        expr: Expression::ForLoop { var, iter, body },
                        span: e.span(),
                    })
                });

            choice((raw, keyword_full, for_loop, if_statement, include))
        })
        .repeated()
        .collect::<Vec<Box<SpannedExpr>>>()
    }
}
