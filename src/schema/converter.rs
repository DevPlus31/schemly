use crate::schema::ast;
use crate::types::{Config, ModelDefinition, Field, FieldType, FillableGuarded, ValidationRule};

/// Converts schema AST to internal types used by generators
pub struct SchemaConverter;

impl SchemaConverter {
    pub fn convert_to_config(schema: ast::Schema) -> Result<Config, String> {
        let mut config = Config::default();
        
        // Convert models
        for ast_model in schema.models {
            let model = Self::convert_model(ast_model)?;
            config.models.push(model);
        }
        
        Ok(config)
    }
    
    fn convert_model(ast_model: ast::Model) -> Result<ModelDefinition, String> {
        let mut model = ModelDefinition {
            name: ast_model.name.clone(),
            table: ast_model.get_table_name(),
            timestamps: ast_model.has_timestamps(),
            soft_deletes: ast_model.has_soft_deletes(),
            fields: Vec::new(),
            relationships: Vec::new(),
            pivot_tables: Vec::new(),
            fillable_guarded: Self::convert_fillable(&ast_model),
            traits: ast_model.get_traits(),
            validation_rules: Vec::new(),
            compound_indexes: ast_model.get_indexes(),
            compound_uniques: ast_model.get_compound_uniques(),
        };
        
        // Convert fields
        for ast_field in &ast_model.fields {
            // Skip timestamp fields as they're handled by timestamps flag
            if Self::is_timestamp_field(&ast_field.name) {
                continue;
            }
            
            let field = Self::convert_field(ast_field)?;
            model.fields.push(field);
        }
        
        Ok(model)
    }
    
    fn is_timestamp_field(name: &str) -> bool {
        matches!(name, "createdAt" | "created_at" | "updatedAt" | "updated_at" | "deletedAt" | "deleted_at")
    }
    
    fn convert_fillable(ast_model: &ast::Model) -> FillableGuarded {
        let fillable = ast_model.get_fillable();
        
        if !fillable.is_empty() {
            FillableGuarded::Fillable(fillable)
        } else {
            // Default to fillable with all non-id fields
            let fields: Vec<String> = ast_model.fields.iter()
                .filter(|f| !f.is_id() && !Self::is_timestamp_field(&f.name))
                .map(|f| f.get_map_name())
                .collect();
            FillableGuarded::Fillable(fields)
        }
    }
    
    fn convert_field(ast_field: &ast::Field) -> Result<Field, String> {
        let field_type = Self::convert_field_type(&ast_field.field_type)?;

        let field = Field {
            name: ast_field.name.clone(),
            field_type,
            nullable: ast_field.optional,
            unique: ast_field.is_unique(),
            default: Self::extract_default(ast_field),
            length: Self::extract_length(ast_field),
            index: false,
            enum_values: Vec::new(),
            decimal_precision: None,
            unsigned: false,
            auto_increment: ast_field.is_id(),
            primary: ast_field.is_id(),
            comment: None,
            validation_rules: Self::extract_validation_rules(ast_field),
            cast_type: None,
        };

        Ok(field)
    }
    
    fn convert_field_type(ast_type: &ast::FieldType) -> Result<FieldType, String> {
        match ast_type {
            ast::FieldType::String => Ok(FieldType::String),
            ast::FieldType::Int => Ok(FieldType::Integer),
            ast::FieldType::BigInt => Ok(FieldType::BigInteger),
            ast::FieldType::Float => Ok(FieldType::Float),
            ast::FieldType::Decimal => Ok(FieldType::Decimal),
            ast::FieldType::Boolean => Ok(FieldType::Boolean),
            ast::FieldType::DateTime => Ok(FieldType::DateTime),
            ast::FieldType::Json => Ok(FieldType::Json),
            ast::FieldType::Bytes => Ok(FieldType::Binary),
            ast::FieldType::Model(_) => {
                // Relationships are handled separately
                // For now, treat as integer (foreign key)
                Ok(FieldType::Integer)
            }
            ast::FieldType::Enum(_) => Ok(FieldType::Enum),
            ast::FieldType::Unsupported(s) => {
                Err(format!("Unsupported field type: {}", s))
            }
        }
    }
    
