use thiserror::Error;

#[derive(Error, Debug)]
pub enum GeneratorError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Schema parsing error: {0}")]
    ParseError(String),
    #[error("Model validation error: {0}")]
    ModelValidation(String),
    #[error("Template error: {0}")]
    Template(String),
    #[error("Field validation error: {0}")]
    FieldValidation(String),
    #[error("Configuration error: {0}")]
    Configuration(String),
    #[error("Invalid identifier: {0}")]
    InvalidIdentifier(String),
}



pub type Result<T> = std::result::Result<T, GeneratorError>;