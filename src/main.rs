mod ast;
mod config;
mod interpreter;
mod lexer;
mod parser;
mod token;
mod inputs;

use std::env;
use std::fs;
use std::process;

use config::Config;
use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {}", error);
        process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let arguments: Vec<String> = env::args().collect();

    let source_path = arguments
        .get(1)
        .map(String::as_str)
        .unwrap_or("code/main.evs");

    let config_path = arguments
        .get(2)
        .map(String::as_str)
        .unwrap_or("evenus.json");

    let source = fs::read_to_string(source_path)
        .map_err(|error| {
            format!(
                "Could not read source file '{}': {}",
                source_path, error
            )
        })?;

    let config = Config::load(config_path)?;

    let mut lexer = Lexer::new(&source, config);
    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;
    println!("Parsed program: {:#?}", program);
    let mut interpreter = Interpreter::new();
    interpreter.run(&program)?;

    Ok(())
}