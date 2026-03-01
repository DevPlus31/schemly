//! MCP Server for Schemly - exposes init_schema, generate, and doctor as MCP tools.

use rmcp::{
    ServerHandler, ServiceExt,
    model::{ServerCapabilities, ServerInfo},
    tool,
    transport::stdio,
};
use schemly::generators::*;
use schemly::schema::{SchemaConverter, parse_schema};
use schemly::types::Config;
use schemly::validation::Validator;
use std::fs;
use std::path::Path;

/// Schemly MCP Server - Laravel code generator from Prisma-like schema
#[derive(Debug, Clone)]
struct SchemlyServer;

impl SchemlyServer {
    pub fn new() -> Self {
        Self
    }
}

#[tool(tool_box)]
impl SchemlyServer {
    /// Initialize a new schema.schemly file with example models.
    #[tool(description = "Create a default schema.schemly file with example models")]
    fn init_schema(
        &self,
        #[tool(param)]
        #[schemars(description = "Output file path for the schema file (e.g., 'schema.schemly')")]
        output_path: String,
        #[tool(param)]
        #[schemars(description = "Force overwrite if file already exists")]
        force: Option<bool>,
    ) -> Result<String, String> {
        let path = Path::new(&output_path);
        let force = force.unwrap_or(false);

        if path.exists() && !force {
            return Err(format!(
                "File '{}' already exists. Set force=true to overwrite.",
                output_path
            ));
        }

        let schema_content = create_default_schema();
        fs::write(&output_path, schema_content)
            .map_err(|e| format!("Failed to write file: {}", e))?;

        Ok(format!(
            "✓ Created {}\n\nNext steps:\n  1. Edit {} to define your models\n  2. Use the 'generate' tool to create Laravel code",
            output_path, output_path
        ))
    }

    /// Generate Laravel code from a schema string.
    #[tool(description = "Generate Laravel models, controllers, migrations, etc. from a Prisma-like schema string")]
    fn generate(
        &self,
        #[tool(param)]
        #[schemars(description = "The schema content as a string (Prisma-like syntax)")]
        schema_content: String,
        #[tool(param)]
        #[schemars(description = "Laravel project root directory where files will be generated")]
        output_path: String,
        #[tool(param)]
        #[schemars(description = "Force overwrite existing files")]
        force: Option<bool>,
        #[tool(param)]
        #[schemars(description = "Use Domain-Driven Design folder structure")]
        ddd: Option<bool>,
        #[tool(param)]
        #[schemars(description = "Generate only specific components (comma-separated: models,migrations,controllers,resources,factories,dtos,requests)")]
        only: Option<String>,
    ) -> Result<String, String> {
        let force = force.unwrap_or(false);
        let ddd = ddd.unwrap_or(false);

        // Parse schema
        let schema = parse_schema(&schema_content)
            .map_err(|e| format!("Schema parse error: {}", e))?;

        // Convert to config
        let mut config = SchemaConverter::convert_to_config(schema)
            .map_err(|e| format!("Schema conversion error: {}", e))?;

        // Apply options
        config.output_dir = output_path.clone();
        config.force_overwrite = force;
        config.use_ddd_structure = ddd;

        // Parse --only components
        if let Some(only_str) = only {
            apply_only_filter(&mut config, &only_str);
        }

        // Validate config
        validate_config(&config)?;

        // Generate files
        let result = generate_all(&config)?;

        Ok(result)
    }

