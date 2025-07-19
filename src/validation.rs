use crate::error::{GeneratorError, Result};
use crate::types::{ModelDefinition, Field, FieldType};

/// Validates and sanitizes input for code generation
pub struct Validator;

impl Validator {
    /// Validates a complete model definition
    pub fn validate_model(model: &ModelDefinition) -> Result<()> {
        // Validate model name
        Self::validate_identifier(&model.name, "Model name")?;
        
        // Validate table name
        Self::validate_table_name(&model.table)?;
        
        // Validate that model has at least one field or timestamps
        if model.fields.is_empty() && !model.timestamps {
            return Err(GeneratorError::ModelValidation(
                format!("Model '{}' must have at least one field or timestamps enabled", model.name)
            ));
        }
        
        // Validate each field
        for field in &model.fields {
            Self::validate_field(field)?;
        }
        
        // Check for duplicate field names
        let mut field_names = std::collections::HashSet::new();
        for field in &model.fields {
            if !field_names.insert(&field.name) {
                return Err(GeneratorError::ModelValidation(
                    format!("Duplicate field name '{}' in model '{}'", field.name, model.name)
                ));
            }
        }
        
        // Validate that ID field is not manually defined (it's auto-generated)
        if model.fields.iter().any(|f| f.name == "id") {
            return Err(GeneratorError::ModelValidation(
                format!("Model '{}' should not manually define 'id' field - it's auto-generated", model.name)
            ));
        }
        
        Ok(())
    }
    
    /// Validates a single field definition
    pub fn validate_field(field: &Field) -> Result<()> {
        // Validate field name
        Self::validate_identifier(&field.name, "Field name")?;
        
        // Validate field type specific constraints
        match field.field_type {
            FieldType::String | FieldType::Text | FieldType::LongText | FieldType::MediumText => {
                if field.length.is_some() && field.length.unwrap() == 0 {
                    return Err(GeneratorError::FieldValidation(
                        format!("String field '{}' cannot have zero length", field.name)
                    ));
                }
            }
            FieldType::Decimal => {
                if field.decimal_precision.is_none() {
                    return Err(GeneratorError::FieldValidation(
                        format!("Decimal field '{}' must specify precision and scale", field.name)
                    ));
                }
                let precision = field.decimal_precision.as_ref().unwrap();
                if precision.precision == 0 || precision.scale > precision.precision {
                    return Err(GeneratorError::FieldValidation(
                        format!("Invalid decimal precision for field '{}': precision={}, scale={}", 
                               field.name, precision.precision, precision.scale)
                    ));
                }
            }
            FieldType::Enum => {
                if field.enum_values.is_empty() {
                    return Err(GeneratorError::FieldValidation(
                        format!("Enum field '{}' must specify at least one value", field.name)
                    ));
                }
                // Validate enum values
                for enum_value in &field.enum_values {
                    if enum_value.value.is_empty() {
                        return Err(GeneratorError::FieldValidation(
                            format!("Enum field '{}' has empty enum value", field.name)
                        ));
                    }
                }
            }
            _ => {} // Other types don't need special validation
        }
        
        // Validate that auto_increment is only used with integer types
        if field.auto_increment && !matches!(field.field_type, 
            FieldType::Integer | FieldType::BigInteger | FieldType::TinyInteger | 
            FieldType::SmallInteger | FieldType::MediumInteger) {
            return Err(GeneratorError::FieldValidation(
                format!("Field '{}' cannot use auto_increment with type {:?}", field.name, field.field_type)
            ));
        }
        
        // Validate that primary key fields are not nullable
        if field.primary && field.nullable {
            return Err(GeneratorError::FieldValidation(
                format!("Primary key field '{}' cannot be nullable", field.name)
            ));
        }
        
        Ok(())
    }
    
