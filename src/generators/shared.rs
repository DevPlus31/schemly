use crate::types::{Config, ModelDefinition, FieldType};
use std::fs;

/// Resolves file paths for both traditional Laravel and DDD structures
pub struct PathResolver;

impl PathResolver {
    /// Get the file path for a model component
    pub fn get_model_path(model: &ModelDefinition, config: &Config) -> String {
        if config.use_ddd_structure {
            format!("{}/app/Domain/{}/Models/{}.php", config.output_dir, model.name, model.name)
        } else {
            format!("{}/app/Models/{}.php", config.output_dir, model.name)
        }
    }



    /// Get the file path for a resource
    pub fn get_resource_path(model: &ModelDefinition, config: &Config) -> String {
        if config.use_ddd_structure {
            format!("{}/app/Domain/{}/Resources/{}Resource.php", config.output_dir, model.name, model.name)
        } else {
            format!("{}/app/Http/Resources/{}Resource.php", config.output_dir, model.name)
        }
    }

    /// Get the file path for a factory
    pub fn get_factory_path(model: &ModelDefinition, config: &Config) -> String {
        if config.use_ddd_structure {
            format!("{}/app/Domain/{}/Factories/{}Factory.php", config.output_dir, model.name, model.name)
        } else {
            format!("{}/database/factories/{}Factory.php", config.output_dir, model.name)
        }
    }

    /// Get the file path for a DTO
    pub fn get_dto_path(model: &ModelDefinition, config: &Config) -> String {
        if config.use_ddd_structure {
            format!("{}/app/Domain/{}/DTOs/{}DTO.php", config.output_dir, model.name, model.name)
        } else {
            format!("{}/app/DTOs/{}DTO.php", config.output_dir, model.name)
        }
    }


}

/// Resolves namespaces for both traditional Laravel and DDD structures
pub struct NamespaceResolver;

impl NamespaceResolver {
    /// Get the namespace for a model
    pub fn get_model_namespace(model: &ModelDefinition, config: &Config) -> String {
        if config.use_ddd_structure {
            format!("App\\Domain\\{}\\Models", model.name)
        } else {
            config.namespace.clone()
        }
    }



    /// Get the namespace for a resource
    pub fn get_resource_namespace(model: &ModelDefinition, config: &Config) -> String {
        if config.use_ddd_structure {
            format!("App\\Domain\\{}\\Resources", model.name)
        } else {
            "App\\Http\\Resources".to_string()
        }
    }

    /// Get the namespace for a factory
    pub fn get_factory_namespace(model: &ModelDefinition, config: &Config) -> String {
        if config.use_ddd_structure {
            format!("App\\Domain\\{}\\Factories", model.name)
        } else {
            "Database\\Factories".to_string()
        }
    }

    /// Get the namespace for a DTO
    pub fn get_dto_namespace(model: &ModelDefinition, config: &Config) -> String {
        if config.use_ddd_structure {
            format!("App\\Domain\\{}\\DTOs", model.name)
        } else {
            "App\\DTOs".to_string()
        }
    }
}

/// Creates directories for both traditional Laravel and DDD structures
pub struct DirectoryCreator;

impl DirectoryCreator {
    /// Create all necessary directories for a model
    pub fn create_model_directories(model: &ModelDefinition, config: &Config) -> crate::error::Result<()> {
        if config.use_ddd_structure {
            let base_domain_dir = format!("{}/app/Domain/{}", config.output_dir, model.name);
            
            let dirs = [
                &base_domain_dir,
                &format!("{}/Models", base_domain_dir),
                &format!("{}/Resources", base_domain_dir),
                &format!("{}/Factories", base_domain_dir),
                &format!("{}/DTOs", base_domain_dir),
            ];

            for dir in dirs {
                fs::create_dir_all(dir)?;
            }
        } else {
            let dirs = [
                &format!("{}/app/Models", config.output_dir),
                &format!("{}/app/Http/Resources", config.output_dir),
                &format!("{}/database/factories", config.output_dir),
                &format!("{}/app/DTOs", config.output_dir),
            ];

            for dir in dirs {
                fs::create_dir_all(dir)?;
            }
        }

        // Always create these directories (they don't change with DDD)
        let common_dirs = [
            &format!("{}/app/Http/Controllers", config.output_dir),
            &format!("{}/database/migrations", config.output_dir),
        ];

        for dir in common_dirs {
            fs::create_dir_all(dir)?;
        }

        Ok(())
    }
}

/// Utility functions for field type handling
pub struct FieldTypeHelper;