    /// Check a Laravel project for compatibility with Schemly.
    #[tool(description = "Check a Laravel project directory for compatibility with Schemly")]
    fn doctor(
        &self,
        #[tool(param)]
        #[schemars(description = "Path to the Laravel project root directory")]
        path: String,
    ) -> Result<String, String> {
        let mut output = format!("🔍 Checking Laravel project at: {}\n\n", path);
        let path_obj = Path::new(&path);

        // Check composer.json
        let composer_json = path_obj.join("composer.json");
        if !composer_json.exists() {
            output.push_str("❌ composer.json not found\n");
            return Ok(output);
        }
        output.push_str("✓ composer.json found\n");

        // Check artisan
        let artisan = path_obj.join("artisan");
        if !artisan.exists() {
            output.push_str("❌ artisan file not found\n");
            return Ok(output);
        }
        output.push_str("✓ artisan file found\n");

        // Check app directory
        let app_dir = path_obj.join("app");
        if !app_dir.exists() {
            output.push_str("❌ app/ directory not found\n");
        } else {
            output.push_str("✓ app/ directory found\n");
        }

        // Check Models directory
        let models_dir = path_obj.join("app").join("Models");
        if models_dir.exists() {
            output.push_str("✓ app/Models/ directory found\n");
        } else {
            output.push_str("⚠ app/Models/ directory not found (will be created)\n");
        }

        // Check database/migrations
        let migrations_dir = path_obj.join("database").join("migrations");
        if migrations_dir.exists() {
            output.push_str("✓ database/migrations/ directory found\n");
        } else {
            output.push_str("⚠ database/migrations/ directory not found (will be created)\n");
        }

        output.push_str("\n✓ Laravel project looks compatible with Schemly!\n");
        Ok(output)
    }

    /// Analyze a Schemly schema file and return its AST as JSON.
    #[tool(description = "Analyze a Schemly schema file and return its Abstract Syntax Tree (AST) as JSON. Useful for AI agents to understand the existing database structure.")]
    fn analyze_schema(
        &self,
        #[tool(param)]
        #[schemars(description = "Path to the .schemly file to analyze")]
        path: String,
    ) -> Result<String, String> {
        let schema_content = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read schema file at {}: {}", path, e))?;

        let schema = parse_schema(&schema_content)
            .map_err(|e| format!("Schema parse error: {}", e))?;

        let json = serde_json::to_string_pretty(&schema)
            .map_err(|e| format!("Serialization error: {}", e))?;

        Ok(json)
    }

    /// Check for drifted generated files.
    #[tool(description = "Check if generated Laravel files on disk differ from what Schemly would generate in-memory from the provided schema. Useful to detect human modifications before overwriting.")]
    fn check_drifts(
        &self,
        #[tool(param)]
        #[schemars(description = "The schema content as a string (Prisma-like syntax)")]
        schema_content: String,
        #[tool(param)]
        #[schemars(description = "Laravel project root directory where files are generated")]
        output_path: String,
        #[tool(param)]
        #[schemars(description = "Use Domain-Driven Design folder structure")]
        ddd: Option<bool>,
        #[tool(param)]
        #[schemars(description = "Check only specific components (comma-separated: models,migrations,controllers,resources,factories,dtos,requests)")]
        only: Option<String>,
    ) -> Result<String, String> {
        let ddd = ddd.unwrap_or(false);

        // Parse schema
        let schema = parse_schema(&schema_content)
            .map_err(|e| format!("Schema parse error: {}", e))?;

        // Convert to config
        let mut config = SchemaConverter::convert_to_config(schema)
            .map_err(|e| format!("Schema conversion error: {}", e))?;

        // Apply options
        config.output_dir = output_path.clone();
        config.use_ddd_structure = ddd;

        // Parse --only components
        if let Some(only_str) = only {
            apply_only_filter(&mut config, &only_str);
        }

        // Validate config
        validate_config(&config)?;

        // Check drifts
        let result = check_drifts_all(&config)?;

        Ok(result)
    }
}

// Implement the ServerHandler trait for SchemlyServer
#[tool(tool_box)]
impl ServerHandler for SchemlyServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("Schemly MCP Server - Generate Laravel code from Prisma-like schema files. Use init_schema to create a new schema file, generate to create Laravel code from a schema string, doctor to check project compatibility, analyze_schema to inspect existing schemas, and check_drifts to detect modifications.".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

// Helper functions

fn create_default_schema() -> &'static str {
    r#"// Schemly Schema File
// Learn more: https://schemly.dev/docs

generator laravel {
  provider = "schemly"
  output   = "./app"
}

datasource db {
  provider = "mysql"
  url      = env("DATABASE_URL")
}

// Example model
model User {
  id        Int      @id @default(autoincrement())
  name      String   @db.VarChar(255)
  email     String   @unique @db.VarChar(255)
  password  String   @db.VarChar(255)
  createdAt DateTime @default(now()) @map("created_at")
  updatedAt DateTime @updatedAt @map("updated_at")

  posts     Post[]

  @@map("users")
  @@traits(["HasFactory", "Notifiable"])
  @@fillable(["name", "email", "password"])
}

model Post {
  id        Int      @id @default(autoincrement())
  title     String   @db.VarChar(255)
  content   String   @db.LongText
  published Boolean  @default(false)
  userId    Int      @map("user_id")
  createdAt DateTime @default(now()) @map("created_at")
  updatedAt DateTime @updatedAt @map("updated_at")

  user      User     @relation(fields: [userId], references: [id], onDelete: Cascade)

  @@map("posts")
  @@traits(["HasFactory"])
  @@fillable(["title", "content", "published", "user_id"])
}
"#
}

