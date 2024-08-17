use std::process::Stdio;

use execute::{shell, Execute};
use upon::{Engine, Syntax, Template, Value};

use crate::color::color::color_to_string;

pub fn format_hook(
    engine: &Engine,
    render_data: &mut Value,
    hook: &String,
    colors_to_compare: &Option<Vec<crate::color::color::ColorDefinition>>,
    compare_to: &Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let closest_color: Option<String> = if colors_to_compare.is_some() && compare_to.is_some() {
        let s = engine.compile(compare_to.as_ref().unwrap())?;
        let compare_to = s.render(engine, &render_data).to_string()?;
        Some(color_to_string(
            colors_to_compare.as_ref().unwrap(),
            &compare_to,
        ))
    } else {
        None
    };

    let t = engine.compile(hook)?;
    let res = if colors_to_compare.is_some() && compare_to.is_some() {
        format_hook_text(render_data, closest_color, t)
    } else {
        format_hook_text(render_data, None, t)
    };

    let mut command = shell(&res);

    command.stdout(Stdio::inherit());

    let output = command.execute_output()?;

    if let Some(exit_code) = output.status.code() {
        if exit_code != 0 {
            error!("Failed executing command: {:?}", &res)
        }
    } else {
        eprintln!("Interrupted!");
    }

    Ok(())
}

pub fn format_hook_text(
    render_data: &mut Value,
    closest_color: Option<String>,
    template: Template<'_>,
) -> String {
    let syntax = Syntax::builder().expr("{{", "}}").block("<*", "*>").build();
    let engine = Engine::with_syntax(syntax);

    match render_data {
        Value::Map(ref mut map) => {
            map.insert("closest_color".to_string(), Value::from(closest_color));
        }
        _ => {
            debug!("not map")
        }
    }

    let data = template.render(&engine, &render_data).to_string().unwrap();

    data
}
