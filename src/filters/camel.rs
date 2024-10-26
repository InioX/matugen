use upon::Value;

use crate::color::parse::check_string_value;

pub fn camel_case(value: &Value) -> Result<String, String> {
    let string = check_string_value(value).unwrap();

    let mut result = String::new();
    let mut capitalize_next = false;

    for c in string.chars() {
        if c == '_' {
            capitalize_next = true;
        } else {
            if capitalize_next {
                result.push(c.to_uppercase().next().unwrap());
                capitalize_next = false;
            } else {
                result.push(c);
            }
        }
    }

    debug!("Converting to camelCase: {} to {}", string, result);

    Ok(result)
}
