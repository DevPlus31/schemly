mod error;
mod generators;
mod schema;
mod template;
mod types;
mod validation;

use clap::{Parser, Subcommand};
use error::Result;
use generators::*;
use std::fs;
use std::path::Path;
use types::Config;
use validation::Validator;

#[derive(Debug, PartialEq)]
pub enum WriteResult {
    Written,
    Skipped,
    Error(String),
}

#[derive(Debug, Default)]
struct GenerationStats {
    written: usize,
    skipped: usize,
    errors: usize,
}




fn safe_write_file(file_path: &str, content: &str, force: bool) -> Result<WriteResult> {
    if force {
        // Always attempt to write, overwriting existing files
        match fs::write(file_path, content) {
            Ok(()) => Ok(WriteResult::Written),
            Err(e) => Ok(WriteResult::Error(e.to_string())),
        }
    } else {
        // Check if file exists first
        if Path::new(file_path).exists() {
            Ok(WriteResult::Skipped)
        } else {
            match fs::write(file_path, content) {
                Ok(()) => Ok(WriteResult::Written),
                Err(e) => Ok(WriteResult::Error(e.to_string())),
            }
        }
    }
}

#[derive(Parser)]
#[command(name = "schemly")]
#[command(version)]
#[command(about = "A Laravel code generator from Prisma-like schema files")]
#[command(long_about = "Generate Laravel models, controllers, resources, factories, migrations, and pivot tables from Prisma-like schema files.

EXAMPLES:
    schemly init                                      # Create default schema.schemly file
    schemly generate                                  # Generate all components
    schemly generate --dry-run                        # Preview what would be generated
    schemly generate --force                          # Overwrite existing files
    schemly generate --only models,migrations         # Generate only specific components
    schemly watch                                     # Watch schema file and auto-generate
    schemly doctor                                    # Check Laravel project compatibility

SAFETY:
    By default, existing files are NOT overwritten. Use --force to overwrite.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Path to schema file (default: ./schema.schemly)
    #[arg(short, long, global = true)]
    file: Option<String>,

    /// Print detailed logs
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Creates a default schema.schemly file
    Init {
        /// Output path for the schema file
        #[arg(short, long, default_value = "schema.schemly")]
        output: String,

        /// Force overwrite if file exists
        #[arg(long)]
        force: bool,
    },

    /// Compiles the schema into Laravel code
    Generate {
        /// Laravel project root directory
        #[arg(short, long, default_value = ".")]
        output: String,

        /// Preview what would be generated without writing files
        #[arg(long)]
        dry_run: bool,

        /// Force overwrite existing files
        #[arg(long)]
        force: bool,

        /// Generate only specific components (comma-separated: models,migrations,controllers,resources,factories,dtos,requests,pivot)
        #[arg(long, value_delimiter = ',')]
        only: Option<Vec<String>>,

        /// Exclude specific components (comma-separated: models,migrations,controllers,resources,factories,dtos,requests,pivot)
        #[arg(long, value_delimiter = ',', conflicts_with = "only")]
        exclude: Option<Vec<String>>,

        /// Use Domain-Driven Design folder structure
        #[arg(long)]
        ddd: bool,
    },

    /// Watches the schema file and auto-generates on save
    Watch {
        /// Laravel project root directory
        #[arg(short, long, default_value = ".")]
        output: String,

        /// Force overwrite existing files
        #[arg(long)]
        force: bool,

        /// Generate only specific components (comma-separated)
        #[arg(long, value_delimiter = ',')]
        only: Option<Vec<String>>,

        /// Exclude specific components (comma-separated)
        #[arg(long, value_delimiter = ',', conflicts_with = "only")]
        exclude: Option<Vec<String>>,
    },

    /// Checks your Laravel project for compatibility
    Doctor {
        /// Laravel project root directory
        #[arg(short, long, default_value = ".")]
        path: String,
    },

    /// Creates AI editor rules (.cursorrules, .windsurfrules) for Schemly
    InitRules {
        /// Output directory (default: current directory)
        #[arg(short, long, default_value = ".")]
        output: String,

        /// Force overwrite if files exist
        #[arg(long)]
        force: bool,
    },
}

