use crate::engine::Value;

#[macro_export]
macro_rules! expect_args {
    ($args:expr, $( $ty:ty ),* $(,)?) => {{
        let expected_len = [$(
            stringify!($ty)
        ),*].len();
        if $args.len() < expected_len {
            return Err($crate::engine::FilterError::new(
                $crate::engine::FilterErrorKind::NotEnoughArguments,
            ));
        }

        let mut _i = 0;
        (
            $(
                {
                    let spanned = &$args[_i];
                    _i += 1;
                    match <$ty as $crate::engine::helpers::ExpectFromValue>::expect_from(&spanned.value) {
                        Ok(v) => v,
                        Err(actual) => {
                            return Err($crate::engine::FilterError::new(
                                $crate::engine::FilterErrorKind::InvalidArgumentType {
                                    span: spanned.span,
                                    expected: stringify!($ty).to_string(),
                                    actual,
                                }
                            ));
                        }
                    }
                }
            ),*
        )
    }};
}

pub trait ExpectFromValue: Sized {
    fn expect_from(value: &Value) -> Result<Self, String>;
}

impl ExpectFromValue for String {
    fn expect_from(value: &Value) -> Result<Self, String> {
        match value {
            Value::Ident(s) => Ok(s.clone()),
            other => Err(other.variant_name()),
        }
    }
}

impl ExpectFromValue for i64 {
    fn expect_from(value: &Value) -> Result<Self, String> {
        match value {
            Value::Int(i) => Ok(*i),
            other => Err(other.variant_name()),
        }
    }
}

impl ExpectFromValue for f64 {
    fn expect_from(value: &Value) -> Result<Self, String> {
        match value {
            Value::Float(f) => Ok(*f),
            Value::Int(i) => Ok(*i as f64),
            other => Err(other.variant_name()),
        }
    }
}
