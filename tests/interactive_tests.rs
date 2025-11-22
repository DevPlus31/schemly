use schemly::interactive::InteractiveMode;
use schemly::types::{Config, ModelDefinition, Field, FieldType, FillableGuarded};

fn create_test_config() -> Config {
    Config {
        models: vec![
            ModelDefinition {
                name: "User".to_string(),
                table: "users".to_string(),
                fields: vec![
                    Field {
                        name: "name".to_string(),
                        field_type: FieldType::String,
                        nullable: false,
                        unique: false,
                        default: None,
                        length: Some(255),
                        index: false,
                        enum_values: vec![],
                        decimal_precision: None,
                        unsigned: false,
                        auto_increment: false,
                        primary: false,
                        comment: None,
                        validation_rules: vec![],
                        cast_type: None,
                    },
                ],
                timestamps: true,
                soft_deletes: false,
                relationships: vec![],
                pivot_tables: vec![],
                validation_rules: vec![],
                traits: vec![],
                fillable_guarded: FillableGuarded::All,
            },
            ModelDefinition {
                name: "Post".to_string(),
                table: "posts".to_string(),
                fields: vec![
                    Field {
                        name: "title".to_string(),
                        field_type: FieldType::String,
                        nullable: false,
                        unique: false,
                        default: None,
                        length: Some(255),
                        index: false,
                        enum_values: vec![],
                        decimal_precision: None,
                        unsigned: false,
                        auto_increment: false,
                        primary: false,
                        comment: None,
                        validation_rules: vec![],
                        cast_type: None,
                    },
                ],
                timestamps: true,
                soft_deletes: false,
                relationships: vec![],
                pivot_tables: vec![],
                validation_rules: vec![],
                traits: vec![],
                fillable_guarded: FillableGuarded::All,
            },
        ],
        output_dir: "/tmp/test".to_string(),
        namespace: "App\\Models".to_string(),
        generate_models: true,
        generate_controllers: true,
        generate_resources: true,
        generate_factories: true,
        generate_migrations: true,
        generate_pivot_tables: true,
        generate_validation_rules: true,
        generate_dto: false, // Initially disabled
        use_ddd_structure: false, // Initially disabled
        database_engine: "mysql".to_string(),
        force_overwrite: false,
    }
}

#[test]
fn test_interactive_mode_component_selection() {
    // This test verifies that the ComponentSelection struct works correctly
    use schemly::interactive::ComponentSelection;

    let selection = ComponentSelection::default();
    
    // Test default values
    assert_eq!(selection.generate_models, true);
    assert_eq!(selection.generate_controllers, true);
    assert_eq!(selection.generate_resources, true);
    assert_eq!(selection.generate_factories, true);
    assert_eq!(selection.generate_migrations, true);
    assert_eq!(selection.generate_pivot_tables, true);
    assert_eq!(selection.generate_dto, false); // DTOs are disabled by default
}

#[test]
fn test_interactive_mode_config_structure() {
    let config = create_test_config();
    
    // Test that config has the expected structure
    assert_eq!(config.models.len(), 2);
    assert_eq!(config.models[0].name, "User");
    assert_eq!(config.models[1].name, "Post");
    assert_eq!(config.use_ddd_structure, false);
    assert_eq!(config.generate_dto, false);
}

// Note: Interactive mode tests that require user input are difficult to test
// in an automated way. The actual interactive functionality would need to be
// tested manually or with more complex mocking of stdin/stdout.

#[test]
fn test_interactive_mode_exists() {
    // This test just verifies that the InteractiveMode struct exists
    // and can be referenced (compilation test)
    let _interactive_mode = InteractiveMode;
    
    // If this compiles, the InteractiveMode struct is properly defined
    assert!(true);
}

#[cfg(test)]
mod manual_tests {
    // These are manual tests that can be run to verify interactive functionality
    // They are not run automatically but serve as documentation
    
    #[allow(dead_code)]
    fn manual_test_interactive_mode() {
        // To manually test interactive mode, run:
        // cargo run -- --config test_ddd.yaml --interactive --output /tmp/test-interactive
        // 
        // This will:
        // 1. Prompt for DDD structure (y/n)
        // 2. Show model selection with User and Post
        // 3. Show component selection with visual indicators
        // 4. Display generation summary
        // 5. Confirm before generation
    }
    
    #[allow(dead_code)]
    fn manual_test_ddd_structure() {
        // To manually test DDD structure, run:
        // cargo run -- --config test_ddd.yaml --ddd --only-dto --output /tmp/test-ddd
        //
        // Expected output structure:
        // /tmp/test-ddd/app/Domain/User/DTOs/UserDTO.php
        // /tmp/test-ddd/app/Domain/Post/DTOs/PostDTO.php
    }
    
    #[allow(dead_code)]
    fn manual_test_traditional_structure() {
        // To manually test traditional structure, run:
        // cargo run -- --config test_ddd.yaml --no-ddd --only-dto --output /tmp/test-traditional
        //
        // Expected output structure:
        // /tmp/test-traditional/app/DTOs/UserDTO.php
        // /tmp/test-traditional/app/DTOs/PostDTO.php
    }
}

#[test]
fn test_config_clone_functionality() {
    // Test that Config can be cloned (needed for interactive mode)
    let config = create_test_config();
    let cloned_config = config.clone();
    
    assert_eq!(config.models.len(), cloned_config.models.len());
    assert_eq!(config.use_ddd_structure, cloned_config.use_ddd_structure);
    assert_eq!(config.generate_dto, cloned_config.generate_dto);
    assert_eq!(config.models[0].name, cloned_config.models[0].name);
}

#[test]
fn test_model_definition_clone_functionality() {
    // Test that ModelDefinition can be cloned (needed for model selection)
    let config = create_test_config();
    let model = &config.models[0];
    let cloned_model = model.clone();
    
    assert_eq!(model.name, cloned_model.name);
    assert_eq!(model.table, cloned_model.table);
    assert_eq!(model.timestamps, cloned_model.timestamps);
    assert_eq!(model.fields.len(), cloned_model.fields.len());
}

#[test]
fn test_interactive_mode_integration_points() {
    // Test that all the integration points for interactive mode exist
    let config = create_test_config();
    
    // Test that config has all the fields that interactive mode modifies
    assert!(config.models.len() > 0); // Models for selection
    
    // Test boolean flags that interactive mode can toggle
    let _use_ddd = config.use_ddd_structure;
    let _gen_models = config.generate_models;
    let _gen_controllers = config.generate_controllers;
    let _gen_resources = config.generate_resources;
    let _gen_factories = config.generate_factories;
    let _gen_migrations = config.generate_migrations;
    let _gen_pivot_tables = config.generate_pivot_tables;
    let _gen_dto = config.generate_dto;
    
    // If this compiles and runs, all integration points exist
    assert!(true);
}