fn apply_only_filter(config: &mut Config, only_str: &str) {
    // Reset all to false first
    config.generate_models = false;
    config.generate_controllers = false;
    config.generate_resources = false;
    config.generate_factories = false;
    config.generate_migrations = false;
    config.generate_pivot_tables = false;
    config.generate_requests = false;
    config.generate_dto = false;

    for component in only_str.split(',') {
        match component.trim().to_lowercase().as_str() {
            "models" | "model" => config.generate_models = true,
            "controllers" | "controller" => config.generate_controllers = true,
            "resources" | "resource" => config.generate_resources = true,
            "factories" | "factory" => config.generate_factories = true,
            "migrations" | "migration" => config.generate_migrations = true,
            "pivot" | "pivots" | "pivot_tables" => config.generate_pivot_tables = true,
            "requests" | "request" => config.generate_requests = true,
            "dtos" | "dto" => config.generate_dto = true,
            _ => {} // Ignore unknown components
        }
    }
}

fn validate_config(config: &Config) -> Result<(), String> {
    for model in &config.models {
        Validator::validate_model(model)
            .map_err(|e| format!("Validation error: {}", e))?;
    }
    Ok(())
}


fn generate_all(config: &Config) -> Result<String, String> {
    let mut output = String::new();
    let mut written = 0;
    let mut skipped = 0;
    let mut errors = 0;

    macro_rules! process {
        ($cond:expr, $gen:expr, $name:expr, $model:expr) => {
            if $cond {
                match run_generator($gen, $model, config, $name) {
                    Ok(msg) => { output.push_str(&msg); written += 1; }
                    Err(msg) if msg.contains("already exists") => { output.push_str(&msg); skipped += 1; }
                    Err(msg) => { output.push_str(&msg); errors += 1; }
                }
            }
        }
    }

    for model in &config.models {
        process!(config.generate_models, &model_generator::ModelGenerator, "model", model);
        process!(config.generate_migrations, &migration_generator::MigrationGenerator, "migration", model);
        process!(config.generate_controllers, &controller_generator::ControllerGenerator, "controller", model);
        process!(config.generate_resources, &resource_generator::ResourceGenerator, "resource", model);
        process!(config.generate_factories, &factory_generator::FactoryGenerator, "factory", model);
        process!(config.generate_dto, &dto_generator::DtoGenerator, "DTO", model);

        if config.generate_requests {
            for action in ["store", "update"] {
                let action_content = match request_generator::RequestGenerator.generate_action(model, config, action) {
                    Ok(c) => c,
                    Err(e) => { output.push_str(&format!("❌ Failed to generate {} Request for {}: {}\n", action, model.name, e)); errors += 1; "".to_string() }
                };
                if !action_content.is_empty() {
                    let action_path = request_generator::RequestGenerator.get_file_path_action(model, config, action);
                    match safe_write_file(&action_path, &action_content, config.force_overwrite) {
                        Ok(msg) => { output.push_str(&msg); written += 1; }
                        Err(msg) if msg.contains("already exists") => { output.push_str(&msg); skipped += 1; }
                        Err(msg) => { output.push_str(&msg); errors += 1; }
                    }
                }
            }
        }
    }

    // Summary
    output.push_str(&format!(
        "\n📊 Summary: {} written, {} skipped, {} errors\n",
        written, skipped, errors
    ));

    Ok(output)
}