struct LaravelGenerator {
    config: Config,
}

impl LaravelGenerator {
    pub fn from_file(file_path: &str) -> Result<Self> {
        let schema_content = fs::read_to_string(file_path)?;
        let schema = schema::parse_schema(&schema_content)
            .map_err(error::GeneratorError::ParseError)?;
        let config = schema::SchemaConverter::convert_to_config(schema)
            .map_err(error::GeneratorError::ParseError)?;
        config.validate()?;
        Ok(LaravelGenerator { config })
    }

    pub fn generate_all(&self) -> Result<()> {
        self.create_directories()?;

        let mut stats = GenerationStats::default();

        // Generate pivot tables first
        if self.config.generate_pivot_tables {
            // Process pivot tables from each model
            for model in &self.config.models {
                for pivot_table in &model.pivot_tables {
                    let result = self.generate_pivot_table(pivot_table)?;
                    self.update_stats(&mut stats, result);
                }
            }
        }

        for model in &self.config.models {
            // Validate each model before processing
            Validator::validate_model(model)?;

            if self.config.generate_models {
                let result = self.generate_component(&model_generator::ModelGenerator, model, &format!("Generated model: {}", model.name))?;
                self.update_stats(&mut stats, result);
            }

            if self.config.generate_migrations {
                let result = self.generate_component(&migration_generator::MigrationGenerator, model, &format!("Generated migration for table: {}", model.table))?;
                self.update_stats(&mut stats, result);
            }

            if self.config.generate_controllers {
                let result = self.generate_component(&controller_generator::ControllerGenerator, model, &format!("Generated controller: {}Controller", model.name))?;
                self.update_stats(&mut stats, result);
            }

            if self.config.generate_resources {
                let result = self.generate_component(&resource_generator::ResourceGenerator, model, &format!("Generated resource: {}Resource", model.name))?;
                self.update_stats(&mut stats, result);
            }

            if self.config.generate_factories {
                let result = self.generate_component(&factory_generator::FactoryGenerator, model, &format!("Generated factory: {}Factory", model.name))?;
                self.update_stats(&mut stats, result);
            }

            if self.config.generate_dto {
                let result = self.generate_component(&dto_generator::DtoGenerator, model, &format!("Generated DTO: {}DTO", model.name))?;
                self.update_stats(&mut stats, result);
            }

            if self.config.generate_requests {
                let (store_res, update_res) = self.generate_request(model)?;
                self.update_stats(&mut stats, store_res);
                self.update_stats(&mut stats, update_res);
            }
        }

        // Enhanced summary logging
        self.print_summary(&stats);
        Ok(())
    }

    fn create_directories(&self) -> Result<()> {
        // Create base output directory
        fs::create_dir_all(&self.config.output_dir)?;

        // Create directories for each model using the shared DirectoryCreator
        for model in &self.config.models {
            generators::shared::DirectoryCreator::create_model_directories(model, &self.config)?;
        }

        Ok(())
    }

    fn generate_pivot_table(&self, pivot_table: &types::PivotTable) -> Result<WriteResult> {
        let generator = pivot_table_generator::PivotTableGenerator;
        let content = generator.generate_pivot_table(pivot_table, &self.config)?;
        let file_path = generator.get_pivot_file_path(pivot_table, &self.config);

        let result = safe_write_file(&file_path, &content, self.config.force_overwrite)?;
        match &result {
            WriteResult::Written => println!("Generated pivot table: {}", pivot_table.name),
            WriteResult::Skipped => {
                println!("Warning: File already exists, skipping: {}", file_path)
            }
            WriteResult::Error(e) => println!("Error writing {}: {}", file_path, e),
        }
        Ok(result)
    }

