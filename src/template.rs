use crate::error::{GeneratorError, Result};
use std::collections::HashMap;

/// Template context for rendering templates with placeholders
#[derive(Debug, Clone)]
pub struct TemplateContext {
    variables: HashMap<String, String>,
}

impl TemplateContext {
    /// Create a new empty template context
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    /// Add a variable to the context
    pub fn set<K: Into<String>, V: Into<String>>(&mut self, key: K, value: V) -> &mut Self {
        self.variables.insert(key.into(), value.into());
        self
    }

    /// Add a variable to the context (builder pattern)
    pub fn with<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.set(key, value);
        self
    }

    /// Get a variable from the context
    pub fn get(&self, key: &str) -> Option<&String> {
        self.variables.get(key)
    }

    /// Check if a variable exists in the context
    pub fn contains(&self, key: &str) -> bool {
        self.variables.contains_key(key)
    }

    /// Get all variable names
    pub fn keys(&self) -> Vec<&String> {
        self.variables.keys().collect()
    }
}

impl Default for TemplateContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Template renderer that replaces placeholders with context values
pub struct TemplateRenderer;

impl TemplateRenderer {
    /// Render a template with the given context
    pub fn render(template: &str, context: &TemplateContext) -> Result<String> {
        let mut result = template.to_string();
        let mut missing_variables = Vec::new();
        let mut used_variables = std::collections::HashSet::new();

        // Find all placeholders in the template
        let placeholders = Self::find_placeholders(template)?;

        // Replace each placeholder
        for placeholder in placeholders {
            let var_name = Self::extract_variable_name(&placeholder)?;
            
            if let Some(value) = context.get(&var_name) {
                result = result.replace(&placeholder, value);
                used_variables.insert(var_name.clone());
            } else {
                missing_variables.push(var_name);
            }
        }

        // Check for missing variables
        if !missing_variables.is_empty() {
            return Err(GeneratorError::Template(
                format!("Missing template variables: {}", missing_variables.join(", "))
            ));
        }

        // Warn about unused variables (in debug mode)
        #[cfg(debug_assertions)]
        {
            let unused_variables: Vec<_> = context.keys()
                .into_iter()
                .filter(|key| !used_variables.contains(*key))
                .collect();
            
            if !unused_variables.is_empty() {
                eprintln!("Warning: Unused template variables: {:?}", unused_variables);
            }
        }

        Ok(result)
    }

    /// Find all placeholders in a template
    fn find_placeholders(template: &str) -> Result<Vec<String>> {
        let mut placeholders = Vec::new();
        let chars = template.chars().collect::<Vec<_>>();
        let mut i = 0;

        while i < chars.len() {
            if i < chars.len() - 1 && chars[i] == '{' && chars[i + 1] == '{' {
                // Found start of placeholder
                let start_pos = i;
                i += 2; // Skip the opening {{

                let mut placeholder_content = String::new();
                let mut found_end = false;

                // Look for closing }} but stop if we encounter another {{
                while i < chars.len() - 1 {
                    if chars[i] == '}' && chars[i + 1] == '}' {
                        // Found closing }}
                        i += 2; // Skip the closing }}
                        found_end = true;
                        break;
                    } else if chars[i] == '{' && chars[i + 1] == '{' {
                        // Found another opening {{ before closing the current one
                        break;
                    } else {
                        placeholder_content.push(chars[i]);
                        i += 1;
                    }
                }

                if !found_end {
                    return Err(GeneratorError::Template(
                        format!("Unclosed placeholder starting at position {}", start_pos)
                    ));
                }

                let full_placeholder = format!("{{{{{}}}}}", placeholder_content);
                placeholders.push(full_placeholder);
            } else {
                i += 1;
            }
        }

        Ok(placeholders)
    }

    /// Extract variable name from a placeholder (e.g., "{{variable_name}}" -> "variable_name")
    fn extract_variable_name(placeholder: &str) -> Result<String> {
        if !placeholder.starts_with("{{") || !placeholder.ends_with("}}") {
            return Err(GeneratorError::Template(
                format!("Invalid placeholder format: {}", placeholder)
            ));
        }

        let content = &placeholder[2..placeholder.len()-2];
        let trimmed = content.trim();
        
        if trimmed.is_empty() {
            return Err(GeneratorError::Template(
                "Empty placeholder found".to_string()
            ));
        }

        // Validate variable name (basic validation)
        if !trimmed.chars().all(|c| c.is_alphanumeric() || c == '_' || c == ' ') {
            return Err(GeneratorError::Template(
                format!("Invalid variable name in placeholder: {}", trimmed)
            ));
        }

        Ok(trimmed.to_string())
    }

