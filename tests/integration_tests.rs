use schemly::types::{Config, ModelDefinition, Field, FieldType, FillableGuarded, DecimalPrecision};
use schemly::generators::{Generator, dto_generator::DtoGenerator};
use std::fs;
use tempfile::TempDir;

fn create_test_model() -> ModelDefinition {
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
            Field {
                name: "email".to_string(),
                field_type: FieldType::String,
                nullable: false,
                unique: true,
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
            Field {
                name: "age".to_string(),
                field_type: FieldType::Integer,
                nullable: true,
                unique: false,
                default: None,
                length: None,
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
    }
}

fn create_test_config(output_dir: &str, use_ddd: bool) -> Config {
    Config {
        models: vec![create_test_model()],
        output_dir: output_dir.to_string(),
        namespace: "App\\Models".to_string(),
        generate_models: true,
        generate_controllers: true,
        generate_resources: true,
        generate_factories: true,
        generate_migrations: true,
        generate_pivot_tables: true,
        generate_validation_rules: true,
        generate_dto: true,
        use_ddd_structure: use_ddd,
        database_engine: "mysql".to_string(),
        force_overwrite: false,
    }
}

#[test]
fn test_dto_generation_integration_traditional() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().to_str().unwrap();
    
    let model = create_test_model();
    let config = create_test_config(output_path, false);
    
    let generator = DtoGenerator;
    let content = generator.generate(&model, &config).unwrap();
    let file_path = generator.get_file_path(&model, &config);
    
    // Create directory structure
    let dto_dir = format!("{}/app/DTOs", output_path);
    fs::create_dir_all(&dto_dir).unwrap();
    
    // Write file
    fs::write(&file_path, &content).unwrap();
    
    // Verify file exists
    assert!(std::path::Path::new(&file_path).exists());
    
    // Verify file content
    let written_content = fs::read_to_string(&file_path).unwrap();
    assert!(written_content.contains("namespace App\\DTOs;"));
    assert!(written_content.contains("class UserDTO {"));
    assert!(written_content.contains("public int $id"));
    assert!(written_content.contains("public string $name"));
    assert!(written_content.contains("public string $email"));
    assert!(written_content.contains("public ?int $age"));
}

#[test]
fn test_dto_generation_integration_ddd() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().to_str().unwrap();
    
    let model = create_test_model();
    let config = create_test_config(output_path, true);
    
    let generator = DtoGenerator;
    let content = generator.generate(&model, &config).unwrap();
    let file_path = generator.get_file_path(&model, &config);
    
    // Create directory structure
    let dto_dir = format!("{}/app/Domain/User/DTOs", output_path);
    fs::create_dir_all(&dto_dir).unwrap();
    
    // Write file
    fs::write(&file_path, &content).unwrap();
    
    // Verify file exists
    assert!(std::path::Path::new(&file_path).exists());
    
    // Verify file content
    let written_content = fs::read_to_string(&file_path).unwrap();
    assert!(written_content.contains("namespace App\\Domain\\User\\DTOs;"));
    assert!(written_content.contains("class UserDTO {"));
    assert!(written_content.contains("public int $id"));
    assert!(written_content.contains("public string $name"));
    assert!(written_content.contains("public string $email"));
    assert!(written_content.contains("public ?int $age"));
}

#[test]
fn test_dto_content_structure() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().to_str().unwrap();
    
    let model = create_test_model();
    let config = create_test_config(output_path, false);
    
    let generator = DtoGenerator;
    let content = generator.generate(&model, &config).unwrap();
    
    // Test constructor structure
    assert!(content.contains("public function __construct"));
    assert!(content.contains("public int $id,"));
    assert!(content.contains("public string $name,"));
    assert!(content.contains("public string $email,"));
    assert!(content.contains("public ?int $age,"));
    assert!(content.contains("public ?string $created_at,"));
    assert!(content.contains("public ?string $updated_at"));
    
    // Test fromArray method
    assert!(content.contains("public static function fromArray(array $data): self"));
    assert!(content.contains("return new self("));
    assert!(content.contains("$data['id'],"));
    assert!(content.contains("$data['name'],"));
    assert!(content.contains("$data['email'],"));
    assert!(content.contains("$data['age'],"));
    assert!(content.contains("$data['created_at'],"));
    assert!(content.contains("$data['updated_at']"));
    
    // Test toArray method
    assert!(content.contains("public function toArray(): array"));
    assert!(content.contains("return ["));
    assert!(content.contains("'id' => $this->id,"));
    assert!(content.contains("'name' => $this->name,"));
    assert!(content.contains("'email' => $this->email,"));
    assert!(content.contains("'age' => $this->age,"));
    assert!(content.contains("'created_at' => $this->created_at,"));
    assert!(content.contains("'updated_at' => $this->updated_at"));
}

