use upon::{Engine, Syntax, Template, Value};

// use regex::{Captures, Regex};
// use material_colors::color::Argb;
// use super::color::{format_hex, rgb_from_argb};

// pub enum Variables {
//     Invalid,
//     ComparedColor,
//     SourceImage,
//     SourceColor,
// }

// impl Variables {
//     fn from(mut input: &str) -> Self {
//         if input.starts_with("{") && input.ends_with("}") {
//             input = input.remove_first_char().remove_last_char();
//         }

//         match input {
//             "closest_color" => Variables::ComparedColor,
//             "source_image" => Variables::SourceImage,
//             "source_color" => Variables::SourceColor,
//             _ => {
//                 error!("Invalid variable: {{{}}}", input);
//                 Variables::Invalid
//             }
//         }
//     }
// }

// trait StrExt {
//     fn remove_last_char(&self) -> &str;
//     fn remove_first_char(&self) -> &str;
// }

// impl StrExt for str {
//     fn remove_last_char(&self) -> &str {
//         match self.char_indices().next_back() {
//             Some((i, _)) => &self[..i],
//             None => self,
//         }
//     }
//     fn remove_first_char(&self) -> &str {
//         self.chars()
//             .next()
//             .map(|c| &self[c.len_utf8()..])
//             .unwrap_or("")
//     }
// }

// pub fn replace_hook_keywords(
//     input: &str,
//     default_value: &String,
//     src_img: Option<&String>,
//     closest_color: Option<&String>,
//     source_color: &Argb,
// ) -> String {
//     let re = Regex::new(r"\{.*?\}").unwrap();

//     let source_formatted = format_hex(&rgb_from_argb(*source_color));

//     let result = re.replace_all(input, |cap: &Captures| {
//         match Variables::from(&cap[0]) {
//             Variables::Invalid => &cap[0],
//             Variables::ComparedColor => closest_color.unwrap_or(default_value),
//             Variables::SourceImage => src_img.unwrap_or(default_value),
//             Variables::SourceColor => &source_formatted,
//         }
//         .to_string()
//     });

//     return result.to_string();
// }

pub fn format_hook_text(render_data: &mut Value, closest_color: Option<String>, template: Template<'_>) -> String {
    let syntax = Syntax::builder().expr("{{", "}}").block("<*", "*>").build();
    let mut engine = Engine::with_syntax(syntax);
    
    match render_data {
        Value::Map(ref mut map) => {
                map.insert("closest_color".to_string(), Value::from(closest_color));
        },
        _ => {
            debug!("not map")
        }
    }

    let data = template
    .render(&engine,&render_data)
    .to_string().unwrap();

    return data
}