    /// Validates and sanitizes PHP/Laravel identifiers (class names, field names, etc.)
    pub fn validate_identifier(name: &str, context: &str) -> Result<()> {
        if name.is_empty() {
            return Err(GeneratorError::InvalidIdentifier(
                format!("{} cannot be empty", context)
            ));
        }
        
        // Check length (reasonable limit for PHP identifiers)
        if name.len() > 64 {
            return Err(GeneratorError::InvalidIdentifier(
                format!("{} '{}' is too long (max 64 characters)", context, name)
            ));
        }
        
        // Check that it starts with a letter or underscore
        let first_char = name.chars().next().unwrap();
        if !first_char.is_ascii_alphabetic() && first_char != '_' {
            return Err(GeneratorError::InvalidIdentifier(
                format!("{} '{}' must start with a letter or underscore", context, name)
            ));
        }
        
        // Check that all characters are valid (letters, numbers, underscores)
        if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return Err(GeneratorError::InvalidIdentifier(
                format!("{} '{}' contains invalid characters (only letters, numbers, and underscores allowed)", context, name)
            ));
        }
        
        // Check against PHP reserved words
        if Self::is_php_reserved_word(name) {
            return Err(GeneratorError::InvalidIdentifier(
                format!("{} '{}' is a PHP reserved word", context, name)
            ));
        }
        
