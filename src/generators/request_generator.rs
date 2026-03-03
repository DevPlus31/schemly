use crate::generators::Generator;
use crate::generators::shared::{PathResolver, NamespaceResolver};
use crate::types::{Config, ModelDefinition};
use crate::validation::Validator;
use crate::template::{TemplateContext, TemplateRenderer};
use serde::{Deserialize, Serialize};

// Type aliases for better readability
type GeneratorResult<T> = crate::error::Result<T>;

/// Generator for Laravel Form Request classes
///
/// Supports both traditional Laravel structure (app/Http/Requests/) and
/// Domain-Driven Design structure (app/Domain/{Model}/Requests/)
pub struct RequestGenerator;

// Template constants
const TEMPLATE: &str = include_str!("../templates/request.php.template");

// Template variable names
mod template_vars {
    pub const NAMESPACE: &str = "namespace";
    pub const REQUEST_NAME: &str = "request_name";
    pub const RULES: &str = "rules";
}

const REQUIRED_TEMPLATE_VARS: &[&str] = &[
    template_vars::NAMESPACE,
    template_vars::REQUEST_NAME,
    template_vars::RULES,
];

#[derive(Debug, Serialize, Deserialize)]
struct RuleContext {
    field: String,
    validation: String,
}

impl Generator for RequestGenerator {
    fn generate(&self, model: &ModelDefinition, config: &Config) -> GeneratorResult<String> {
        self.validate_inputs(model, config)?;
        // By default, the `generate` trait method is called once per Component per Model by the main loop.
        // But for Requests, we actually want to generate TWO files (Store and Update).
        // To fit into the existing Generator trait which returns a single String, 
        // we'll primarily use this trait for one (e.g., Store) or handle it differently in the CLI. 
        // For Schemly's design, we'll implement a custom `generate_both` method and just use `generate` as a proxy to `Store`.
        let context = self.build_template_context(model, config, "store")?;
        self.render_template(&context)
    }

    fn get_file_path(&self, model: &ModelDefinition, config: &Config) -> String {
        PathResolver::get_request_path(model, config, "store")
    }
}

impl RequestGenerator {
    pub fn generate_action(&self, model: &ModelDefinition, config: &Config, action: &str) -> GeneratorResult<String> {
        self.validate_inputs(model, config)?;
        let context = self.build_template_context(model, config, action)?;
        self.render_template(&context)
    }

    pub fn get_file_path_action(&self, model: &ModelDefinition, config: &Config, action: &str) -> String {
        PathResolver::get_request_path(model, config, action)
    }

    /// Validates model and configuration inputs
    fn validate_inputs(&self, model: &ModelDefinition, config: &Config) -> GeneratorResult<()> {
        Validator::validate_model(model)?;

        if config.output_dir.is_empty() {
            return Err(crate::error::GeneratorError::Configuration(
                "Output directory cannot be empty".to_string()
            ));
        }

        Validator::validate_identifier(&model.name, "Request class name")?;
        Ok(())
    }

    /// Builds the template context with all required variables
    fn build_template_context(&self, model: &ModelDefinition, config: &Config, action: &str) -> GeneratorResult<TemplateContext> {
        let namespace = NamespaceResolver::get_request_namespace(model, config);
        
        // e.g., "StoreUserRequest" or "UpdateUserRequest"
        let prefix = if action == "store" { "Store" } else { "Update" };
        let request_name = format!("{}{}", prefix, &model.name);
        
        let rules = self.generate_rules(model, action)?;
        let mut rules_str = String::new();
        for rule in &rules {
            rules_str.push_str(&format!("'{}' => '{}',\n            ", rule.field, rule.validation));
        }

        let context = TemplateContext::new()
            .with(template_vars::NAMESPACE, format!("\\{}", namespace))
            .with(template_vars::REQUEST_NAME, request_name)
            .with(template_vars::RULES, rules_str.trim_end());

        Ok(context)
    }

    /// Renders the Request template with the provided context
    fn render_template(&self, context: &TemplateContext) -> GeneratorResult<String> {
        TemplateRenderer::render_with_required_vars(
            TEMPLATE,
            context,
            REQUIRED_TEMPLATE_VARS
        )
    }

    /// Generates rules list
    fn generate_rules(&self, model: &ModelDefinition, action: &str) -> GeneratorResult<Vec<RuleContext>> {
        let mut rules = Vec::new();

        for field in &model.fields {
            // Skip ID and common timestamps for insertion rules
            if field.name == "id" || field.name == "created_at" || field.name == "updated_at" || field.name == "deleted_at" {
                continue;
            }

            let mut field_rules = Vec::new();

            // Usually, 'store' requires fields, 'update' might make them sometimes nullable
            // Schemly currently doesn't differentiate required vs optional elegantly at the REST level other than `nullable` flag.
            if !field.nullable && action == "store" {
                field_rules.push("required".to_string());
            } else if field.nullable {
                field_rules.push("nullable".to_string());
            }

            match field.field_type {
                crate::types::FieldType::String | crate::types::FieldType::Text | crate::types::FieldType::LongText | crate::types::FieldType::MediumText => {
                    field_rules.push("string".to_string());
                    if let Some(len) = field.length {
                        field_rules.push(format!("max:{}", len));
                    }
                }
                crate::types::FieldType::Integer | crate::types::FieldType::BigInteger | crate::types::FieldType::TinyInteger => {
                    field_rules.push("integer".to_string());
                }
                crate::types::FieldType::Float | crate::types::FieldType::Decimal => {
                    field_rules.push("numeric".to_string());
                }
                crate::types::FieldType::Boolean => {
                    field_rules.push("boolean".to_string());
                }
                crate::types::FieldType::Date => {
                    field_rules.push("date".to_string());
                }
                crate::types::FieldType::DateTime | crate::types::FieldType::Timestamp => {
                    field_rules.push("date".to_string());
                }
                crate::types::FieldType::Json => {
                    field_rules.push("array".to_string());
                }
                crate::types::FieldType::Uuid => {
                    field_rules.push("uuid".to_string());
                }
                _ => {}
            }

            // Expose user defined rules from the schema (@validate(...))
            for custom_rule in &field.validation_rules {
                field_rules.push(custom_rule.rule.clone());
            }

            if !field_rules.is_empty() {
                rules.push(RuleContext {
                    field: field.name.clone(),
                    validation: field_rules.join("|"),
                });
            }
        }

        Ok(rules)
    }
}