    /// Render a template with validation of required variables
    pub fn render_with_required_vars(
        template: &str, 
        context: &TemplateContext, 
        required_vars: &[&str]
    ) -> Result<String> {
        // Check that all required variables are present
        let missing_required: Vec<_> = required_vars
            .iter()
            .filter(|&var| !context.contains(var))
            .collect();

        if !missing_required.is_empty() {
            return Err(GeneratorError::Template(
                format!("Missing required template variables: {}",
                       missing_required.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(", "))
            ));
        }

        Self::render(template, context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_context_basic() {
        let mut context = TemplateContext::new();
        context.set("name", "John");
        context.set("age", "30");

        assert_eq!(context.get("name"), Some(&"John".to_string()));
        assert_eq!(context.get("age"), Some(&"30".to_string()));
        assert_eq!(context.get("missing"), None);
        assert!(context.contains("name"));
        assert!(!context.contains("missing"));
    }

    #[test]
    fn test_template_context_builder() {
        let context = TemplateContext::new()
            .with("name", "John")
            .with("age", "30");

        assert_eq!(context.get("name"), Some(&"John".to_string()));
        assert_eq!(context.get("age"), Some(&"30".to_string()));
    }

    #[test]
    fn test_simple_template_rendering() {
        let template = "Hello {{name}}, you are {{age}} years old!";
        let context = TemplateContext::new()
            .with("name", "John")
            .with("age", "30");

        let result = TemplateRenderer::render(template, &context).unwrap();
        assert_eq!(result, "Hello John, you are 30 years old!");
    }

    #[test]
    fn test_template_with_spaces() {
        let template = "Hello {{ name }}, you are {{ age }} years old!";
        let context = TemplateContext::new()
            .with("name", "John")
            .with("age", "30");

        let result = TemplateRenderer::render(template, &context).unwrap();
        assert_eq!(result, "Hello John, you are 30 years old!");
    }

    #[test]
    fn test_missing_variable_error() {
        let template = "Hello {{name}}, you are {{age}} years old!";
        let context = TemplateContext::new()
            .with("name", "John");
            // missing "age"

        let result = TemplateRenderer::render(template, &context);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Missing template variables: age"));
    }

    #[test]
    fn test_find_placeholders() {
        let template = "Hello {{name}}, you are {{age}} years old! {{greeting}}";
        let placeholders = TemplateRenderer::find_placeholders(template).unwrap();
        
        assert_eq!(placeholders.len(), 3);
        assert!(placeholders.contains(&"{{name}}".to_string()));
        assert!(placeholders.contains(&"{{age}}".to_string()));
        assert!(placeholders.contains(&"{{greeting}}".to_string()));
    }

    #[test]
    fn test_unclosed_placeholder_error() {
        let template = "Hello {{name, you are {{age}} years old!";
        let result = TemplateRenderer::find_placeholders(template);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unclosed placeholder"));
    }

    #[test]
    fn test_empty_placeholder_error() {
        let template = "Hello {{}}, you are {{age}} years old!";
        let result = TemplateRenderer::render(template, &TemplateContext::new());
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Empty placeholder"));
    }

    #[test]
    fn test_render_with_required_vars() {
        let template = "Hello {{name}}, you are {{age}} years old!";
        let context = TemplateContext::new()
            .with("name", "John")
            .with("age", "30");

        let result = TemplateRenderer::render_with_required_vars(
            template, 
            &context, 
            &["name", "age"]
        ).unwrap();
        
        assert_eq!(result, "Hello John, you are 30 years old!");
    }

    #[test]
    fn test_render_with_missing_required_vars() {
        let template = "Hello {{name}}, you are {{age}} years old!";
        let context = TemplateContext::new()
            .with("name", "John");
            // missing required "age"

        let result = TemplateRenderer::render_with_required_vars(
            template,
            &context,
            &["name", "age"]
        );

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Missing required template variables: age"));
    }

    #[test]
    fn test_invalid_placeholder_format() {
        let template = "Hello {name}, you are {{age}} years old!";
        let context = TemplateContext::new()
            .with("name", "John")
            .with("age", "30");

        // Should work fine - single braces are not placeholders
        let result = TemplateRenderer::render(template, &context).unwrap();
        assert_eq!(result, "Hello {name}, you are 30 years old!");
    }

    #[test]
    fn test_nested_braces_in_content() {
        let template = "Hello {{name}}, your data is {{data}} years old!";
        let context = TemplateContext::new()
            .with("name", "John")
            .with("data", "{key: value}");

        let result = TemplateRenderer::render(template, &context).unwrap();
        assert_eq!(result, "Hello John, your data is {key: value} years old!");
    }

    #[test]
    fn test_multiple_missing_variables() {
        let template = "Hello {{name}}, you are {{age}} and live in {{city}}!";
        let context = TemplateContext::new()
            .with("name", "John");
            // missing "age" and "city"

        let result = TemplateRenderer::render(template, &context);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Missing template variables"));
        assert!(error_msg.contains("age"));
        assert!(error_msg.contains("city"));
    }

    #[test]
    fn test_template_with_only_spaces_in_placeholder() {
        let template = "Hello {{   }}, you are {{age}} years old!";
        let context = TemplateContext::new()
            .with("age", "30");

        let result = TemplateRenderer::render(template, &context);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Empty placeholder"));
    }

    #[test]
    fn test_template_with_invalid_variable_characters() {
        let template = "Hello {{na@me}}, you are {{age}} years old!";
        let context = TemplateContext::new()
            .with("na@me", "John")
            .with("age", "30");

        let result = TemplateRenderer::render(template, &context);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid variable name"));
    }
}
