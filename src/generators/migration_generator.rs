use crate::error::Result;
use crate::generators::Generator;
use crate::types::{Config, ModelDefinition, Relationship};
use chrono::Utc;

pub struct MigrationGenerator;

// Include the template file at compile time
const MIGRATION_TEMPLATE: &str = include_str!("../templates/migration.php.template");

impl Generator for MigrationGenerator {
    fn generate(&self, model: &ModelDefinition, _config: &Config) -> Result<String> {
        // Prepare the template data
        let table_name = &model.table;

        // Handle ID field
        let has_custom_primary = model.fields.iter().any(|f| f.primary);
        let id_field = if has_custom_primary {
            "".to_string()
        } else {
            "$table->id();\n".to_string()
        };

        // Generate field definitions
        let mut fields = String::new();
        for field in &model.fields {
            if field.name != "id" || field.primary {
                fields.push_str(&self.build_field_definition(field));
            }
        }

        // Handle timestamps
        let timestamps = if model.timestamps {
            "$table->timestamps();".to_string()
        } else {
            "".to_string()
        };

        // Handle soft deletes
        let soft_deletes = if model.soft_deletes {
            "$table->softDeletes();".to_string()
        } else {
            "".to_string()
        };

        // Generate foreign key constraints
        let mut foreign_keys = String::new();
        for relationship in &model.relationships {
            if let Relationship::BelongsTo(rel) = relationship {
                if let Some(foreign_key) = &rel.foreign_key {
                    let referenced_table = self.model_name_to_table(&rel.model);
                    let on_delete = rel.on_delete.as_deref().unwrap_or("restrict");
                    let on_update = rel.on_update.as_deref().unwrap_or("restrict");

                    foreign_keys.push_str(&format!("Schema::table('{}', function (Blueprint $table) {{\n", model.table));
                    foreign_keys.push_str(&format!("    $table->foreign('{}')->references('id')->on('{}')->onDelete('{}')->onUpdate('{}');\n",
                                                foreign_key, referenced_table, on_delete, on_update));
                    foreign_keys.push_str("});\n\n");
                }
            }
        }

        // Replace placeholders in the template
        let content = MIGRATION_TEMPLATE
            .replace("{{table_name}}", table_name)
            .replace("{{id_field}}", &id_field)
            .replace("{{fields}}", &fields)
            .replace("{{timestamps}}", &timestamps)
            .replace("{{soft_deletes}}", &soft_deletes)
            .replace("{{foreign_keys}}", &foreign_keys);

        Ok(content)
    }

    fn get_file_path(&self, model: &ModelDefinition, config: &Config) -> String {
        let timestamp = Utc::now().format("%Y_%m_%d_%H%M%S");
        format!(
            "{}/database/migrations/{}_create_{}_table.php",
            config.output_dir,
            timestamp,
            model.table
        )
    }
}

impl MigrationGenerator {
    fn model_name_to_table(&self, model_name: &str) -> String {
        let snake_case = self.pascal_to_snake_case(model_name);
        self.pluralize(&snake_case)
    }

    fn pascal_to_snake_case(&self, input: &str) -> String {
        input
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if i > 0 && c.is_uppercase() {
                    format!("_{}", c.to_lowercase())
                } else {
                    c.to_lowercase().to_string()
                }
            })
            .collect::<String>()
    }

    fn pluralize(&self, word: &str) -> String {
        if word.ends_with("y") {
            format!("{}ies", &word[..word.len()-1])
        } else if word.ends_with("s") {
            format!("{}es", word)
        } else {
            format!("{}s", word)
        }
    }

    fn build_field_definition(&self, field: &crate::types::Field) -> String {
        let mut definition = String::new();

        // Base field type
        let field_method = match field.field_type {
            crate::types::FieldType::String => {
                if let Some(length) = field.length {
                    format!("string('{}', {})", field.name, length)
                } else {
                    format!("string('{}')", field.name)
                }
            },
            _ => format!("{}('{}')", field.field_type.to_migration_type(), field.name),
        };

        definition.push_str(&format!("            $table->{}", field_method));

        // Add modifiers
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

        definition.push_str(";\n");
        definition
    }
    
}