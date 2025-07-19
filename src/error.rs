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
    #[error("Template error: {0}")]
    Template(String),
    #[error("Field validation error: {0}")]
    FieldValidation(String),
    #[error("Configuration error: {0}")]
    Configuration(String),
    #[error("Invalid identifier: {0}")]
    InvalidIdentifier(String),
    #[error("Generator error in {context}: {source}")]
    GeneratorContext {
        context: String,
        source: Box<GeneratorError>,
    },
    #[error("Multiple errors occurred")]
    Multiple(Vec<GeneratorError>),
}

impl GeneratorError {
    /// Add context to an error
    pub fn with_context<S: Into<String>>(self, context: S) -> Self {
        GeneratorError::GeneratorContext {
            context: context.into(),
            source: Box::new(self),
        }
    }

    /// Create a multiple error from a vector of errors
    pub fn multiple(errors: Vec<GeneratorError>) -> Self {
        GeneratorError::Multiple(errors)
    }

    /// Check if this error is recoverable
    pub fn is_recoverable(&self) -> bool {
        match self {
            GeneratorError::Io(_) => false,
            GeneratorError::Yaml(_) => false,
            GeneratorError::ModelValidation(_) => true,
            GeneratorError::FieldValidation(_) => true,
            GeneratorError::InvalidIdentifier(_) => true,
            GeneratorError::Configuration(_) => true,
            GeneratorError::Template(_) => false,
            GeneratorError::InvalidFieldType(_) => true,
            GeneratorError::InvalidRelationshipType(_) => true,
            GeneratorError::GeneratorContext { source, .. } => source.is_recoverable(),
            GeneratorError::Multiple(errors) => errors.iter().all(|e| e.is_recoverable()),
        }
    }
}

pub type Result<T> = std::result::Result<T, GeneratorError>;