use crate::token::Token;
use crate::config::Config;

pub struct Lexer {
    characters: Vec<char>,
    position: usize,
    config: Config,
}

impl Lexer {
    pub fn new(source: &str, config: Config) -> Self {
        Lexer {
            characters: source.chars().collect(),
            position: 0,
            config,
        }
    }
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while self.position < self.characters.len() {
            let current_char = self.characters[self.position];
            match current_char {
                '0'..='9' => {
                    let number = self.read_number();
                    tokens.push(Token::Number(number));
                }
                '"' => {
                    let string_literal = self.read_string_literal();
                    tokens.push(Token::StringLiteral(string_literal));
                }
                '(' => {
                    tokens.push(Token::BracketOpen);
                    self.position += 1;
                }
                ')' => {
                    tokens.push(Token::BracketClose);
                    self.position += 1;
                }
                ';' => {
                    tokens.push(Token::EndOfLine);
                    self.position += 1;
                }
                _ if current_char.is_alphabetic() => {
                    let identifier = self.read_identifier();
                    tokens.push(identifier);
                }
                '=' => {
                    if self.position + 1 < self.characters.len()
                        && self.characters[self.position + 1] == '='
                    {
                        tokens.push(Token::DoubleEquals);
                        self.position += 2;
                    } else {
                        tokens.push(Token::Equals);
                        self.position += 1;
                    }
                }
                '-' => {
                    tokens.push(Token::Minus);
                    self.position += 1;
                }
                '+' => {
                    tokens.push(Token::Plus);
                    self.position += 1;
                }
                '*' => {
                    tokens.push(Token::Multiply);
                    self.position += 1;
                }
                '/' => {
                    tokens.push(Token::Divide);
                    self.position += 1;
                }
                '{' => {
                    tokens.push(Token::CurlyOpen);
                    self.position += 1;
                }
                '}' => {
                    tokens.push(Token::CurlyClose);
                    self.position += 1;
                }
                _ => {
                    // Skip whitespace and unrecognized characters
                    self.position += 1;
                }
            }
        }
        tokens.push(Token::EndOfFile);
        tokens
    }
    pub fn read_number(&mut self) -> i32 {
        let start = self.position;
        while self.position < self.characters.len() {
            let current_char = self.characters[self.position];
            if !current_char.is_ascii_digit() {
                break;
            }
            self.position += 1;
        }
        self.characters[start..self.position]
            .iter()
            .collect::<String>()
            .parse()
            .unwrap()
    }
    pub fn read_string_literal(&mut self) -> String {
        self.position += 1; // Skip the opening quote
        let mut value = String::new();

        while let Some(character) = self.characters.get(self.position) {
            if *character == '"' {
                self.position += 1;
                return value;
            }

            value.push(*character);
            self.position += 1;
        }

        panic!("Unterminated string");
    }
    pub fn read_identifier(&mut self) -> Token {
        let start = self.position;

        while self.position < self.characters.len() {
            let current_char = self.characters[self.position];

            if !current_char.is_alphanumeric() && current_char != '_' {
                break;
            }

            self.position += 1;
        }

        let word = self.characters[start..self.position]
            .iter()
            .collect::<String>();

        if word == self.config.keywords.print {
            Token::Print
        } else if word == self.config.keywords.variable {
            Token::Variable
        } else if word == self.config.keywords.if_statement {
            Token::If
        } else if word == self.config.keywords.else_statement {
            Token::Else
        } else if word == "true" {
            Token::True
        } else if word == "false" {
            Token::False
        } else if word == self.config.keywords.input {
            Token::Input
        } else {
            Token::Identifier(word)
        }
    }
}
