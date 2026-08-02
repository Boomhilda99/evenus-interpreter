#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(i32),
    StringLiteral(String),
    Identifier(String),
    BracketOpen,
    BracketClose,
    EndOfLine,
    EndOfFile,
    CurlyOpen,
    CurlyClose,


    Print,
    Variable,
    If,
    Else,
    Input,

    True,
    False,
    DoubleEquals,
    Equals,
    Minus,
    Plus,
    Multiply,
    Divide,
}