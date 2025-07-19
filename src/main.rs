mod config;
mod error;
mod generators;
mod interactive;
mod template;
mod types;
mod utils;
mod validation;

use clap::Parser;
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
#[command(about = "A Larav el code generator from YAML configuration")]
#[command(long_about = "Generate Laravel models, controllers, resources, factories, migrations, and pivot tables from YAML configuration files.

EXAMPLES:
    schemly --config models.yml                    # Generate all components
    schemly --config models.yml --only-models     # Generate only models
    schemly --config models.yml --force           # Overwrite existing files
    schemly --config models.yml --output /path/to/laravel-project

SAFETY:
    By default, existing files are NOT overwritten. Use --force to overwrite.")]
struct Cli {
    #[arg(short, long, help = "Path to YAML configuration file")]
    config: String,

    #[arg(
        short,
        long,
        default_value = ".",
        help = "Laravel project root directory"
    )]
    output: String,

    // Exclusion flags (skip generation)
    #[arg(long, help = "Skip model generation")]
    no_models: bool,

    #[arg(long, help = "Skip controller generation")]
    no_controllers: bool,

    #[arg(long, help = "Skip resource generation")]
    no_resources: bool,

    #[arg(long, help = "Skip factory generation")]
    no_factories: bool,

    #[arg(long, help = "Skip migration generation")]
    no_migrations: bool,

    #[arg(long, help = "Skip pivot table generation")]
    no_pivot_tables: bool,

    // Inclusion flags (generate only specified types)
    #[arg(long, help = "Generate only models")]
    only_models: bool,

    #[arg(long, help = "Generate only controllers")]
    only_controllers: bool,

    #[arg(long, help = "Generate only resources")]
    only_resources: bool,

    #[arg(long, help = "Generate only factories")]
    only_factories: bool,

    #[arg(long, help = "Generate only migrations")]
    only_migrations: bool,

    #[arg(long, help = "Generate only pivot tables")]
    only_pivot_tables: bool,

    #[arg(long, help = "Generate only DTOs")]
    only_dto: bool,

    #[arg(long, help = "Skip DTO generation")]
    no_dto: bool,

    #[arg(long, help = "Use Domain-Driven Design folder structure")]
    ddd: bool,

    #[arg(long, help = "Use traditional Laravel folder structure")]
    no_ddd: bool,

    #[arg(short = 'i', long, help = "Interactive mode for selecting models and components")]
    interactive: bool,

    #[arg(long, help = "Force overwrite existing files")]
    force: bool,
}

struct LaravelGenerator {
    config: Config,
}

impl LaravelGenerator {
    pub fn new(yaml_content: &str) -> Result<Self> {
        let config: Config = serde_yaml::from_str(yaml_content)?;
        config.validate()?;
        Ok(LaravelGenerator { config })
    }

    pub fn from_file(file_path: &str) -> Result<Self> {
        let yaml_content = fs::read_to_string(file_path)?;
        Self::new(&yaml_content)
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
                let result = self.generate_model(model)?;
                self.update_stats(&mut stats, result);
            }

            if self.config.generate_migrations {
                let result = self.generate_migration(model)?;
                self.update_stats(&mut stats, result);
            }

            if self.config.generate_controllers {
                let result = self.generate_controller(model)?;
                self.update_stats(&mut stats, result);
            }

            if self.config.generate_resources {
                let result = self.generate_resource(model)?;
                self.update_stats(&mut stats, result);
            }

            if self.config.generate_factories {
                let result = self.generate_factory(model)?;
                self.update_stats(&mut stats, result);
            }