impl FieldTypeHelper {
    /// Convert field type to PHP type hint
    pub fn to_php_type_hint(field_type: &FieldType) -> &'static str {
        match field_type {
            FieldType::String | FieldType::Text | FieldType::LongText | FieldType::MediumText => "string",
            FieldType::Integer | FieldType::BigInteger | FieldType::TinyInteger | 
            FieldType::SmallInteger | FieldType::MediumInteger => "int",
            FieldType::Float | FieldType::Decimal => "float",
            FieldType::Boolean => "bool",
            FieldType::Json => "array",
            FieldType::Date | FieldType::DateTime | FieldType::Timestamp => "string",
            FieldType::Uuid => "string",
            FieldType::Enum => "string",
            FieldType::Binary => "string",
            FieldType::Inet => "string",
        }
    }

    /// Check if field should be nullable in PHP
    pub fn is_nullable_in_php(field_name: &str, nullable: bool) -> bool {
        // ID fields are never nullable in PHP constructors
        if field_name == "id" {
            return false;
        }
        nullable
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{FieldType, FillableGuarded};

    fn create_test_model() -> ModelDefinition {
        ModelDefinition {
            name: "User".to_string(),
            table: "users".to_string(),
            fields: vec![],
            timestamps: true,
            soft_deletes: false,
            relationships: vec![],
            pivot_tables: vec![],
            validation_rules: vec![],
            traits: vec![],
            fillable_guarded: FillableGuarded::All,
        }
    }

    fn create_test_config(use_ddd: bool) -> Config {
        Config {
            models: vec![],
            output_dir: "/tmp/test".to_string(),
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
    fn test_path_resolver_traditional_structure() {
        let model = create_test_model();
        let config = create_test_config(false);

        assert_eq!(
            PathResolver::get_model_path(&model, &config),
            "/tmp/test/app/Models/User.php"
        );

        assert_eq!(
            PathResolver::get_resource_path(&model, &config),
            "/tmp/test/app/Http/Resources/UserResource.php"
        );
        assert_eq!(
            PathResolver::get_factory_path(&model, &config),
            "/tmp/test/database/factories/UserFactory.php"
        );
        assert_eq!(
            PathResolver::get_dto_path(&model, &config),
            "/tmp/test/app/DTOs/UserDTO.php"
        );
    }

    #[test]
    fn test_path_resolver_ddd_structure() {
        let model = create_test_model();
        let config = create_test_config(true);

        assert_eq!(
            PathResolver::get_model_path(&model, &config),
            "/tmp/test/app/Domain/User/Models/User.php"
        );

        assert_eq!(
            PathResolver::get_resource_path(&model, &config),
            "/tmp/test/app/Domain/User/Resources/UserResource.php"
        );
        assert_eq!(
            PathResolver::get_factory_path(&model, &config),
            "/tmp/test/app/Domain/User/Factories/UserFactory.php"
        );
        assert_eq!(
            PathResolver::get_dto_path(&model, &config),
            "/tmp/test/app/Domain/User/DTOs/UserDTO.php"
        );
    }

    #[test]
    fn test_namespace_resolver_traditional_structure() {
        let model = create_test_model();
        let config = create_test_config(false);

        assert_eq!(
            NamespaceResolver::get_model_namespace(&model, &config),
            "App\\Models"
        );

        assert_eq!(
            NamespaceResolver::get_resource_namespace(&model, &config),
            "App\\Http\\Resources"
        );
        assert_eq!(
            NamespaceResolver::get_factory_namespace(&model, &config),
            "Database\\Factories"
        );
        assert_eq!(
            NamespaceResolver::get_dto_namespace(&model, &config),
            "App\\DTOs"
        );
    }

    #[test]
    fn test_namespace_resolver_ddd_structure() {
        let model = create_test_model();
        let config = create_test_config(true);

        assert_eq!(
            NamespaceResolver::get_model_namespace(&model, &config),
            "App\\Domain\\User\\Models"
        );

        assert_eq!(
            NamespaceResolver::get_resource_namespace(&model, &config),
            "App\\Domain\\User\\Resources"
        );
        assert_eq!(
            NamespaceResolver::get_factory_namespace(&model, &config),
            "App\\Domain\\User\\Factories"
        );
        assert_eq!(
            NamespaceResolver::get_dto_namespace(&model, &config),
            "App\\Domain\\User\\DTOs"
        );
    }

    #[test]
    fn test_field_type_helper_php_type_hints() {
        assert_eq!(FieldTypeHelper::to_php_type_hint(&FieldType::String), "string");
        assert_eq!(FieldTypeHelper::to_php_type_hint(&FieldType::Integer), "int");
        assert_eq!(FieldTypeHelper::to_php_type_hint(&FieldType::BigInteger), "int");
        assert_eq!(FieldTypeHelper::to_php_type_hint(&FieldType::Boolean), "bool");
        assert_eq!(FieldTypeHelper::to_php_type_hint(&FieldType::Float), "float");
        assert_eq!(FieldTypeHelper::to_php_type_hint(&FieldType::Decimal), "float");
        assert_eq!(FieldTypeHelper::to_php_type_hint(&FieldType::Json), "array");
        assert_eq!(FieldTypeHelper::to_php_type_hint(&FieldType::Date), "string");
        assert_eq!(FieldTypeHelper::to_php_type_hint(&FieldType::DateTime), "string");
        assert_eq!(FieldTypeHelper::to_php_type_hint(&FieldType::Timestamp), "string");
    }

    #[test]
    fn test_field_type_helper_nullable_in_php() {
        // ID fields are never nullable in PHP constructors
        assert_eq!(FieldTypeHelper::is_nullable_in_php("id", true), false);
        assert_eq!(FieldTypeHelper::is_nullable_in_php("id", false), false);

        // Other fields respect the nullable flag
        assert_eq!(FieldTypeHelper::is_nullable_in_php("name", true), true);
        assert_eq!(FieldTypeHelper::is_nullable_in_php("name", false), false);
        assert_eq!(FieldTypeHelper::is_nullable_in_php("email", true), true);
        assert_eq!(FieldTypeHelper::is_nullable_in_php("email", false), false);
    }
}
