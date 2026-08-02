use std::collections::HashMap;

use crate::ast::{BinaryOperator, Expression, Program, Statement};
use crate::inputs::read_input;

#[derive(Debug, Clone)]
pub enum Value {
    Number(i32),
    String(String),
    Boolean(bool),
}

pub struct Interpreter {
    variables: HashMap<String, Value>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn run(&mut self, program: &Program) -> Result<(), String> {
        for statement in &program.statements {
            self.execute_statement(statement)?;
        }

        Ok(())
    }

    fn execute_statement(&mut self, statement: &Statement) -> Result<(), String> {
        match statement {
            Statement::Print(expression) => {
                let value = self.evaluate_expression(expression)?;

                match value {
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

                Ok(())
            }

            Statement::VariableDeclaration { name, value } => {
                let evaluated_value = self.evaluate_expression(value)?;

                self.variables.insert(name.clone(), evaluated_value);

                Ok(())
            }
            Statement::If {
                condition,
                body,
                else_body,
            } => {
                let condition_value = self.evaluate_expression(condition)?;

                match condition_value {
                    Value::Boolean(true) => {
                        for statement in body {
                            self.execute_statement(statement)?;
                        }
                    }

                    Value::Boolean(false) => {
                        if let Some(else_statements) = else_body {
                            for statement in else_statements {
                                self.execute_statement(statement)?;
                            }
                        }
                    }

                    _ => {
                        return Err("If condition must evaluate to a boolean".to_string());
                    }
                }

                Ok(())
            }
            Statement::Input(variable_name) => {
                let value = self.evaluate_expression(variable_name)?;
                read_input(&value);

                Ok(())
            }
        }
    }

    fn evaluate_expression(&self, expression: &Expression) -> Result<Value, String> {
        match expression {
            Expression::Number(number) => Ok(Value::Number(*number)),

            Expression::StringLiteral(text) => Ok(Value::String(text.clone())),
            
            Expression::Input { prompt } => {
                let prompt_value = self.evaluate_expression(prompt)?;
                let input = read_input(&prompt_value);
                Ok(Value::String(input))
            }

            Expression::Identifier(name) => {
                if name == "true" {
                    return Ok(Value::Boolean(true));
                } else if name == "false" {
                    return Ok(Value::Boolean(false));
                }
                self.variables
                    .get(name)
                    .cloned()
                    .ok_or_else(|| format!("Undefined variable '{}'", name))
            }

            Expression::Boolean(boolean) => Ok(Value::Boolean(*boolean)),
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                let left_value = self.evaluate_expression(left)?;

                let right_value = self.evaluate_expression(right)?;

                self.evaluate_binary(left_value, operator, right_value)
            }
        }
    }

    fn evaluate_binary(
        &self,
        left: Value,
        operator: &BinaryOperator,
        right: Value,
    ) -> Result<Value, String> {
        match (left, operator, right) {
            (Value::Number(left), BinaryOperator::Add, Value::Number(right)) => {
                Ok(Value::Number(left + right))
            }

            (Value::Number(left), BinaryOperator::Subtract, Value::Number(right)) => {
                Ok(Value::Number(left - right))
            }

            (Value::Number(left), BinaryOperator::Multiply, Value::Number(right)) => {
                Ok(Value::Number(left * right))
            }

            (Value::Number(left), BinaryOperator::Divide, Value::Number(right)) => {
                if right == 0 {
                    Err("Cannot divide by zero".to_string())
                } else {
                    Ok(Value::Number(left / right))
                }
            }

            (Value::Number(left), BinaryOperator::DoubleEquals, Value::Number(right)) => {
                Ok(Value::Boolean(left == right))
            }

            (Value::String(left), BinaryOperator::DoubleEquals, Value::String(right)) => {
                Ok(Value::Boolean(left == right))
            }

            (Value::Boolean(left), BinaryOperator::DoubleEquals, Value::Boolean(right)) => {
                Ok(Value::Boolean(left == right))
            }

            (left, BinaryOperator::DoubleEquals, right) => Ok(Value::Boolean(
                std::mem::discriminant(&left) == std::mem::discriminant(&right) && false,
            )),

            _ => Err("Invalid operands for operator".to_string()),
        }
    }
}
