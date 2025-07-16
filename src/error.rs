use thiserror::Error;

#[derive(Error, Debug)]
pub enum GeneratorError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("YAML parsing error: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("Invalid field type: {0}")]
    InvalidFieldType(String),
    #[error("Invalid relationship type: {0}")]
    InvalidRelationshipType(String),
    #[error("Model validation error: {0}")]
    ModelValidation(String),
}

pub type Result<T> = std::result::Result<T, GeneratorError>;