use crate::types::{Config, ModelDefinition};
use crate::validation::Validator;
use std::io::{self, Write};

/// Interactive mode for selecting models and components
pub struct InteractiveMode;

#[derive(Debug, Clone)]
pub struct ComponentSelection {
    pub generate_models: bool,
    pub generate_controllers: bool,
    pub generate_resources: bool,
    pub generate_factories: bool,
    pub generate_migrations: bool,
    pub generate_pivot_tables: bool,
    pub generate_dto: bool,
}

impl Default for ComponentSelection {
    fn default() -> Self {
        ComponentSelection {
            generate_models: true,
            generate_controllers: true,
            generate_resources: true,
            generate_factories: true,
            generate_migrations: true,
            generate_pivot_tables: true,
            generate_dto: false,
        }
    }
}

impl InteractiveMode {
    /// Run interactive mode and return updated config
    pub fn run(mut config: Config) -> crate::error::Result<Config> {
        println!("🚀 Welcome to Schemly Interactive Mode!");
        println!("═══════════════════════════════════════");
        println!();

        // Step 1: Configure DDD structure
        config.use_ddd_structure = Self::prompt_ddd_structure(config.use_ddd_structure)?;
        println!();

        // Step 2: Select models to generate
        let selected_models = Self::select_models(&config.models)?;
        println!();

        // Step 3: Select components for each model
        let component_selection = Self::select_components(&config)?;
        println!();

        // Apply selections to config
        config.models = selected_models;
        config.generate_models = component_selection.generate_models;
        config.generate_controllers = component_selection.generate_controllers;
        config.generate_resources = component_selection.generate_resources;
        config.generate_factories = component_selection.generate_factories;
        config.generate_migrations = component_selection.generate_migrations;
        config.generate_pivot_tables = component_selection.generate_pivot_tables;
        config.generate_dto = component_selection.generate_dto;

        // Step 4: Show summary
        Self::show_summary(&config);

        Ok(config)
    }

    /// Prompt for DDD structure preference
    fn prompt_ddd_structure(current: bool) -> crate::error::Result<bool> {
        let current_str = if current { "enabled" } else { "disabled" };
        println!("📁 Domain-Driven Design Structure");
        println!("   Current setting: {} ({})", current_str, if current { "✓" } else { "✗" });
        println!("   DDD structure organizes files in app/Domain/{{ModelName}}/ folders");
        println!();
        
        loop {
            print!("   Enable DDD structure? [y/N]: ");
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim().to_lowercase();
            
            match input.as_str() {
                "y" | "yes" => return Ok(true),
                "n" | "no" | "" => return Ok(false),
                _ => println!("   Please enter 'y' for yes or 'n' for no."),
            }
        }
    }

    /// Select which models to generate
    fn select_models(models: &[ModelDefinition]) -> crate::error::Result<Vec<ModelDefinition>> {
        println!("📋 Model Selection");
        println!("   Select which models to generate:");
        println!();

        if models.is_empty() {
            println!("   ⚠️  No models found in configuration file.");
            return Ok(Vec::new());
        }

        let mut selected_models = Vec::new();

        for (index, model) in models.iter().enumerate() {
            // Validate model before offering it for selection
            if let Err(e) = Validator::validate_model(model) {
                println!("   [{}] {} (table: {}) - ❌ INVALID: {}",
                         index + 1, model.name, model.table, e);
                continue;
            }

            loop {
                print!("   [{}] {} (table: {}) - Generate? [Y/n]: ",
                       index + 1, model.name, model.table);
                io::stdout().flush()?;

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                let input = input.trim().to_lowercase();

                match input.as_str() {
                    "y" | "yes" | "" => {
                        selected_models.push(model.clone());
                        println!("       ✓ {} selected", model.name);
                        break;
                    },
                    "n" | "no" => {
                        println!("       ✗ {} skipped", model.name);
                        break;
                    },
                    _ => println!("       Please enter 'y' for yes or 'n' for no."),
                }
            }
        }

        if selected_models.is_empty() {
            println!("   ⚠️  No models selected. Nothing will be generated.");
        } else {
            println!("   ✓ {} model(s) selected for generation", selected_models.len());
        }

        Ok(selected_models)
    }

    /// Select which components to generate
    fn select_components(config: &Config) -> crate::error::Result<ComponentSelection> {
        println!("🔧 Component Selection");
        println!("   Select which components to generate:");
        println!("   (Components marked with ⚠️  are currently disabled in config)");
        println!();

        let mut selection = ComponentSelection::default();

        // Show current config status and prompt for each component
        selection.generate_models = Self::prompt_component(
            "Models", config.generate_models, "Eloquent model classes")?;
        
        selection.generate_controllers = Self::prompt_component(
            "Controllers", config.generate_controllers, "Resource controllers with CRUD operations")?;
        
        selection.generate_resources = Self::prompt_component(
            "Resources", config.generate_resources, "API resource classes for JSON responses")?;
        
        selection.generate_factories = Self::prompt_component(
            "Factories", config.generate_factories, "Model factories for testing and seeding")?;
        
        selection.generate_migrations = Self::prompt_component(
            "Migrations", config.generate_migrations, "Database migration files")?;
        
        selection.generate_pivot_tables = Self::prompt_component(
            "Pivot Tables", config.generate_pivot_tables, "Pivot table migrations for many-to-many relationships")?;
        
        selection.generate_dto = Self::prompt_component(
            "DTOs", config.generate_dto, "Data Transfer Objects with toArray/fromArray methods")?;

        Ok(selection)
    }

    /// Prompt for a single component
    fn prompt_component(name: &str, current: bool, description: &str) -> crate::error::Result<bool> {
        let status_icon = if current { "✓" } else { "⚠️ " };
        let default_char = if current { "Y" } else { "N" };
        let opposite_char = if current { "n" } else { "y" };
        
        loop {
            print!("   {} {} - {} [{}{}]: ", 
                   status_icon, name, description, default_char, opposite_char.to_lowercase());
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim().to_lowercase();
            
            match input.as_str() {
                "y" | "yes" => return Ok(true),
                "n" | "no" => return Ok(false),
                "" => return Ok(current), // Use current config as default
                _ => println!("       Please enter 'y' for yes or 'n' for no."),
            }
        }
    }

    /// Show generation summary
    fn show_summary(config: &Config) {
        println!("📊 Generation Summary");
        println!("═══════════════════════");
        println!("   Structure: {}", if config.use_ddd_structure { "Domain-Driven Design" } else { "Traditional Laravel" });
        println!("   Models to generate: {}", config.models.len());
        
        let mut components = Vec::new();
        if config.generate_models { components.push("Models"); }
        if config.generate_controllers { components.push("Controllers"); }
        if config.generate_resources { components.push("Resources"); }
        if config.generate_factories { components.push("Factories"); }
        if config.generate_migrations { components.push("Migrations"); }
        if config.generate_pivot_tables { components.push("Pivot Tables"); }
        if config.generate_dto { components.push("DTOs"); }
        
        println!("   Components: {}", components.join(", "));
        
        if !config.models.is_empty() {
            println!("   Selected models:");
            for model in &config.models {
                println!("     • {} ({})", model.name, model.table);
            }
        }
        
        println!();
        print!("   Continue with generation? [Y/n]: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_lowercase();
        
        if input == "n" || input == "no" {
            println!("   Generation cancelled.");
            std::process::exit(0);
        }
        
        println!("   🚀 Starting generation...");
        println!();
    }
}