fn check_drifts_all(config: &Config) -> Result<String, String> {
    let mut output = String::new();
    let mut intact = 0;
    let mut drifted = 0;
    let mut missing = 0;
    let mut errors = 0;

    macro_rules! check {
        ($cond:expr, $gen:expr, $name:expr, $model:expr) => {
            if $cond {
                match run_drift_check($gen, $model, config, $name) {
                    Ok(msg) => { 
                        output.push_str(&msg); 
                        if msg.contains("INTACT") { intact += 1; } else if msg.contains("DRIFTED") { drifted += 1; } else { missing += 1; }
                    }
                    Err(msg) => { output.push_str(&msg); errors += 1; }
                }
            }
        }
    }

    for model in &config.models {
        check!(config.generate_models, &model_generator::ModelGenerator, "model", model);
        check!(config.generate_migrations, &migration_generator::MigrationGenerator, "migration", model);
        check!(config.generate_controllers, &controller_generator::ControllerGenerator, "controller", model);
        check!(config.generate_resources, &resource_generator::ResourceGenerator, "resource", model);
        check!(config.generate_factories, &factory_generator::FactoryGenerator, "factory", model);
        check!(config.generate_dto, &dto_generator::DtoGenerator, "DTO", model);

        if config.generate_requests {
            for action in ["store", "update"] {
                let action_content = match request_generator::RequestGenerator.generate_action(model, config, action) {
                    Ok(c) => c,
                    Err(e) => { output.push_str(&format!("❌ Failed to generate {} Request for {}: {}\n", action, model.name, e)); errors += 1; "".to_string() }
                };
                if !action_content.is_empty() {
                    let action_path = request_generator::RequestGenerator.get_file_path_action(model, config, action);
                    match check_file_drift(&action_path, &action_content) {
                        Ok(msg) => { 
                            output.push_str(&msg); 
                            if msg.contains("INTACT") { intact += 1; } else if msg.contains("DRIFTED") { drifted += 1; } else { missing += 1; }
                        }
                        Err(msg) => { output.push_str(&msg); errors += 1; }
                    }
                }
            }
        }
    }

    // Summary
    output.push_str(&format!(
        "\n📊 Summary: {} intact, {} drifted, {} missing, {} errors\n",
        intact, drifted, missing, errors
    ));

    Ok(output)
}

fn run_generator<G: Generator>(
    generator: &G,
    model: &schemly::types::ModelDefinition,
    config: &Config,
    component_name: &str,
) -> Result<String, String> {
    let content = generator.generate(model, config)
        .map_err(|e| format!("❌ Failed to generate {} for {}: {}\n", component_name, model.name, e))?;
    let file_path = generator.get_file_path(model, config);

    safe_write_file(&file_path, &content, config.force_overwrite)
}

fn safe_write_file(path: &str, content: &str, force: bool) -> Result<String, String> {
    let path_obj = Path::new(path);

    if path_obj.exists() && !force {
        return Err(format!("⚠ File already exists, skipping: {}\n", path));
    }

    // Create parent directories if needed
    if let Some(parent) = path_obj.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("❌ Failed to create directory: {}\n", e))?;
    }

    fs::write(path, content)
        .map_err(|e| format!("❌ Failed to write {}: {}\n", path, e))?;

    Ok(format!("✓ Generated: {}\n", path))
}

fn run_drift_check<G: Generator>(
    generator: &G,
    model: &schemly::types::ModelDefinition,
    config: &Config,
    component_name: &str,
) -> Result<String, String> {
    let expected_content = generator.generate(model, config)
        .map_err(|e| format!("❌ Failed to generate {} for {}: {}\n", component_name, model.name, e))?;
    let file_path = generator.get_file_path(model, config);

    check_file_drift(&file_path, &expected_content)
}

fn check_file_drift(path: &str, expected_content: &str) -> Result<String, String> {
    let path_obj = Path::new(path);
    if !path_obj.exists() {
        return Ok(format!("❌ MISSING: {}\n", path));
    }

    let actual_content = fs::read_to_string(path)
        .map_err(|e| format!("❌ Error reading {}: {}\n", path, e))?;

    if actual_content == expected_content {
        Ok(format!("✓ INTACT: {}\n", path))
    } else {
        Ok(format!("⚠ DRIFTED: {}\n", path))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create the server and run over stdio
    let service = SchemlyServer::new()
        .serve(stdio())
        .await?;

    service.waiting().await?;

    Ok(())
}