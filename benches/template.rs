use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};

use matugen::{scheme::{get_custom_color_schemes, get_schemes, SchemesEnum}, template_util::template::{self, get_render_data, render_template}};
use template::add_engine_filters;
use upon::{Engine, Syntax};

fn parse_template(data: &str) {
    let source_color = material_colors::color::Argb::new(255, 255, 0, 0);

    let syntax = Syntax::builder().expr("{{", "}}").block("<*", "*>").build();
    let mut engine = Engine::with_syntax(syntax);

    add_engine_filters(&mut engine);

    let (scheme_dark, scheme_light) = get_schemes(source_color, &None, &None);
    let schemes = get_custom_color_schemes(
        source_color,
        scheme_dark,
        scheme_light,
        &None,
        &None,
        &None
    );
    let render_data = get_render_data(&schemes, &source_color,&SchemesEnum::Dark, &None, None).unwrap();

    engine.add_template("a", data.repeat(50)).expect("failed to add template");
    render_template(&engine, &"a".to_string(), &render_data, None).expect("failed to render template");
}

fn criterion_benchmark(c: &mut Criterion) {
    let data = 
    r#"
    <* for name, value in colors *>
        {{name}} {{value.default.rgba}};
    <* endfor *>
    "#;
    let data_filter = 
    r#"
    <* for name, value in colors *>
        {{name | replace: "_", "-" }} {{value.default.rgba | set_alpha: 0.7 | set_hue: -180.0 }};
    <* endfor *>
    "#;

    c.bench_function("parse 20", |b| b.iter(|| parse_template(black_box(&data.repeat(20)))));
    c.bench_function("parse 20 filters", |b| b.iter(|| parse_template(black_box(&data_filter.repeat(20)))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);