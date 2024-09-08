use lexer::Lexer;
use parser::Parser;

mod lexer;
mod node;
mod parser;

fn main() {
    // let source = ".}|aa {{    colors.source_color.default.hex }} {{ aaa }} {{ aaa }} {{ aaaa | set_alpha: 100 }} {{ colors..aaaaaa....a }}";
    // let source = ".}|aa {{    colors.source_color.default.hex }} {{ a1.aaa }} {{ a2 }}";
    //     let source = r#"

    // ahskldflsdflksdfjlkdfjlksdfjlsdkfj
    // textxatetaxtetxaata
    // {F}[ASF}ADS{F}DSF]

    // {{
    //      aaa. a.
    //        aaa
    // }

    // asffafdsafsdarrararaeraeraaeraesaeasfsdf

    // {{ doubledot..aaa }}

    // {{ colors.aaa.default.hex | thingy }}
    // {{ colors.aaa.default.hex | thingy: aaaa }}
    // "#;

    let source = r#"{{ colors colors }}

{{ colors.source_color.default.hex }}

{{ image }}"#;

    let mut parser = Parser::new(source, "filenameeeeeeee.txt");

    // parser.get_keywords();

    println!("----------------------\n");

    let parsed = parser.parse();

    println!("parsed: \n{:#?}", parsed);
}
