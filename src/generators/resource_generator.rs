use crate::error::Result;
use crate::generators::Generator;
use crate::generators::shared::{PathResolver, NamespaceResolver};
use crate::types::{Config, ModelDefinition};

pub struct ResourceGenerator;

impl Generator for ResourceGenerator {
    fn generate(&self, model: &ModelDefinition, config: &Config) -> Result<String> {
        let mut content = String::new();

        content.push_str("<?php\n\n");
        let namespace = NamespaceResolver::get_resource_namespace(model, config);
        content.push_str(&format!("namespace {};\n\n", namespace));
        content.push_str("use Illuminate\\Http\\Request;\n");
        content.push_str("use Illuminate\\Http\\Resources\\Json\\JsonResource;\n\n");

        content.push_str(&format!("class {}Resource extends JsonResource\n{{\n", model.name));
        content.push_str("    public function toArray(Request $request): array\n    {\n");
        content.push_str("        return [\n");

        // Always include ID
        content.push_str("            'id' => $this->id,\n");

        // Include all fields
        for field in &model.fields {
            if field.name != "id" {
                content.push_str(&format!("            '{}' => $this->{},\n", field.name, field.name));
            }
        }

        // Include timestamps if enabled
        if model.timestamps {
            content.push_str("            'created_at' => $this->created_at,\n");
            content.push_str("            'updated_at' => $this->updated_at,\n");
        }

        // Include soft delete timestamp if enabled
        if model.soft_deletes {
            content.push_str("            'deleted_at' => $this->deleted_at,\n");
        }

        content.push_str("        ];\n");
        content.push_str("    }\n");
        content.push_str("}\n");

        Ok(content)
    }

    fn get_file_path(&self, model: &ModelDefinition, config: &Config) -> String {
        PathResolver::get_resource_path(model, config)
    }
}