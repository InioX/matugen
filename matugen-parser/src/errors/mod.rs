use parse::ParseError;

pub mod parse;

pub fn handle_error<T>(f: Result<T, ParseError<'_>>) -> Option<ParseError> {
    if let Err(e) = f {
        std::eprintln!("{}", e);
        Some(e)
    } else {
        None
    }
}

pub fn handle_error_panic<T>(f: Result<T, ParseError<'_>>) {
    if let Err(ref e) = f {
        panic!("{}", e);
    };
}
