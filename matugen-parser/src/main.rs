use core::panic;
use std::path::PathBuf;

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

    let args: Vec<String> = std::env::args().collect();

    if args.len() <= 1 {
        panic!("Must provide file path");
    }

    let file_path = PathBuf::from(&args[1]);
    let binding = std::fs::canonicalize(&file_path).unwrap();
    let file_path_absolute = binding.as_os_str().to_str().unwrap();

    // let file_path = "./matugen-parser/example/template.txt";
    let src = std::fs::read_to_string(&file_path).unwrap();

    let mut parser = Parser::new(&src, file_path_absolute);

    // parser.get_keywords();

    println!("----------------------\n");

    let parsed = parser.parse();

    println!("parsed: \n{:#?}", parsed);
}