    fn generate_component<G: generators::Generator>(
        &self,
        generator: &G,
        model: &types::ModelDefinition,
        message: &str,
    ) -> Result<WriteResult> {
        let content = generator.generate(model, &self.config)?;
        let file_path = generator.get_file_path(model, &self.config);

        let result = safe_write_file(&file_path, &content, self.config.force_overwrite)?;
        match &result {
            WriteResult::Written => println!("{}", message),
            WriteResult::Skipped => {
                println!("Warning: File already exists, skipping: {}", file_path)
            }
            WriteResult::Error(e) => println!("Error writing {}: {}", file_path, e),
        }
        Ok(result)
    }

    fn generate_request(&self, model: &types::ModelDefinition) -> Result<(WriteResult, WriteResult)> {
        let generator = request_generator::RequestGenerator;

        // Store
        let store_content = generator.generate_action(model, &self.config, "store")
            .map_err(|e| error::GeneratorError::Template(e.to_string()))?;
        let store_file_path = generator.get_file_path_action(model, &self.config, "store");
        let store_result = safe_write_file(&store_file_path, &store_content, self.config.force_overwrite)?;

        match &store_result {
            WriteResult::Written => println!("Generated request: Store{}Request", model.name),
            WriteResult::Skipped => println!("Warning: File already exists, skipping: {}", store_file_path),
            WriteResult::Error(e) => println!("Error writing {}: {}", store_file_path, e),
        }

        // Update
        let update_content = generator.generate_action(model, &self.config, "update")
            .map_err(|e| error::GeneratorError::Template(e.to_string()))?;
        let update_file_path = generator.get_file_path_action(model, &self.config, "update");
        let update_result = safe_write_file(&update_file_path, &update_content, self.config.force_overwrite)?;

        match &update_result {
            WriteResult::Written => println!("Generated request: Update{}Request", model.name),
            WriteResult::Skipped => println!("Warning: File already exists, skipping: {}", update_file_path),
            WriteResult::Error(e) => println!("Error writing {}: {}", update_file_path, e),
        }

        Ok((store_result, update_result))
    }

    fn update_stats(&self, stats: &mut GenerationStats, result: WriteResult) {
        match result {
            WriteResult::Written => stats.written += 1,
            WriteResult::Skipped => stats.skipped += 1,
            WriteResult::Error(_) => stats.errors += 1,
        }
    }

    fn print_summary(&self, stats: &GenerationStats) {
        let total = stats.written + stats.skipped + stats.errors;
        if total > 0 {
            println!("\nSummary:");
            if stats.written > 0 {
                println!("  ✓ {} files generated successfully", stats.written);
            }
            if stats.skipped > 0 {
                println!("  ⚠ {} files skipped (already exist)", stats.skipped);
            }
            if stats.errors > 0 {
                println!("  ✗ {} files failed to generate", stats.errors);
            }
            println!("  Total: {} files processed", total);
        }
    }
}

impl Config {
    fn validate(&self) -> Result<()> {
        for model in &self.models {
            if model.name.is_empty() {
                return Err(error::GeneratorError::ModelValidation(
                    "Model name cannot be empty".to_string(),
                ));
            }
            if model.table.is_empty() {
                return Err(error::GeneratorError::ModelValidation(
                    "Table name cannot be empty".to_string(),
                ));
            }
        }
        Ok(())
    }
}



