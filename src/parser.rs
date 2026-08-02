use crate::ast::{BinaryOperator, Expression, Program, Statement};
use crate::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            if self.matches(&Token::EndOfLine) {
                continue;
            }

            let statement = self.parse_statement()?;
            statements.push(statement);
        }

        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.current() {
            Some(Token::Print) => self.parse_print_statement(),

            Some(Token::Input) => self.parse_input_statement(),

            Some(Token::Variable) => self.parse_variable_declaration(),

            Some(Token::If) => self.parse_if_statement(),

            Some(token) => Err(format!(
                "Unexpected token at position {}: {:?}",
                self.position, token
            )),

            None => Err("Unexpected end of token stream".to_string()),
        }
    }

    fn parse_print_statement(&mut self) -> Result<Statement, String> {
        self.expect(&Token::Print, "Expected print keyword")?;

        self.expect(&Token::BracketOpen, "Expected '(' after print")?;

        let expression = self.parse_expression()?;

        self.expect(&Token::BracketClose, "Expected ')' after print expression")?;

        self.expect(&Token::EndOfLine, "Expected ';' after print statement")?;

        Ok(Statement::Print(expression))
    }

    fn parse_input_statement(&mut self) -> Result<Statement, String> {
        self.expect(&Token::Input, "Expected input keyword")?;

        self.expect(&Token::BracketOpen, "Expected '(' after input")?;

        let expression = self.parse_expression()?;

        self.expect(&Token::BracketClose, "Expected ')' after input expression")?;

        self.expect(&Token::EndOfLine, "Expected ';' after input statement")?;

        Ok(Statement::Input(expression))
    }

    fn parse_variable_declaration(&mut self) -> Result<Statement, String> {
        self.expect(&Token::Variable, "Expected variable keyword")?;

        let name = match self.advance() {
            Some(Token::Identifier(name)) => name,

            Some(token) => {
                return Err(format!("Expected variable name, found {:?}", token));
            }

            None => {
                return Err("Expected variable name, found end of file".to_string());
            }
        };

        self.expect(&Token::Equals, "Expected '=' after variable name")?;

        let value = self.parse_expression()?;

        self.expect(&Token::EndOfLine, "Expected ';' after variable declaration")?;

        Ok(Statement::VariableDeclaration { name, value })
    }

    fn parse_if_statement(&mut self) -> Result<Statement, String> {
        self.expect(&Token::If, "Expected 'if' keyword")?;

        let condition = self.parse_expression()?;

        self.expect(&Token::CurlyOpen, "Expected '{' after if condition")?;

        let body = self.parse_block()?;

        let else_body = if self.matches(&Token::Else) {
            self.expect(&Token::CurlyOpen, "Expected '{' after else")?;

            Some(self.parse_block()?)
        } else {
            None
        };

        Ok(Statement::If {
            condition,
            body,
            else_body,
        })
    }

    fn parse_block(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();

        while !matches!(
            self.current(),
            Some(Token::CurlyClose) | Some(Token::EndOfFile) | None
        ) {
            if self.matches(&Token::EndOfLine) {
                continue;
            }

            statements.push(self.parse_statement()?);
        }

        self.expect(&Token::CurlyClose, "Expected '}' after block")?;

        Ok(statements)
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> Result<Expression, String> {
        let mut expression = self.parse_addition()?;

        while matches!(self.current(), Some(Token::DoubleEquals)) {
            self.position += 1;

            let right = self.parse_addition()?;

            expression = Expression::Binary {
                left: Box::new(expression),
                operator: BinaryOperator::DoubleEquals,
                right: Box::new(right),
            };
        }

        Ok(expression)
    }
    fn parse_addition(&mut self) -> Result<Expression, String> {
        let mut expression = self.parse_multiplication()?;

        loop {
            let operator = match self.current() {
                Some(Token::Plus) => BinaryOperator::Add,
                Some(Token::Minus) => BinaryOperator::Subtract,
                _ => break,
            };

            self.position += 1;

            let right = self.parse_multiplication()?;

            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expression)
    }

    fn parse_multiplication(&mut self) -> Result<Expression, String> {
        let mut expression = self.parse_primary()?;

        loop {
            let operator = match self.current() {
                Some(Token::Multiply) => BinaryOperator::Multiply,

                Some(Token::Divide) => BinaryOperator::Divide,

                _ => break,
            };

            self.position += 1;

            let right = self.parse_primary()?;

            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expression)
    }

    fn parse_primary(&mut self) -> Result<Expression, String> {
        match self.advance() {
            Some(Token::Number(number)) => Ok(Expression::Number(number)),

            Some(Token::StringLiteral(text)) => Ok(Expression::StringLiteral(text)),

            Some(Token::Identifier(name)) => Ok(Expression::Identifier(name)),
            Some(Token::True) => Ok(Expression::Boolean(true)),
            Some(Token::False) => Ok(Expression::Boolean(false)),

            Some(Token::Input) => self.parse_input_expression(),

            Some(Token::BracketOpen) => {
                let expression = self.parse_expression()?;

                self.expect(&Token::BracketClose, "Expected ')' after expression")?;

                Ok(expression)
            }

            Some(token) => Err(format!("Expected expression, found {:?}", token)),

            None => Err("Expected expression, found end of file".to_string()),
        }
    }

    fn parse_input_expression(&mut self) -> Result<Expression, String> {
        self.expect(&Token::BracketOpen, "Expected '(' after input")?;

        let prompt = self.parse_expression()?;

        self.expect(&Token::BracketClose, "Expected ')' after input prompt")?;

        Ok(Expression::Input {
            prompt: Box::new(prompt),
        })
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.position)?.clone();
        self.position += 1;
        Some(token)
    }

    fn expect(&mut self, expected: &Token, message: &str) -> Result<(), String> {
        match self.current() {
            Some(token) if token == expected => {
                self.position += 1;
                Ok(())
            }

            Some(token) => Err(format!(
                "{}: found {:?} at position {}",
                message, token, self.position
            )),

            None => Err(format!("{}: reached end of token stream", message)),
        }
    }

    fn matches(&mut self, expected: &Token) -> bool {
        if self.current() == Some(expected) {
            self.position += 1;
            true
        } else {
            false
        }
    }

    fn is_at_end(&self) -> bool {
        matches!(self.current(), Some(Token::EndOfFile) | None)
    }
}
