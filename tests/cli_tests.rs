use std::process::Command;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_cli_help() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8(output.stdout).unwrap();
    
    // Check that help contains key information
    assert!(stdout.contains("Usage: schemly"));
    assert!(stdout.contains("--config"));
    assert!(stdout.contains("--output"));
    assert!(stdout.contains("--ddd"));
    assert!(stdout.contains("--no-ddd"));
    assert!(stdout.contains("--interactive"));
    assert!(stdout.contains("--only-dto"));
    assert!(stdout.contains("--no-dto"));
}

#[test]
fn test_cli_dto_generation_traditional() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().to_str().unwrap();
    
    // Create a simple test YAML file
    let yaml_content = r#"
models:
  - name: TestModel
    table: test_models
    timestamps: true
    fields:
      - name: name
        type: string
        nullable: false
      - name: email
        type: string
        nullable: false
        unique: true
"#;
    
    let yaml_file = temp_dir.path().join("test.yaml");
    fs::write(&yaml_file, yaml_content).unwrap();
    
    // Run the CLI command
    let output = Command::new("cargo")
        .args(&[
            "run", "--",
            "--config", yaml_file.to_str().unwrap(),
            "--no-ddd",
            "--only-dto",
            "--output", output_path
        ])
        .output()
        .expect("Failed to execute command");

    // Check that command succeeded
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    
    // Check that DTO file was created
    let dto_file = format!("{}/app/DTOs/TestModelDTO.php", output_path);
    assert!(std::path::Path::new(&dto_file).exists(), "DTO file was not created");
    
    // Check file content
    let content = fs::read_to_string(&dto_file).unwrap();
    assert!(content.contains("namespace App\\DTOs;"));
    assert!(content.contains("class TestModelDTO {"));
    assert!(content.contains("public string $name"));
    assert!(content.contains("public string $email"));
}

#[test]
fn test_cli_dto_generation_ddd() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().to_str().unwrap();
    
    // Create a simple test YAML file
    let yaml_content = r#"
models:
  - name: TestModel
    table: test_models
    timestamps: true
    fields:
      - name: name
        type: string
        nullable: false
      - name: email
        type: string
        nullable: false
        unique: true
"#;
    
    let yaml_file = temp_dir.path().join("test.yaml");
    fs::write(&yaml_file, yaml_content).unwrap();
    
    // Run the CLI command with DDD structure
    let output = Command::new("cargo")
        .args(&[
            "run", "--",
            "--config", yaml_file.to_str().unwrap(),
            "--ddd",
            "--only-dto",
            "--output", output_path
        ])
        .output()
        .expect("Failed to execute command");

    // Check that command succeeded
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    
    // Check that DTO file was created in DDD structure
    let dto_file = format!("{}/app/Domain/TestModel/DTOs/TestModelDTO.php", output_path);
    assert!(std::path::Path::new(&dto_file).exists(), "DTO file was not created in DDD structure");
    
    // Check file content
    let content = fs::read_to_string(&dto_file).unwrap();
    assert!(content.contains("namespace App\\Domain\\TestModel\\DTOs;"));
    assert!(content.contains("class TestModelDTO {"));
    assert!(content.contains("public string $name"));
    assert!(content.contains("public string $email"));
}

#[test]
fn test_cli_validation_error() {
    // Test with invalid YAML file
    let temp_dir = TempDir::new().unwrap();
    let yaml_file = temp_dir.path().join("invalid.yaml");
    fs::write(&yaml_file, "invalid: yaml: content: [").unwrap();
    
    let output = Command::new("cargo")
        .args(&[
            "run", "--",
            "--config", yaml_file.to_str().unwrap(),
            "--only-dto"
        ])
        .output()
        .expect("Failed to execute command");

    // Command should fail with invalid YAML
    assert!(!output.status.success());
}

