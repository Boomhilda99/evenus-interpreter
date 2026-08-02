# Evenus

**Evenus** is an interpreted programming language written in **Rust** where the syntax itself is configurable. Rather than forcing one set of keywords, Evenus allows you to customise the language through an `evenus.json` configuration file, making it possible to create a language that feels natural to you.

> ⚠️ Evenus is currently in active development. Features and syntax may change as the language evolves.

## Features

* 🔧 Configurable language keywords
* ⚡ Written in Rust
* 📝 Variables
* ➕ Arithmetic expressions with operator precedence
* 💬 String literals
* 🔢 Number literals
* 🌳 Lexer → Parser → AST → Interpreter architecture

## Example
Using this config
```json
{
    "keywords": {
        "print": "output",
        "variable": "let",
        "if_statement": "if",
        "else_statement": "but",
        "inf_loop": "loop",
        "for_loop": "for",
        "not": "opposite",
        "function": "fn",
        "input": "ask"
    },
    "syntax": {
        "indent": true
    }
}
```
Creates this syntax
```evenus
let userPassword = ask("What is the admin password? ");
let age = ask("What is your age? ");

let adminPass = "Yilti";
if userPassword == adminPass {
    if age == "100" {
        output("Access Granted");
    } but {
        output("Access Denied.");
    }
} but {
    output("Access Denied.");
}
```


## Configurable Syntax

Evenus is designed so that language keywords can be customised.
I have another github repo with a config script helper, please check that out if you are confused.

Because of this, the same interpreter can understand completely different keyword styles simply by changing the configuration.


## Building

Clone the repository:

```bash
git clone https://github.com/<your-username>/evenus.git
cd evenus
```

Run the interpreter:

```bash
cargo run "FILENAME TO MAIN.EVS" "FILENAME TO EVENUS.JSON"
evenus-interpreter.exe "FILENAME TO MAIN.EVS" "FILENAME TO EVENUS.JSON"
```

## Motivation

Evenus started as a project to better understand how interpreters work while experimenting with the idea of a programming language whose syntax can be customised. It serves as both a learning project and a platform for exploring language design.

## Contributing

Suggestions, bug reports, and pull requests are always welcome. As the language is still evolving, feedback on the syntax and overall design is especially appreciated.

## License

This project is licensed under the MIT License.