            if self.config.generate_dto {
                let result = self.generate_dto(model)?;
                self.update_stats(&mut stats, result);
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

    fn generate_model(&self, model: &types::ModelDefinition) -> Result<WriteResult> {
        let generator = model_generator::ModelGenerator;
        let content = generator.generate(model, &self.config)?;
        let file_path = generator.get_file_path(model, &self.config);

        let result = safe_write_file(&file_path, &content, self.config.force_overwrite)?;
        match &result {
            WriteResult::Written => println!("Generated model: {}", model.name),
            WriteResult::Skipped => {
                println!("Warning: File already exists, skipping: {}", file_path)
            }
            WriteResult::Error(e) => println!("Error writing {}: {}", file_path, e),
        }
        Ok(result)
    }

    fn generate_migration(&self, model: &types::ModelDefinition) -> Result<WriteResult> {
        let generator = migration_generator::MigrationGenerator;
        let content = generator.generate(model, &self.config)?;
        let file_path = generator.get_file_path(model, &self.config);

        let result = safe_write_file(&file_path, &content, self.config.force_overwrite)?;
        match &result {
            WriteResult::Written => println!("Generated migration for table: {}", model.table),
            WriteResult::Skipped => {
                println!("Warning: File already exists, skipping: {}", file_path)
            }
            WriteResult::Error(e) => println!("Error writing {}: {}", file_path, e),
        }
        Ok(result)
    }

    fn generate_controller(&self, model: &types::ModelDefinition) -> Result<WriteResult> {
        let generator = controller_generator::ControllerGenerator;
        let content = generator.generate(model, &self.config)?;
        let file_path = generator.get_file_path(model, &self.config);

        let result = safe_write_file(&file_path, &content, self.config.force_overwrite)?;
        match &result {
            WriteResult::Written => println!("Generated controller: {}Controller", model.name),
            WriteResult::Skipped => {
                println!("Warning: File already exists, skipping: {}", file_path)
            }
            WriteResult::Error(e) => println!("Error writing {}: {}", file_path, e),
        }
        Ok(result)
    }

    fn generate_resource(&self, model: &types::ModelDefinition) -> Result<WriteResult> {
        let generator = resource_generator::ResourceGenerator;
        let content = generator.generate(model, &self.config)?;
        let file_path = generator.get_file_path(model, &self.config);

        let result = safe_write_file(&file_path, &content, self.config.force_overwrite)?;
        match &result {
            WriteResult::Written => println!("Generated resource: {}Resource", model.name),
            WriteResult::Skipped => {
                println!("Warning: File already exists, skipping: {}", file_path)
            }
            WriteResult::Error(e) => println!("Error writing {}: {}", file_path, e),
        }
        Ok(result)
    }

    fn generate_factory(&self, model: &types::ModelDefinition) -> Result<WriteResult> {
        let generator = factory_generator::FactoryGenerator;
        let content = generator.generate(model, &self.config)?;
        let file_path = generator.get_file_path(model, &self.config);

        let result = safe_write_file(&file_path, &content, self.config.force_overwrite)?;
        match &result {
            WriteResult::Written => println!("Generated factory: {}Factory", model.name),
            WriteResult::Skipped => {
                println!("Warning: File already exists, skipping: {}", file_path)
            }
            WriteResult::Error(e) => println!("Error writing {}: {}", file_path, e),
        }
        Ok(result)
    }

    fn generate_dto(&self, model: &types::ModelDefinition) -> Result<WriteResult> {
        let generator = dto_generator::DtoGenerator;
        let content = generator.generate(model, &self.config)?;
        let file_path = generator.get_file_path(model, &self.config);

        let result = safe_write_file(&file_path, &content, self.config.force_overwrite)?;
        match &result {
            WriteResult::Written => println!("Generated DTO: {}DTO", model.name),
            WriteResult::Skipped => {
                println!("Warning: File already exists, skipping: {}", file_path)
            }
            WriteResult::Error(e) => println!("Error writing {}: {}", file_path, e),
        }
        Ok(result)
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

// CLI validation functions
fn validate_generation_flags(cli: &Cli) -> Result<()> {
    // Check for conflicting DDD flags
    if cli.ddd && cli.no_ddd {
        return Err(error::GeneratorError::ModelValidation(
            "Error: Cannot use both --ddd and --no-ddd flags".to_string(),
        ));
    }

    // Check if at least one component type will be enabled
    if has_only_flags(cli) {
        let enabled_count = [
            cli.only_models,
            cli.only_controllers,
            cli.only_resources,
            cli.only_factories,
            cli.only_migrations,
            cli.only_pivot_tables,
            cli.only_dto,
        ]
        .iter()
        .filter(|&&x| x)
        .count();

        if enabled_count == 0 {
            return Err(error::GeneratorError::ModelValidation(
                "Error: At least one component type must be enabled when using --only-* flags"
                    .to_string(),
            ));
        }
    } else {
        // Check if all components are disabled with --no-* flags
        let disabled_count = [
            cli.no_models,
            cli.no_controllers,
            cli.no_resources,
            cli.no_factories,
            cli.no_migrations,
            cli.no_pivot_tables,
            cli.no_dto,
        ]
        .iter()
        .filter(|&&x| x)
        .count();

        if disabled_count == 7 {
            return Err(error::GeneratorError::ModelValidation(
                "Error: At least one component type must be enabled for generation".to_string(),
            ));
        }
    }

    Ok(())
}

fn has_only_flags(cli: &Cli) -> bool {
    cli.only_models
        || cli.only_controllers
        || cli.only_resources
        || cli.only_factories
        || cli.only_migrations
        || cli.only_pivot_tables
        || cli.only_dto
}

fn get_enabled_components(cli: &Cli) -> Vec<String> {
    let mut enabled = Vec::new();

    if has_only_flags(cli) {
        if cli.only_models {
            enabled.push("models".to_string());
        }
        if cli.only_controllers {
            enabled.push("controllers".to_string());
        }
        if cli.only_resources {
            enabled.push("resources".to_string());
        }
        if cli.only_factories {
            enabled.push("factories".to_string());
        }
        if cli.only_migrations {
            enabled.push("migrations".to_string());
        }
        if cli.only_pivot_tables {
            enabled.push("pivot tables".to_string());
        }
        if cli.only_dto {
            enabled.push("DTOs".to_string());
        }
    } else {
        if !cli.no_models {
            enabled.push("models".to_string());
        }
        if !cli.no_controllers {
            enabled.push("controllers".to_string());
        }
        if !cli.no_resources {
            enabled.push("resources".to_string());
        }
        if !cli.no_factories {
            enabled.push("factories".to_string());
        }
        if !cli.no_migrations {
            enabled.push("migrations".to_string());
        }
        if !cli.no_pivot_tables {
            enabled.push("pivot tables".to_string());
        }
        if !cli.no_dto {
            enabled.push("DTOs".to_string());
        }
    }

    enabled
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Validate CLI flags
    validate_generation_flags(&cli)?;

    let mut generator = LaravelGenerator::from_file(&cli.config)?;

    // Handle interactive mode
    if cli.interactive {
        generator.config = interactive::InteractiveMode::run(generator.config)?;
    }

    // Override config with CLI options
    generator.config.output_dir = cli.output.clone();
    generator.config.force_overwrite = cli.force;

    // Apply DDD structure flags
    if cli.ddd {
        generator.config.use_ddd_structure = true;
    } else if cli.no_ddd {
        generator.config.use_ddd_structure = false;
    }

    // Warn user about force flag
    if cli.force {
        println!("⚠️  Warning: --force flag enabled. Existing files will be overwritten!");
    }

    // Apply generation flags with proper precedence (only if not in interactive mode)
    if !cli.interactive {
        if has_only_flags(&cli) {
            // When --only-* flags are used, disable all then enable only specified
            generator.config.generate_models = cli.only_models;
            generator.config.generate_controllers = cli.only_controllers;
            generator.config.generate_resources = cli.only_resources;
            generator.config.generate_factories = cli.only_factories;
            generator.config.generate_migrations = cli.only_migrations;
            generator.config.generate_pivot_tables = cli.only_pivot_tables;
            generator.config.generate_dto = cli.only_dto;
        } else {
            // When --no-* flags are used, start with defaults and disable specified
            generator.config.generate_models = !cli.no_models;
            generator.config.generate_controllers = !cli.no_controllers;
            generator.config.generate_resources = !cli.no_resources;
            generator.config.generate_factories = !cli.no_factories;
            generator.config.generate_migrations = !cli.no_migrations;
            generator.config.generate_pivot_tables = !cli.no_pivot_tables;
            generator.config.generate_dto = !cli.no_dto;
        }
    }

    // Log which components will be generated
    let enabled_components = get_enabled_components(&cli);
    println!("Generating: {}", enabled_components.join(", "));

    generator.generate_all()?;
    Ok(())
}
