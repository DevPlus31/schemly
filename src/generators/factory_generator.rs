use crate::error::Result;
use crate::generators::Generator;
use crate::generators::shared::{PathResolver, NamespaceResolver};
use crate::types::{Config, ModelDefinition, FieldType};

pub struct FactoryGenerator;

impl Generator for FactoryGenerator {
    fn generate(&self, model: &ModelDefinition, config: &Config) -> Result<String> {
        let mut content = String::new();

        content.push_str("<?php\n\n");
        let namespace = NamespaceResolver::get_factory_namespace(model, config);
        content.push_str(&format!("namespace {};\n\n", namespace));
        content.push_str("use Illuminate\\Database\\Eloquent\\Factories\\Factory;\n");
        let model_namespace = NamespaceResolver::get_model_namespace(model, config);
        content.push_str(&format!("use {}\\{};\n\n", model_namespace, model.name));

        content.push_str(&format!("class {}Factory extends Factory\n{{\n", model.name));
        content.push_str(&format!("    protected $model = {}::class;\n\n", model.name));

        content.push_str("    public function definition(): array\n    {\n");
        content.push_str("        return [\n");

        for field in &model.fields {
            if field.name != "id" {
                let faker_method = self.get_faker_method(&field.field_type, &field.name);
                content.push_str(&format!("            '{}' => {},\n", field.name, faker_method));
            }
        }

        content.push_str("        ];\n");
        content.push_str("    }\n");
        content.push_str("}\n");

        Ok(content)
    }

    fn get_file_path(&self, model: &ModelDefinition, config: &Config) -> String {
        PathResolver::get_factory_path(model, config)
    }
}

impl FactoryGenerator {
    fn get_faker_method(&self, field_type: &FieldType, field_name: &str) -> String {
        // Try to infer from field name first
        match field_name {
            name if name.contains("email") => "fake()->email()".to_string(),
            name if name.contains("name") => "fake()->name()".to_string(),
            name if name.contains("title") => "fake()->sentence(3)".to_string(),
            name if name.contains("description") || name.contains("content") => "fake()->paragraph()".to_string(),
            name if name.contains("phone") => "fake()->phoneNumber()".to_string(),
            name if name.contains("address") => "fake()->address()".to_string(),
            name if name.contains("city") => "fake()->city()".to_string(),
            name if name.contains("country") => "fake()->country()".to_string(),
            name if name.contains("url") || name.contains("website") => "fake()->url()".to_string(),
            name if name.contains("password") => "fake()->password()".to_string(),
            _ => {
                // Fall back to type-based generation
                match field_type {
                    FieldType::String => "fake()->word()".to_string(),
                    FieldType::Text => "fake()->text()".to_string(),
                    FieldType::Integer | FieldType::BigInteger => "fake()->numberBetween(1, 100)".to_string(),
                    FieldType::Float | FieldType::Decimal => "fake()->randomFloat(2, 0, 1000)".to_string(),
                    FieldType::Boolean => "fake()->boolean()".to_string(),
                    FieldType::Date => "fake()->date()".to_string(),
                    FieldType::DateTime | FieldType::Timestamp => "fake()->dateTime()".to_string(),
                    FieldType::Json => "fake()->words(3)".to_string(),
                    FieldType::Uuid => "fake()->uuid()".to_string(),
                    FieldType::Enum => "fake()->randomElement(['option1', 'option2', 'option3'])".to_string(),
                    FieldType::TinyInteger => "fake()->numberBetween(0, 255)".to_string(),
                    FieldType::SmallInteger => "fake()->numberBetween(-32768, 32767)".to_string(),
                    FieldType::MediumInteger => "fake()->numberBetween(-8388608, 8388607)".to_string(),
                    FieldType::LongText => "fake()->text(2000)".to_string(),
                    FieldType::MediumText => "fake()->text(500)".to_string(),
                    FieldType::Binary => "fake()->sha256()".to_string(),
                    FieldType::Inet => "fake()->ipv4()".to_string(),
                }
            }
        }
    }
}