    fn extract_length(ast_field: &ast::Field) -> Option<u32> {
        // Check for @db attribute with length
        if let Some(db_type) = ast_field.get_db_type() {
            // Parse VarChar(255) -> 255
            if let Some(start) = db_type.find('(') {
                if let Some(end) = db_type.find(')') {
                    let length_str = &db_type[start+1..end];
                    if let Ok(length) = length_str.parse::<u32>() {
                        return Some(length);
                    }
                }
            }
        }
        
        // Default lengths for string types
        match ast_field.field_type {
            ast::FieldType::String => Some(255),
            _ => None,
        }
    }
    
    fn extract_default(ast_field: &ast::Field) -> Option<String> {
        if let Some(default_value) = ast_field.get_default() {
            match default_value {
                ast::Value::String(s) => Some(s.clone()),
                ast::Value::Integer(i) => Some(i.to_string()),
                ast::Value::Boolean(b) => Some(b.to_string()),
                ast::Value::Function { name, .. } => {
                    // Handle special functions
                    match name.as_str() {
                        "autoincrement" => None, // Handled by migration
                        "now" => Some("CURRENT_TIMESTAMP".to_string()),
                        _ => Some(format!("{}()", name)),
                    }
                }
                _ => None,
            }
        } else {
            None
        }
    }
    
    fn extract_validation_rules(ast_field: &ast::Field) -> Vec<ValidationRule> {
        let mut rules = Vec::new();
        
        // Get validation from @validate attribute
        if let Some(validation_str) = ast_field.get_validation_rules() {
            // Parse Laravel validation rules (e.g., "required|email|max:255")
            for rule_str in validation_str.split('|') {
                rules.push(ValidationRule {
                    rule: rule_str.to_string(),
                    parameters: None,
                });
            }
        } else {
            // Auto-generate basic validation rules
            if !ast_field.optional {
                rules.push(ValidationRule {
                    rule: "required".to_string(),
                    parameters: None,
                });
            } else {
                rules.push(ValidationRule {
                    rule: "nullable".to_string(),
                    parameters: None,
                });
            }

            // Add type-specific rules
            match ast_field.field_type {
                ast::FieldType::String => {
                    rules.push(ValidationRule {
                        rule: "string".to_string(),
                        parameters: None,
                    });
                    if let Some(length) = Self::extract_length(ast_field) {
                        rules.push(ValidationRule {
                            rule: "max".to_string(),
                            parameters: Some(vec![length.to_string()]),
                        });
                    }
                }
                ast::FieldType::Int | ast::FieldType::BigInt => {
                    rules.push(ValidationRule {
                        rule: "integer".to_string(),
                        parameters: None,
                    });
                }
                ast::FieldType::Boolean => {
                    rules.push(ValidationRule {
                        rule: "boolean".to_string(),
                        parameters: None,
                    });
                }
                ast::FieldType::DateTime => {
                    rules.push(ValidationRule {
                        rule: "date".to_string(),
                        parameters: None,
                    });
                }
                _ => {}
            }
            
            if ast_field.is_unique() {
                // We'll need the table name for this, skip for now
                // rules.push(ValidationRule {
                //     rule: format!("unique:{}", table_name),
                // });
            }
        }
        
        rules
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::ast;

    #[test]
    fn test_convert_simple_model() {
        let mut schema = ast::Schema::new();

        let mut model = ast::Model::new("User".to_string());

        let mut id_field = ast::Field::new("id".to_string(), ast::FieldType::Int);
        id_field.add_attribute(ast::FieldAttribute::new("id".to_string()));
        model.add_field(id_field);

        let name_field = ast::Field::new("name".to_string(), ast::FieldType::String);
        model.add_field(name_field);

        schema.add_model(model);

        let config = SchemaConverter::convert_to_config(schema).unwrap();

        assert_eq!(config.models.len(), 1);
        assert_eq!(config.models[0].name, "User");
        assert_eq!(config.models[0].fields.len(), 2);
    }
}

