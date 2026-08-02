use std::io;
use crate::interpreter::Value;

pub fn read_input(prompt: &Value) -> String {

    match prompt {
            Value::Number(number) => {
                println!("{}", number);
            }

            Value::String(text) => {
                println!("{}", text);
            }
            Value::Boolean(boolean) => {
                println!("{}", boolean);
            }
            }
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
