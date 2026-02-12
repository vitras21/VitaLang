use std::env;
use std::fmt;
use std::fs;
use std::collections::HashMap;
mod lexer;
mod parser;

#[derive(Debug)]
struct Context();

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "There is Context. Definitely.");
    }
}

fn context() -> Result<(), Context> {
    return Err(Context());
}

pub fn fail() -> ! {
    if let Err(e) = context() {
        eprintln!("\x1b[31m{}\x1b[0m", e);
    }
    std::process::exit(1);
}

impl std::error::Error for Context {}

fn main() {
    let mut precedence_map: HashMap<&'static str, usize> = HashMap::new();

    precedence_map.insert("^^", 4);
    precedence_map.insert("^", 3);
    precedence_map.insert("*", 2); precedence_map.insert("/", 2);
    precedence_map.insert("+", 1); precedence_map.insert("-", 1);
    precedence_map.insert("<", 0); precedence_map.insert(">", 0); precedence_map.insert("=", 0); precedence_map.insert("≥", 0); precedence_map.insert("≤", 0);

    let args: Vec<String> = env::args().collect();

    let script_name = &args[1];

    let script = fs::read_to_string(script_name);

    let tokens = lexer::tokenize(&script.unwrap());

    // for token in &tokens {
    //     println!("{}", token);
    // }

    let ast = parser::Parser::new(tokens, 0, precedence_map).parse();
    println!("AST: {:?}", ast);
}
