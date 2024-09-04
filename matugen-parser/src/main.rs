use lexer::Lexer;
use parser::Parser;

mod lexer;
mod node;
mod parser;

fn main() {
    // let source = ".}|aa {{    colors.source_color.default.hex }} {{ aaa }} {{ aaa }} {{ aaaa | set_alpha: 100 }} {{ colors..aaaaaa....a }}";
    let source = ".}|aa {{    colors.source_color.default.hex }} {{ a1.aaa }} {{ a2 }}";

    let mut parser = Parser::new(source);

    // parser.get_keywords();

    println!("----------------------\n");

    let parsed = parser.parse();

    println!("parsed: \n{:#?}", parsed);
}
