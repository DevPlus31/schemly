use crate::generators::Generator;
use crate::generators::shared::{PathResolver, NamespaceResolver};
use crate::types::{Config, ModelDefinition};
use crate::validation::Validator;
use crate::template::{TemplateContext, TemplateRenderer};

// Type aliases for better readability
type GeneratorResult<T> = crate::error::Result<T>;

pub struct ControllerGenerator;

// Template constants
const TEMPLATE: &str = include_str!("../templates/controller.php.template");

// Template variable names
mod template_vars {
    pub const NAMESPACE: &str = "namespace";
    pub const MODEL_NAME: &str = "model_name";
    pub const MODEL_VAR_NAME: &str = "model_var_name";
    pub const VALIDATION_RULES: &str = "validation_rules";
    pub const REQUEST_NAMESPACE: &str = "request_namespace";
    pub const USE_REQUESTS: &str = "use_requests";
}

const REQUIRED_TEMPLATE_VARS: &[&str] = &[
    template_vars::NAMESPACE,
    template_vars::MODEL_NAME,
    template_vars::MODEL_VAR_NAME,
    template_vars::VALIDATION_RULES,
];

impl Generator for ControllerGenerator {
    fn generate(&self, model: &ModelDefinition, config: &Config) -> GeneratorResult<String> {
        self.validate_inputs(model, config)?;
        let context = self.build_template_context(model, config)?;
        self.render_template(&context)
    }

    fn get_file_path(&self, model: &ModelDefinition, config: &Config) -> String {
        if config.use_ddd_structure {
            format!("{}/app/Domain/{}/Controllers/{}Controller.php", config.output_dir, model.name, model.name)
        } else {
            format!("{}/app/Http/Controllers/{}Controller.php", config.output_dir, model.name)
        }
    }
}

impl ControllerGenerator {
    /// Validates model and configuration inputs
    fn validate_inputs(&self, model: &ModelDefinition, config: &Config) -> GeneratorResult<()> {
        Validator::validate_model(model)?;

        if config.output_dir.is_empty() {
            return Err(crate::error::GeneratorError::Configuration(
                "Output directory cannot be empty".to_string()
            ));
        }

        Validator::validate_identifier(&model.name, "Controller class name")?;
        Ok(())
    }

    /// Builds the template context with all required variables
    fn build_template_context(&self, model: &ModelDefinition, config: &Config) -> GeneratorResult<TemplateContext> {
        let namespace = NamespaceResolver::get_model_namespace(model, config);
        let request_namespace = NamespaceResolver::get_request_namespace(model, config);
        let model_var_name = model.name.to_lowercase();
        
        let mut validation_rules = String::new();
        for field in &model.fields {
            if field.name != "id" {
                let rule = if field.nullable { "nullable" } else { "required" };
                validation_rules.push_str(&format!("'{}' => '{}',\n            ", field.name, rule));
            }
        }

        let context = TemplateContext::new()
            .with(template_vars::NAMESPACE, namespace)
            .with(template_vars::MODEL_NAME, &model.name)
            .with(template_vars::MODEL_VAR_NAME, model_var_name)
            .with(template_vars::VALIDATION_RULES, validation_rules.trim_end())
            .with(template_vars::REQUEST_NAMESPACE, request_namespace)
            .with(template_vars::USE_REQUESTS, if config.generate_requests { "true" } else { "" });

        Ok(context)
    }

    /// Renders the template with the provided context
    fn render_template(&self, context: &TemplateContext) -> GeneratorResult<String> {
        TemplateRenderer::render_with_required_vars(
            TEMPLATE,
            context,
            REQUIRED_TEMPLATE_VARS
        )
    }
}