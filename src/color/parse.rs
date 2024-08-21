use upon::Value;

pub fn parse_color(string: &str) -> Option<&str> {
    if let Some(_s) = string.strip_prefix('#') {
        return Some("hex");
    }

    if let (Some(i), Some(s)) = (string.find('('), string.strip_suffix(')')) {
        let fname = s[..i].trim_end();
        Some(fname)
    } else if string.len() == 6 {
        // Does not matter if it is actually a stripped hex or not, we handle it somewhere else.
        return Some("hex_stripped");
    } else {
        None
    }
}

pub fn check_string_value(value: &Value) -> Option<&String> {
    match value {
        Value::String(v) => Some(v),
        _v => None,
    }
}