#[test]
fn test_cli_conflicting_flags() {
    let temp_dir = TempDir::new().unwrap();
    let yaml_content = r#"
models:
  - name: TestModel
    table: test_models
    fields:
      - name: name
        type: string
"#;
    
    let yaml_file = temp_dir.path().join("test.yaml");
    fs::write(&yaml_file, yaml_content).unwrap();
    
    // Test conflicting DDD flags
    let output = Command::new("cargo")
        .args(&[
            "run", "--",
            "--config", yaml_file.to_str().unwrap(),
            "--ddd",
            "--no-ddd",
            "--only-dto"
        ])
        .output()
        .expect("Failed to execute command");

    // Command should fail with conflicting flags
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Cannot use both --ddd and --no-ddd flags"));
}

#[test]
fn test_cli_no_components_selected() {
    let temp_dir = TempDir::new().unwrap();
    let yaml_content = r#"
models:
  - name: TestModel
    table: test_models
    fields:
      - name: name
        type: string
"#;
    
    let yaml_file = temp_dir.path().join("test.yaml");
    fs::write(&yaml_file, yaml_content).unwrap();
    
    // Test with all components disabled
    let output = Command::new("cargo")
        .args(&[
            "run", "--",
            "--config", yaml_file.to_str().unwrap(),
            "--no-models",
            "--no-controllers",
            "--no-resources",
            "--no-factories",
            "--no-migrations",
            "--no-pivot-tables",
            "--no-dto"
        ])
        .output()
        .expect("Failed to execute command");

    // Command should fail when no components are selected
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("At least one component type must be enabled"));
}

#[test]
fn test_cli_file_overwrite_protection() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().to_str().unwrap();
    
    let yaml_content = r#"
models:
  - name: TestModel
    table: test_models
    fields:
      - name: name
        type: string
"#;
    
    let yaml_file = temp_dir.path().join("test.yaml");
    fs::write(&yaml_file, yaml_content).unwrap();
    
    // Create the DTO file first
    let dto_dir = format!("{}/app/DTOs", output_path);
    fs::create_dir_all(&dto_dir).unwrap();
    let dto_file = format!("{}/TestModelDTO.php", dto_dir);
    fs::write(&dto_file, "<?php\n// Existing file").unwrap();
    
    // Run command without --force flag
    let output = Command::new("cargo")
        .args(&[
            "run", "--",
            "--config", yaml_file.to_str().unwrap(),
            "--no-ddd",
            "--only-dto",
            "--output", output_path
        ])
        .output()
        .expect("Failed to execute command");

    // Command should succeed but skip existing file
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("skipped"));
    
    // File should still contain original content
    let content = fs::read_to_string(&dto_file).unwrap();
    assert!(content.contains("// Existing file"));
}

#[test]
fn test_cli_force_overwrite() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().to_str().unwrap();
    
    let yaml_content = r#"
models:
  - name: TestModel
    table: test_models
    fields:
      - name: name
        type: string
"#;
    
    let yaml_file = temp_dir.path().join("test.yaml");
    fs::write(&yaml_file, yaml_content).unwrap();
    
    // Create the DTO file first
    let dto_dir = format!("{}/app/DTOs", output_path);
    fs::create_dir_all(&dto_dir).unwrap();
    let dto_file = format!("{}/TestModelDTO.php", dto_dir);
    fs::write(&dto_file, "<?php\n// Existing file").unwrap();
    
    // Run command with --force flag
    let output = Command::new("cargo")
        .args(&[
            "run", "--",
            "--config", yaml_file.to_str().unwrap(),
            "--no-ddd",
            "--only-dto",
            "--output", output_path,
            "--force"
        ])
        .output()
        .expect("Failed to execute command");

    // Command should succeed and overwrite file
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Generated DTO: TestModelDTO"));
    
    // File should contain new generated content
    let content = fs::read_to_string(&dto_file).unwrap();
    assert!(content.contains("class TestModelDTO"));
    assert!(!content.contains("// Existing file"));
}
