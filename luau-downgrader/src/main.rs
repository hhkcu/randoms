mod reconstruct;

use luau_ast_rs::parser::Parser;
use std::fs::read_to_string;
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file: &Path = Path::new(&args[0]);
    if (!file.exists()) {
        eprintln!("File does not exist.");
        std::process::exit(1);
    }
    let source = read_to_string(file).unwrap();
    let ast = Parser::new(&source).parse().unwrap();

}
