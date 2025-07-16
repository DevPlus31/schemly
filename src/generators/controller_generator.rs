use crate::error::Result;
use crate::generators::Generator;
use crate::types::{Config, ModelDefinition};

pub struct ControllerGenerator;

impl Generator for ControllerGenerator {
    fn generate(&self, model: &ModelDefinition, config: &Config) -> Result<String> {
        let mut content = String::new();
        let model_name = &model.name;
        let model_variable = model_name.to_lowercase();

        content.push_str("<?php\n\n");
        content.push_str("namespace App\\Http\\Controllers;\n\n");
        content.push_str("use Illuminate\\Http\\Request;\n");
        content.push_str(&format!("use {}\\{};\n", config.namespace, model_name));
        
        if config.generate_resources {
            content.push_str(&format!("use App\\Http\\Resources\\{}Resource;\n", model_name));
        }
        
        content.push_str("\n");
        content.push_str(&format!("class {}Controller extends Controller\n{{\n", model_name));

        // Index method
        content.push_str("    public function index()\n    {\n");
        if config.generate_resources {
            content.push_str(&format!("        return {}Resource::collection({}::all());\n", model_name, model_name));
        } else {
            content.push_str(&format!("        return {}::all();\n", model_name));
        }
        content.push_str("    }\n\n");

        // Show method
        content.push_str(&format!("    public function show({} ${})\n    {{\n", model_name, model_variable));
        if config.generate_resources {
            content.push_str(&format!("        return new {}Resource(${}); \n", model_name, model_variable));
        } else {
            content.push_str(&format!("        return ${};\n", model_variable));
        }
        content.push_str("    }\n\n");

        // Store method
        content.push_str("    public function store(Request $request)\n    {\n");
        content.push_str("        $validated = $request->validate([\n");
        
        for field in &model.fields {
            if field.name != "id" {
                let rule = if field.nullable { "nullable" } else { "required" };
                content.push_str(&format!("            '{}' => '{}',\n", field.name, rule));
            }
        }
        
        content.push_str("        ]);\n\n");
        content.push_str(&format!("        ${} = {}::create($validated);\n", model_variable, model_name));
        
        if config.generate_resources {
            content.push_str(&format!("        return new {}Resource(${}); \n", model_name, model_variable));
        } else {
            content.push_str(&format!("        return ${};\n", model_variable));
        }
        content.push_str("    }\n\n");

        // Update method
        content.push_str(&format!("    public function update(Request $request, {} ${})\n    {{\n", model_name, model_variable));
        content.push_str("        $validated = $request->validate([\n");
        
        for field in &model.fields {
            if field.name != "id" {
                let rule = if field.nullable { "nullable" } else { "required" };
                content.push_str(&format!("            '{}' => '{}',\n", field.name, rule));
            }
        }
        
        content.push_str("        ]);\n\n");
        content.push_str(&format!("        ${}->update($validated);\n", model_variable));
        
        if config.generate_resources {
            content.push_str(&format!("        return new {}Resource(${}); \n", model_name, model_variable));
        } else {
            content.push_str(&format!("        return ${};\n", model_variable));
        }
        content.push_str("    }\n\n");

        // Destroy method
        content.push_str(&format!("    public function destroy({} ${})\n    {{\n", model_name, model_variable));
        content.push_str(&format!("        ${}->delete();\n", model_variable));
        content.push_str("        return response()->json(null, 204);\n");
        content.push_str("    }\n");

        content.push_str("}\n");
        Ok(content)
    }

    fn get_file_path(&self, model: &ModelDefinition, config: &Config) -> String {
        format!("{}/app/Http/Controllers/{}Controller.php", config.output_dir, model.name)
    }
}