        Ok(())
    }
    
    /// Validates table names (similar to identifiers but with different rules)
    pub fn validate_table_name(name: &str) -> Result<()> {
        if name.is_empty() {
            return Err(GeneratorError::InvalidIdentifier(
                "Table name cannot be empty".to_string()
            ));
        }
        
        // Table names can be longer than PHP identifiers
        if name.len() > 128 {
            return Err(GeneratorError::InvalidIdentifier(
                format!("Table name '{}' is too long (max 128 characters)", name)
            ));
        }
        
        // Check that all characters are valid for database table names
        if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return Err(GeneratorError::InvalidIdentifier(
                format!("Table name '{}' contains invalid characters (only letters, numbers, and underscores allowed)", name)
            ));
        }
        
        Ok(())
    }
    
    /// Sanitizes a field name to ensure it's valid for PHP
    pub fn sanitize_field_name(name: &str) -> Result<String> {
        if name.is_empty() {
            return Err(GeneratorError::InvalidIdentifier(
                "Field name cannot be empty".to_string()
            ));
        }
        
        let mut sanitized = String::new();
        
        // Handle first character
        let first_char = name.chars().next().unwrap();
        if first_char.is_ascii_alphabetic() || first_char == '_' {
            sanitized.push(first_char);
        } else if first_char.is_ascii_digit() {
            sanitized.push('_');
            sanitized.push(first_char);
        } else {
            sanitized.push('_');
        }
        
        // Handle remaining characters
        for c in name.chars().skip(1) {
            if c.is_ascii_alphanumeric() || c == '_' {
                sanitized.push(c);
            } else {
                sanitized.push('_');
            }
        }
        
        // Validate the sanitized name
        Self::validate_identifier(&sanitized, "Sanitized field name")?;
        
        Ok(sanitized)
    }
    
    /// Checks if a string is a PHP reserved word
    fn is_php_reserved_word(word: &str) -> bool {
        const PHP_RESERVED_WORDS: &[&str] = &[
            "abstract", "and", "array", "as", "break", "callable", "case", "catch", "class",
            "clone", "const", "continue", "declare", "default", "die", "do", "echo", "else",
            "elseif", "empty", "enddeclare", "endfor", "endforeach", "endif", "endswitch",
            "endwhile", "eval", "exit", "extends", "final", "finally", "for", "foreach",
            "function", "global", "goto", "if", "implements", "include", "include_once",
            "instanceof", "insteadof", "interface", "isset", "list", "namespace", "new",
            "or", "print", "private", "protected", "public", "require", "require_once",
            "return", "static", "switch", "throw", "trait", "try", "unset", "use", "var",
            "while", "xor", "yield", "int", "float", "bool", "string", "true", "false",
            "null", "void", "iterable", "object", "mixed", "never"
        ];
        
        PHP_RESERVED_WORDS.contains(&word.to_lowercase().as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{FillableGuarded, DecimalPrecision, EnumValue};

    fn create_valid_field() -> Field {
        Field {
            name: "test_field".to_string(),
            field_type: FieldType::String,
            nullable: false,
            unique: false,
            default: None,
            length: Some(255),
            index: false,
            enum_values: vec![],
            decimal_precision: None,
            unsigned: false,
            auto_increment: false,
            primary: false,
            comment: None,
            validation_rules: vec![],
            cast_type: None,
        }
    }

    fn create_valid_model() -> ModelDefinition {
        ModelDefinition {
            name: "TestModel".to_string(),
            table: "test_models".to_string(),
            fields: vec![create_valid_field()],
            timestamps: true,
            soft_deletes: false,
            relationships: vec![],
            pivot_tables: vec![],
            validation_rules: vec![],
            traits: vec![],
            fillable_guarded: FillableGuarded::All,
        }
    }

    #[test]
    fn test_validate_valid_model() {
        let model = create_valid_model();
        assert!(Validator::validate_model(&model).is_ok());
    }

    #[test]
    fn test_validate_empty_model_name() {
        let mut model = create_valid_model();
        model.name = "".to_string();
        assert!(Validator::validate_model(&model).is_err());
    }

    #[test]
    fn test_validate_model_with_no_fields_no_timestamps() {
        let mut model = create_valid_model();
        model.fields = vec![];
        model.timestamps = false;
        assert!(Validator::validate_model(&model).is_err());
    }

    #[test]
    fn test_validate_duplicate_field_names() {
        let mut model = create_valid_model();
        model.fields = vec![create_valid_field(), create_valid_field()];
        assert!(Validator::validate_model(&model).is_err());
    }

    #[test]
    fn test_validate_manual_id_field() {
        let mut model = create_valid_model();
        let mut id_field = create_valid_field();
        id_field.name = "id".to_string();
        model.fields.push(id_field);
        assert!(Validator::validate_model(&model).is_err());
    }

    #[test]
    fn test_validate_identifier_valid() {
        assert!(Validator::validate_identifier("valid_name", "Test").is_ok());
        assert!(Validator::validate_identifier("_private", "Test").is_ok());
        assert!(Validator::validate_identifier("name123", "Test").is_ok());
    }

    #[test]
    fn test_validate_identifier_invalid() {
        assert!(Validator::validate_identifier("", "Test").is_err());
        assert!(Validator::validate_identifier("123invalid", "Test").is_err());
        assert!(Validator::validate_identifier("invalid-name", "Test").is_err());
        assert!(Validator::validate_identifier("class", "Test").is_err()); // PHP reserved word
    }

    #[test]
    fn test_sanitize_field_name() {
        assert_eq!(Validator::sanitize_field_name("valid_name").unwrap(), "valid_name");
        assert_eq!(Validator::sanitize_field_name("123invalid").unwrap(), "_123invalid");
        assert_eq!(Validator::sanitize_field_name("invalid-name").unwrap(), "invalid_name");
    }

    #[test]
    fn test_validate_decimal_field() {
        let mut field = create_valid_field();
        field.field_type = FieldType::Decimal;
        field.decimal_precision = None;
        assert!(Validator::validate_field(&field).is_err());

        field.decimal_precision = Some(DecimalPrecision { precision: 8, scale: 2 });
        assert!(Validator::validate_field(&field).is_ok());

        field.decimal_precision = Some(DecimalPrecision { precision: 0, scale: 2 });
        assert!(Validator::validate_field(&field).is_err());
    }

    #[test]
    fn test_validate_enum_field() {
        let mut field = create_valid_field();
        field.field_type = FieldType::Enum;
        field.enum_values = vec![];
        assert!(Validator::validate_field(&field).is_err());

        field.enum_values = vec![EnumValue { value: "active".to_string(), label: None }];
        assert!(Validator::validate_field(&field).is_ok());
    }
}