#[test]
fn test_dto_with_soft_deletes() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().to_str().unwrap();
    
    let mut model = create_test_model();
    model.soft_deletes = true;
    let config = create_test_config(output_path, false);
    
    let generator = DtoGenerator;
    let content = generator.generate(&model, &config).unwrap();
    
    // Test soft delete field is included
    assert!(content.contains("public ?string $deleted_at"));
    assert!(content.contains("$data['deleted_at']"));
    assert!(content.contains("'deleted_at' => $this->deleted_at"));
}

#[test]
fn test_dto_without_timestamps() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().to_str().unwrap();
    
    let mut model = create_test_model();
    model.timestamps = false;
    let config = create_test_config(output_path, false);
    
    let generator = DtoGenerator;
    let content = generator.generate(&model, &config).unwrap();
    
    // Test timestamp fields are not included
    assert!(!content.contains("created_at"));
    assert!(!content.contains("updated_at"));
}

#[test]
fn test_different_field_types() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().to_str().unwrap();
    
    let mut model = create_test_model();
    model.fields = vec![
        Field {
            name: "is_active".to_string(),
            field_type: FieldType::Boolean,
            nullable: false,
            unique: false,
            default: None,
            length: None,
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
        Field {
            name: "price".to_string(),
            field_type: FieldType::Decimal,
            nullable: true,
            unique: false,
            default: None,
            length: None,
            index: false,
            enum_values: vec![],
            decimal_precision: Some(DecimalPrecision { precision: 8, scale: 2 }),
            unsigned: false,
            auto_increment: false,
            primary: false,
            comment: None,
            validation_rules: vec![],
            cast_type: None,
        },
        Field {
            name: "metadata".to_string(),
            field_type: FieldType::Json,
            nullable: true,
            unique: false,
            default: None,
            length: None,
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
    ];
    
    let config = create_test_config(output_path, false);
    
    let generator = DtoGenerator;
    let content = generator.generate(&model, &config).unwrap();
    
    // Test different PHP type hints
    assert!(content.contains("public bool $is_active"));
    assert!(content.contains("public ?float $price"));
    assert!(content.contains("public ?array $metadata"));
}

#[test]
fn test_validation_error_integration_invalid_model_name() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().to_str().unwrap();

    let mut model = create_test_model();
    model.name = "123InvalidName".to_string(); // Invalid: starts with number
    let config = create_test_config(output_path, false);

    let generator = DtoGenerator;
    let result = generator.generate(&model, &config);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("must start with a letter"));
}

#[test]
fn test_validation_error_integration_duplicate_fields() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().to_str().unwrap();

    let mut model = create_test_model();
    // Add duplicate field name
    let duplicate_field = model.fields[0].clone();
    model.fields.push(duplicate_field);
    let config = create_test_config(output_path, false);

    let generator = DtoGenerator;
    let result = generator.generate(&model, &config);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Duplicate field name"));
}

#[test]
fn test_validation_error_integration_empty_model() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().to_str().unwrap();

    let mut model = create_test_model();
    model.fields = vec![]; // No fields
    model.timestamps = false; // No timestamps
    let config = create_test_config(output_path, false);

    let generator = DtoGenerator;
    let result = generator.generate(&model, &config);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("must have at least one field or timestamps"));
}

#[test]
fn test_validation_error_integration_invalid_field_name() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().to_str().unwrap();

    let mut model = create_test_model();
    model.fields[0].name = "invalid-field-name".to_string(); // Invalid: contains hyphen
    let config = create_test_config(output_path, false);

    let generator = DtoGenerator;
    let result = generator.generate(&model, &config);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("invalid characters"));
}

#[test]
fn test_validation_error_integration_empty_output_dir() {
    let _temp_dir = TempDir::new().unwrap();

    let model = create_test_model();
    let mut config = create_test_config("", false);
    config.output_dir = "".to_string(); // Invalid: empty output directory

    let generator = DtoGenerator;
    let result = generator.generate(&model, &config);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Output directory cannot be empty"));
}
