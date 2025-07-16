use crate::error::Result;
use crate::generators::Generator;
use crate::types::{Config, PivotTable, Field, ModelDefinition};
use chrono::Utc;

pub struct PivotTableGenerator;

// Include the template file at compile time
const PIVOT_TABLE_TEMPLATE: &str = include_str!("../templates/pivot_table.php.template");

impl Generator for PivotTableGenerator {
    fn generate(&self, _model: &ModelDefinition, _config: &Config) -> Result<String> {
        // We need to handle the case where we're passed a ModelDefinition instead of a PivotTable
        // This is a temporary solution until we refactor the Generator trait to be more generic
        return Err(crate::error::GeneratorError::ModelValidation(
            "PivotTableGenerator can only generate pivot tables, not models".to_string()));
    }

    fn get_file_path(&self, model: &ModelDefinition, config: &Config) -> String {
        let timestamp = Utc::now().format("%Y_%m_%d_%H%M%S");
        format!(
            "{}/database/migrations/{}_create_{}_table.php",
            config.output_dir,
            timestamp,
            model.name
        )
    }
}

impl PivotTableGenerator {
    pub fn generate_pivot_table(&self, pivot_table: &PivotTable, _config: &Config) -> Result<String> {
        // Prepare template data
        let table_name = &pivot_table.name;
        let foreign_key1 = &pivot_table.foreign_key1;
        let foreign_key2 = &pivot_table.foreign_key2;
        let table1 = self.model_name_to_table(&pivot_table.model1);
        let table2 = self.model_name_to_table(&pivot_table.model2);

        // Generate additional fields
        let mut additional_fields = String::new();
        for field in &pivot_table.additional_fields {
            additional_fields.push_str(&self.build_field_definition(field));
        }

        // Handle timestamps
        let timestamps = if pivot_table.timestamps {
            "$table->timestamps();".to_string()
        } else {
            "".to_string()
        };

        // Replace placeholders in the template
        let content = PIVOT_TABLE_TEMPLATE
            .replace("{{table_name}}", table_name)
            .replace("{{foreign_key1}}", foreign_key1)
            .replace("{{foreign_key2}}", foreign_key2)
            .replace("{{table1}}", &table1)
            .replace("{{table2}}", &table2)
            .replace("{{additional_fields}}", &additional_fields)
            .replace("{{timestamps}}", &timestamps);

        Ok(content)
    }

    pub fn get_pivot_file_path(&self, pivot_table: &PivotTable, config: &Config) -> String {
        let timestamp = Utc::now().format("%Y_%m_%d_%H%M%S");
        format!(
            "{}/database/migrations/{}_create_{}_table.php",
            config.output_dir,
            timestamp,
            pivot_table.name
        )
    }

    fn build_field_definition(&self, field: &Field) -> String {
        let mut definition = String::new();
        let field_method = self.get_field_method(field);
        definition.push_str(&format!("$table->{}", field_method));
        self.add_field_modifiers(&mut definition, field);
        definition.push_str(";\n");
        definition
    }

    fn get_field_method(&self, field: &Field) -> String {
        match field.field_type {
            crate::types::FieldType::String => {
                if let Some(length) = field.length {
                    format!("string('{}', {})", field.name, length)
                } else {
                    format!("string('{}')", field.name)
                }
            }
            crate::types::FieldType::Decimal => {
                if let Some(precision) = &field.decimal_precision {
                    format!("decimal('{}', {}, {})", field.name, precision.precision, precision.scale)
                } else {
                    format!("decimal('{}', 8, 2)", field.name)
                }
            }
            crate::types::FieldType::Enum => {
                let enum_values: Vec<String> = field.enum_values.iter()
                    .map(|v| format!("'{}'", v.value))
                    .collect();
                format!("enum('{}', [{}])", field.name, enum_values.join(", "))
            }
            _ => format!("{}('{}')", field.field_type.to_migration_type(), field.name),
        }
    }

    fn add_field_modifiers(&self, definition: &mut String, field: &Field) {
        if field.unsigned {
            definition.push_str("->unsigned()");
        }

        if field.nullable {
            definition.push_str("->nullable()");
        }

        if field.unique {
            definition.push_str("->unique()");
        }

        if field.index {
            definition.push_str("->index()");
        }

        if let Some(default_value) = &field.default {
            definition.push_str(&format!("->default('{}')", default_value));
        }

        if let Some(comment) = &field.comment {
            definition.push_str(&format!("->comment('{}')", comment));
        }
    }

    fn model_name_to_table(&self, model_name: &str) -> String {
        let snake_case = model_name
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if i > 0 && c.is_uppercase() {
                    format!("_{}", c.to_lowercase())
                } else {
                    c.to_lowercase().to_string()
                }
            })
            .collect::<String>();

        if snake_case.ends_with("y") {
            format!("{}ies", &snake_case[..snake_case.len()-1])
        } else if snake_case.ends_with("s") {
            format!("{}es", snake_case)
        } else {
            format!("{}s", snake_case)
        }
    }
}