// Helper functions
fn apply_component_filters(
    config: &mut Config,
    only: &Option<Vec<String>>,
    exclude: &Option<Vec<String>>,
) {
    if let Some(components) = only {
        // If --only is provided, turn everything OFF first, then enable requested
        config.generate_models = false;
        config.generate_controllers = false;
        config.generate_resources = false;
        config.generate_factories = false;
        config.generate_migrations = false;
        config.generate_pivot_tables = false;
        config.generate_dto = false;
        config.generate_requests = false;

        for component in components {
            match component.to_lowercase().as_str() {
                "models" | "model" => config.generate_models = true,
                "controllers" | "controller" => config.generate_controllers = true,
                "resources" | "resource" => config.generate_resources = true,
                "factories" | "factory" => config.generate_factories = true,
                "migrations" | "migration" => config.generate_migrations = true,
                "pivot" | "pivots" | "pivot_tables" => config.generate_pivot_tables = true,
                "dtos" | "dto" => config.generate_dto = true,
                "requests" | "request" => config.generate_requests = true,
                _ => eprintln!("⚠️  Warning: Unknown component in --only '{}'", component),
            }
        }
    } else if let Some(components) = exclude {
        // If --exclude is provided, disable the requested ones (leaving schema defaults alone otherwise)
        for component in components {
            match component.to_lowercase().as_str() {
                "models" | "model" => config.generate_models = false,
                "controllers" | "controller" => config.generate_controllers = false,
                "resources" | "resource" => config.generate_resources = false,
                "factories" | "factory" => config.generate_factories = false,
                "migrations" | "migration" => config.generate_migrations = false,
                "pivot" | "pivots" | "pivot_tables" => config.generate_pivot_tables = false,
                "dtos" | "dto" => config.generate_dto = false,
                "requests" | "request" => config.generate_requests = false,
                _ => eprintln!("⚠️  Warning: Unknown component in --exclude '{}'", component),
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn get_enabled_components_list(
    models: bool,
    controllers: bool,
    resources: bool,
    factories: bool,
    migrations: bool,
    pivot_tables: bool,
    dtos: bool,
    requests: bool,
) -> Vec<String> {
    let mut enabled = Vec::new();
    if models { enabled.push("models".to_string()); }
    if controllers { enabled.push("controllers".to_string()); }
    if resources { enabled.push("resources".to_string()); }
    if factories { enabled.push("factories".to_string()); }
    if migrations { enabled.push("migrations".to_string()); }
    if pivot_tables { enabled.push("pivot tables".to_string()); }
    if dtos { enabled.push("DTOs".to_string()); }
    if requests { enabled.push("requests".to_string()); }
    enabled
}

fn get_schema_path(file: &Option<String>) -> String {
    file.clone().unwrap_or_else(|| "schema.schemly".to_string())
}

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

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init { output, force } => {
            handle_init(output, *force)
        }
        Commands::Generate { output, dry_run, force, only, exclude, ddd } => {
            handle_generate(&cli, output, *dry_run, *force, only, exclude, *ddd)
        }
        Commands::Watch { output, force, only, exclude } => {
            handle_watch(&cli, output, *force, only, exclude)
        }
        Commands::Doctor { path } => {
            handle_doctor(path)
        }
        Commands::InitRules { output, force } => {
            handle_init_rules(output, *force)
        }
    }
}

fn handle_init(output: &str, force: bool) -> Result<()> {
    let path = Path::new(output);

    if path.exists() && !force {
        return Err(error::GeneratorError::ModelValidation(
            format!("File '{}' already exists. Use --force to overwrite.", output)
        ));
    }

    fs::write(output, create_default_schema())?;
    println!("✓ Created {}", output);
    println!("\nNext steps:");
    println!("  1. Edit {} to define your models", output);
    println!("  2. Run 'schemly generate' to create Laravel code");

    Ok(())
}

fn handle_generate(
    cli: &Cli,
    output: &str,
    dry_run: bool,
    force: bool,
    only: &Option<Vec<String>>,
    exclude: &Option<Vec<String>>,
    ddd: bool,
) -> Result<()> {
    let schema_path = get_schema_path(&cli.file);

    if cli.verbose {
        println!("📄 Reading schema from: {}", schema_path);
    }

    let mut generator = LaravelGenerator::from_file(&schema_path)?;

    // Apply component selection (CLI args take priority over schema config)
    apply_component_filters(&mut generator.config, only, exclude);

    // Override config with CLI options
    generator.config.output_dir = output.to_string();
    generator.config.force_overwrite = force;
    generator.config.use_ddd_structure = ddd;

    // Warn user about force flag
    if force {
        println!("⚠️  Warning: --force flag enabled. Existing files will be overwritten!");
    }

    // Log which components will be generated
    let enabled_components = get_enabled_components_list(
        generator.config.generate_models,
        generator.config.generate_controllers,
        generator.config.generate_resources,
        generator.config.generate_factories,
        generator.config.generate_migrations,
        generator.config.generate_pivot_tables,
        generator.config.generate_dto,
        generator.config.generate_requests,
    );

    if dry_run {
        println!("🔍 Dry run mode - no files will be written\n");
        println!("Would generate: {}", enabled_components.join(", "));
        println!("Output directory: {}", output);
        println!("DDD structure: {}", if ddd { "enabled" } else { "disabled" });
        println!("\nModels to process:");
        for model in &generator.config.models {
            println!("  - {}", model.name);
        }
        return Ok(());
    }

    println!("Generating: {}", enabled_components.join(", "));
    generator.generate_all()?;
    Ok(())
}

fn handle_watch(_cli: &Cli, _output: &str, _force: bool, _only: &Option<Vec<String>>, _exclude: &Option<Vec<String>>) -> Result<()> {
    println!("⚠️  Watch mode is not yet implemented.");
    println!("This feature will automatically regenerate code when the schema file changes.");
    Ok(())
}

fn handle_doctor(path: &str) -> Result<()> {
    println!("🔍 Checking Laravel project at: {}\n", path);

    let path_obj = Path::new(path);

    // Check if it's a Laravel project
    let composer_json = path_obj.join("composer.json");
    let artisan = path_obj.join("artisan");

    if !composer_json.exists() {
        println!("❌ composer.json not found");
        return Ok(());
    }

    if !artisan.exists() {
        println!("❌ artisan file not found");
        return Ok(());
    }

    println!("✓ Valid Laravel project detected");

    // Check directory structure
    let app_dir = path_obj.join("app");
    let models_dir = app_dir.join("Models");
    let controllers_dir = app_dir.join("Http/Controllers");
    let migrations_dir = path_obj.join("database/migrations");

    println!("\nDirectory structure:");
    println!("  {} app/", if app_dir.exists() { "✓" } else { "❌" });
    println!("  {} app/Models/", if models_dir.exists() { "✓" } else { "❌" });
    println!("  {} app/Http/Controllers/", if controllers_dir.exists() { "✓" } else { "❌" });
    println!("  {} database/migrations/", if migrations_dir.exists() { "✓" } else { "❌" });

    println!("\n✓ Project is ready for code generation");

    Ok(())
}

fn handle_init_rules(output: &str, force: bool) -> Result<()> {
    let out_path = Path::new(output);

    let content = r#"# Laravel + Schemly Development Rules
- This project uses Schemly for code generation.
- You have access to the Schemly MCP Server. 
- You must NOT manually create or modify ANY Laravel Models, Migrations, Resource classes, or base Controllers.
- All database and architectural changes MUST be made by editing the `schema.schemly` file and invoking the MCP `generate` tool.
- If asked to create a "Blog Post" feature, write the Schemly schema for it first."#;

    let cursor_path = out_path.join(".cursorrules");
    let windsurf_path = out_path.join(".windsurfrules");

    for path in [&cursor_path, &windsurf_path] {
        if path.exists() && !force {
            println!("⚠ File '{}' already exists. Use --force to overwrite.", path.display());
            continue;
        }
        if let Err(e) = fs::write(path, content) {
            return Err(error::GeneratorError::ModelValidation(format!("Failed to write to {}: {}", path.display(), e)));
        }
        println!("✓ Created {}", path.display());
    }

    Ok(())
}
