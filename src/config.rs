use crate::error::{GeneratorError, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Application configuration with validation and defaults
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Core generation settings
    pub generation: GenerationConfig,
    
    /// Output and structure settings
    pub output: OutputConfig,
    
    /// Code style and formatting settings
    pub style: StyleConfig,
    
    /// Validation and safety settings
    pub validation: ValidationConfig,
    
    /// Feature flags
    pub features: FeatureConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationConfig {
    pub generate_models: bool,
    pub generate_controllers: bool,
    pub generate_resources: bool,
    pub generate_factories: bool,
    pub generate_migrations: bool,
    pub generate_pivot_tables: bool,
    pub generate_dto: bool,
    pub generate_validation_rules: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    pub output_dir: String,
    pub namespace: String,
    pub use_ddd_structure: bool,
    pub force_overwrite: bool,
    pub create_directories: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleConfig {
    pub use_strict_types: bool,
    pub include_comments: bool,
    pub format_code: bool,
    pub line_ending: LineEnding,
    pub indentation: IndentationStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    pub validate_identifiers: bool,
    pub validate_relationships: bool,
    pub strict_mode: bool,
    pub allow_reserved_words: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureConfig {
    pub experimental_features: bool,
    pub debug_mode: bool,
    pub performance_logging: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LineEnding {
    Unix,    // \n
    Windows, // \r\n
    Mac,     // \r
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IndentationStyle {
    Spaces(u8),
    Tabs,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            generation: GenerationConfig::default(),
            output: OutputConfig::default(),
            style: StyleConfig::default(),
            validation: ValidationConfig::default(),
            features: FeatureConfig::default(),
        }
    }
}

impl Default for GenerationConfig {
    fn default() -> Self {
        Self {
            generate_models: true,
            generate_controllers: false,
            generate_resources: false,
            generate_factories: false,
            generate_migrations: false,
            generate_pivot_tables: false,
            generate_dto: false,
            generate_validation_rules: false,
        }
    }
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            output_dir: "./generated".to_string(),
            namespace: "App".to_string(),
            use_ddd_structure: false,
            force_overwrite: false,
            create_directories: true,
        }
    }
}

impl Default for StyleConfig {
    fn default() -> Self {
        Self {
            use_strict_types: true,
            include_comments: true,
            format_code: false,
            line_ending: LineEnding::Unix,
            indentation: IndentationStyle::Spaces(4),
        }
    }
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            validate_identifiers: true,
            validate_relationships: true,
            strict_mode: false,
            allow_reserved_words: false,
        }
    }
}

impl Default for FeatureConfig {
    fn default() -> Self {
        Self {
            experimental_features: false,
            debug_mode: false,
            performance_logging: false,
        }
    }
}

/// Configuration manager for loading, validating, and saving configurations
pub struct ConfigManager;

impl ConfigManager {
    /// Load configuration from file with fallback to defaults
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<AppConfig> {
        let path = path.as_ref();
        
        if !path.exists() {
            return Ok(AppConfig::default());
        }
        
        let content = std::fs::read_to_string(path)
            .map_err(|e| GeneratorError::Configuration(
                format!("Failed to read config file '{}': {}", path.display(), e)
            ))?;
        
        let config: AppConfig = if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
            serde_yaml::from_str(&content)
                .map_err(|e| GeneratorError::Configuration(
                    format!("Invalid YAML in config file: {}", e)
                ))?
        } else {
            serde_json::from_str(&content)
                .map_err(|e| GeneratorError::Configuration(
                    format!("Invalid JSON in config file: {}", e)
                ))?
        };
        
        Self::validate_config(&config)?;
        Ok(config)
    }
    
    /// Save configuration to file
    pub fn save_to_file<P: AsRef<Path>>(config: &AppConfig, path: P) -> Result<()> {
        let path = path.as_ref();
        
        let content = if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
            serde_yaml::to_string(config)
                .map_err(|e| GeneratorError::Configuration(
                    format!("Failed to serialize config to YAML: {}", e)
                ))?
        } else {
            serde_json::to_string_pretty(config)
                .map_err(|e| GeneratorError::Configuration(
                    format!("Failed to serialize config to JSON: {}", e)
                ))?
        };
        
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| GeneratorError::Configuration(
                    format!("Failed to create config directory: {}", e)
                ))?;
        }
        
        std::fs::write(path, content)
            .map_err(|e| GeneratorError::Configuration(
                format!("Failed to write config file: {}", e)
            ))?;
        
        Ok(())
    }
    
    /// Validate configuration values
    pub fn validate_config(config: &AppConfig) -> Result<()> {
        // Validate output directory
        if config.output.output_dir.is_empty() {
            return Err(GeneratorError::Configuration(
                "Output directory cannot be empty".to_string()
            ));
        }
        
        // Validate namespace
        if config.output.namespace.is_empty() {
            return Err(GeneratorError::Configuration(
                "Namespace cannot be empty".to_string()
            ));
        }
        
        // Validate indentation
        if let IndentationStyle::Spaces(count) = config.style.indentation {
            if count == 0 || count > 8 {
                return Err(GeneratorError::Configuration(
                    "Space indentation must be between 1 and 8".to_string()
                ));
            }
        }
        
        Ok(())
    }

    
    /// Create configuration from environment variables
    pub fn from_env() -> AppConfig {
        let mut config = AppConfig::default();
        
        if let Ok(output_dir) = std::env::var("SCHEMLY_OUTPUT_DIR") {
            config.output.output_dir = output_dir;
        }
        
        if let Ok(namespace) = std::env::var("SCHEMLY_NAMESPACE") {
            config.output.namespace = namespace;
        }
        
        if let Ok(ddd) = std::env::var("SCHEMLY_USE_DDD") {
            config.output.use_ddd_structure = ddd.parse().unwrap_or(false);
        }
        
        if let Ok(force) = std::env::var("SCHEMLY_FORCE_OVERWRITE") {
            config.output.force_overwrite = force.parse().unwrap_or(false);
        }
        
        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.output.output_dir, "./generated");
        assert_eq!(config.output.namespace, "App");
        assert!(!config.output.use_ddd_structure);
        assert!(config.generation.generate_models);
        assert!(!config.generation.generate_controllers);
    }

    #[test]
    fn test_config_validation() {
        let mut config = AppConfig::default();
        assert!(ConfigManager::validate_config(&config).is_ok());
        
        config.output.output_dir = String::new();
        assert!(ConfigManager::validate_config(&config).is_err());
    }

    #[test]
    fn test_save_and_load_json() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.json");
        
        let original_config = AppConfig::default();
        ConfigManager::save_to_file(&original_config, &config_path).unwrap();
        
        let loaded_config = ConfigManager::load_from_file(&config_path).unwrap();
        assert_eq!(loaded_config.output.output_dir, original_config.output.output_dir);
    }

    #[test]
    fn test_from_env() {
        unsafe {
            std::env::set_var("SCHEMLY_OUTPUT_DIR", "/custom/output");
            std::env::set_var("SCHEMLY_NAMESPACE", "Custom\\Namespace");
        }

        let config = ConfigManager::from_env();
        assert_eq!(config.output.output_dir, "/custom/output");
        assert_eq!(config.output.namespace, "Custom\\Namespace");

        unsafe {
            std::env::remove_var("SCHEMLY_OUTPUT_DIR");
            std::env::remove_var("SCHEMLY_NAMESPACE");
        }
    }
}
