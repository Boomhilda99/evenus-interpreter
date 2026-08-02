#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Print(Expression),

    VariableDeclaration {
        name: String,
        value: Expression,
    },
    If {
        condition: Expression,
        body: Vec<Statement>,
        else_body: Option<Vec<Statement>>,
    },
    Input(Expression),
}
#[derive(Debug)]
pub enum Expression {
    Number(i32),
    StringLiteral(String),
    Identifier(String),
    Boolean(bool),

    Input {
        prompt: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
}
#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    DoubleEquals,
}