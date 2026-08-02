use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub keywords: Keywords,
}

#[derive(Debug, Deserialize)]
pub struct Keywords {
    pub print: String,
    pub variable: String,

    pub if_statement: String,
    pub else_statement: String,

    pub input: String,
}

impl Config {
    pub fn load(path: &str) -> Result<Self, String> {
        let contents = fs::read_to_string(path)
            .map_err(|error| {
                format!(
                    "Could not read config file '{}': {}",
                    path, error
                )
            })?;

        let config = serde_json::from_str::<Config>(&contents)
            .map_err(|error| {
                format!(
                    "Could not parse config file '{}': {}",
                    path, error
                )
            })?;

        config.validate()?;

        Ok(config)
    }

    fn validate(&self) -> Result<(), String> {
        if self.keywords.print.trim().is_empty() {
            return Err(
                "The print keyword cannot be empty".to_string()
            );
        }

        if self.keywords.variable.trim().is_empty() {
            return Err(
                "The variable keyword cannot be empty".to_string()
            );
        }

        if self.keywords.if_statement.trim().is_empty() {
            return Err(
                "The if statement cannot be empty".to_string()
            );
        }

        if self.keywords.else_statement.trim().is_empty() {
            return Err(
                "The else statement cannot be empty".to_string()
            );
        }

        if self.keywords.print == self.keywords.variable {
            return Err(format!(
                "The print and variable keywords cannot both be '{}'",
                self.keywords.print
            ));
        }

        if self.keywords.print == self.keywords.if_statement {
            return Err(format!(
                "The print and if keywords cannot both be '{}'",
                self.keywords.print
            ));
        }

        if self.keywords.variable == self.keywords.if_statement {
            return Err(format!(
                "The variable and if keywords cannot both be '{}'",
                self.keywords.variable
            ));
        }

        Ok(())
    }
}