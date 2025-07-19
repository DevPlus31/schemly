use crate::generators::Generator;
use crate::generators::shared::{PathResolver, NamespaceResolver, FieldTypeHelper};
use crate::types::{Config, ModelDefinition, Field};
use crate::validation::Validator;
use crate::template::{TemplateContext, TemplateRenderer};

// Type aliases for better readability
type GeneratorResult<T> = crate::error::Result<T>;
type FieldList = Vec<String>;



/// Generator for Laravel DTO (Data Transfer Object) classes
///
/// Supports both traditional Laravel structure (app/DTOs/) and
/// Domain-Driven Design structure (app/Domain/{Model}/DTOs/)
pub struct DtoGenerator;

// Template constants
const TEMPLATE: &str = include_str!("../templates/dto.php.template");

// Template variable names
mod template_vars {
    pub const NAMESPACE: &str = "namespace";
    pub const DTO_NAME: &str = "dto_name";
    pub const CONSTRUCTOR_FIELDS: &str = "constructor_fields";
    pub const FROM_ARRAY_FIELDS: &str = "from_array_fields";
    pub const TO_ARRAY_FIELDS: &str = "to_array_fields";
}

const REQUIRED_TEMPLATE_VARS: &[&str] = &[
    template_vars::NAMESPACE,
    template_vars::DTO_NAME,
    template_vars::CONSTRUCTOR_FIELDS,
    template_vars::FROM_ARRAY_FIELDS,
    template_vars::TO_ARRAY_FIELDS,
];

// Field formatting constants - indentation for different sections
const CONSTRUCTOR_FIELD_INDENT: &str = "        ";
const FROM_ARRAY_FIELD_INDENT: &str = "                ";
const TO_ARRAY_FIELD_INDENT: &str = "            ";

// Standard field names
mod standard_fields {
    pub const ID: &str = "id";
    pub const CREATED_AT: &str = "created_at";
    pub const UPDATED_AT: &str = "updated_at";
    pub const DELETED_AT: &str = "deleted_at";
}

impl Generator for DtoGenerator {
    fn generate(&self, model: &ModelDefinition, config: &Config) -> GeneratorResult<String> {
        self.validate_inputs(model, config)?;
        let context = self.build_template_context(model, config)?;
        self.render_template(&context)
    }

    fn get_file_path(&self, model: &ModelDefinition, config: &Config) -> String {
        PathResolver::get_dto_path(model, config)
    }
}

impl DtoGenerator {
    /// Validates model and configuration inputs
    fn validate_inputs(&self, model: &ModelDefinition, config: &Config) -> GeneratorResult<()> {
        Validator::validate_model(model)?;

        if config.output_dir.is_empty() {
            return Err(crate::error::GeneratorError::Configuration(
                "Output directory cannot be empty".to_string()
            ));
        }

        Validator::validate_identifier(&model.name, "DTO class name")?;
        Ok(())
    }

    /// Builds the template context with all required variables
    fn build_template_context(&self, model: &ModelDefinition, config: &Config) -> GeneratorResult<TemplateContext> {
        let namespace = NamespaceResolver::get_dto_namespace(model, config);
        let constructor_fields = self.generate_constructor_fields(model)?;
        let from_array_fields = self.generate_from_array_fields(model)?;
        let to_array_fields = self.generate_to_array_fields(model)?;

        let context = TemplateContext::new()
            .with(template_vars::NAMESPACE, format!("namespace {};", namespace))
            .with(template_vars::DTO_NAME, &model.name)
            .with(template_vars::CONSTRUCTOR_FIELDS, constructor_fields)
            .with(template_vars::FROM_ARRAY_FIELDS, from_array_fields)
            .with(template_vars::TO_ARRAY_FIELDS, to_array_fields);

        Ok(context)
    }

    /// Renders the DTO template with the provided context
    fn render_template(&self, context: &TemplateContext) -> GeneratorResult<String> {
        TemplateRenderer::render_with_required_vars(
            TEMPLATE,
            context,
            REQUIRED_TEMPLATE_VARS
        )
    }

    /// Generates constructor field declarations
    fn generate_constructor_fields(&self, model: &ModelDefinition) -> GeneratorResult<String> {
        let mut fields = vec![format!("public int ${}", standard_fields::ID)];

        // Add model fields
        fields.extend(self.generate_model_constructor_fields(&model.fields)?);

        // Add timestamp fields
        fields.extend(self.generate_timestamp_constructor_fields(model));

        Ok(fields.join(&format!(",\n{}", CONSTRUCTOR_FIELD_INDENT)))
    }

    /// Generates model-specific constructor fields
    fn generate_model_constructor_fields(&self, fields: &[Field]) -> GeneratorResult<FieldList> {
        let mut result = Vec::new();

        for field in fields {
            if field.name != standard_fields::ID {
                Validator::validate_identifier(&field.name, "Field name")?;

                let php_type = FieldTypeHelper::to_php_type_hint(&field.field_type);
                let nullable_prefix = if FieldTypeHelper::is_nullable_in_php(&field.name, field.nullable) {
                    "?"
                } else {
                    ""
                };

                result.push(format!("public {}{} ${}", nullable_prefix, php_type, field.name));
            }
        }

        Ok(result)
    }

    /// Generates timestamp-related constructor fields
    fn generate_timestamp_constructor_fields(&self, model: &ModelDefinition) -> FieldList {
        let mut fields = Vec::new();

        if model.timestamps {
            fields.extend_from_slice(&[
                format!("public ?string ${}", standard_fields::CREATED_AT),
                format!("public ?string ${}", standard_fields::UPDATED_AT),
            ]);
        }

        if model.soft_deletes {
            fields.push(format!("public ?string ${}", standard_fields::DELETED_AT));
        }

        fields
    }

    /// Generates fromArray method field assignments
    fn generate_from_array_fields(&self, model: &ModelDefinition) -> GeneratorResult<String> {
        let mut fields = vec![format!("$data['{}']", standard_fields::ID)];

        // Add model fields
        fields.extend(self.generate_model_from_array_fields(&model.fields)?);

        // Add timestamp fields
        fields.extend(self.generate_timestamp_from_array_fields(model));

        Ok(fields.join(&format!(",\n{}", FROM_ARRAY_FIELD_INDENT)))
    }

    /// Generates model-specific fromArray fields
    fn generate_model_from_array_fields(&self, fields: &[Field]) -> GeneratorResult<FieldList> {
        let mut result = Vec::new();

        for field in fields {
            if field.name != standard_fields::ID {
                Validator::validate_identifier(&field.name, "Field name")?;
                result.push(format!("$data['{}']", field.name));
            }
        }

        Ok(result)
    }

    /// Generates timestamp-related fromArray fields
    fn generate_timestamp_from_array_fields(&self, model: &ModelDefinition) -> FieldList {
        let mut fields = Vec::new();

        if model.timestamps {
            fields.extend_from_slice(&[
                format!("$data['{}']", standard_fields::CREATED_AT),
                format!("$data['{}']", standard_fields::UPDATED_AT),
            ]);
        }

        if model.soft_deletes {
            fields.push(format!("$data['{}']", standard_fields::DELETED_AT));
        }

        fields
    }

    /// Generates toArray method field mappings
    fn generate_to_array_fields(&self, model: &ModelDefinition) -> GeneratorResult<String> {
        let mut fields = vec![format!("'{}' => $this->{}", standard_fields::ID, standard_fields::ID)];

        // Add model fields
        fields.extend(self.generate_model_to_array_fields(&model.fields)?);

        // Add timestamp fields
        fields.extend(self.generate_timestamp_to_array_fields(model));

        Ok(fields.join(&format!(",\n{}", TO_ARRAY_FIELD_INDENT)))
    }

    /// Generates model-specific toArray fields
    fn generate_model_to_array_fields(&self, fields: &[Field]) -> GeneratorResult<FieldList> {
        let mut result = Vec::new();

        for field in fields {
            if field.name != standard_fields::ID {
                Validator::validate_identifier(&field.name, "Field name")?;
                result.push(format!("'{}' => $this->{}", field.name, field.name));
            }
        }

        Ok(result)
    }

    /// Generates timestamp-related toArray fields
    fn generate_timestamp_to_array_fields(&self, model: &ModelDefinition) -> FieldList {
        let mut fields = Vec::new();

        if model.timestamps {
            fields.extend_from_slice(&[
                format!("'{}' => $this->{}", standard_fields::CREATED_AT, standard_fields::CREATED_AT),
                format!("'{}' => $this->{}", standard_fields::UPDATED_AT, standard_fields::UPDATED_AT),
            ]);
        }

        if model.soft_deletes {
            fields.push(format!("'{}' => $this->{}", standard_fields::DELETED_AT, standard_fields::DELETED_AT));
        }

        fields
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Field, FieldType, FillableGuarded};

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
    fn test_dto_generation_traditional_structure() {
        let generator = DtoGenerator;
        let model = create_test_model();
        let config = create_test_config(false);

        let result = generator.generate(&model, &config).unwrap();

        // Check namespace
        assert!(result.contains("namespace App\\DTOs;"));

        // Check class name
        assert!(result.contains("class UserDTO {"));

        // Check constructor fields
        assert!(result.contains("public int $id"));
        assert!(result.contains("public string $name"));
        assert!(result.contains("public string $email"));
        assert!(result.contains("public ?int $age"));
        assert!(result.contains("public ?string $created_at"));
        assert!(result.contains("public ?string $updated_at"));

        // Check fromArray method
        assert!(result.contains("$data['id']"));
        assert!(result.contains("$data['name']"));
        assert!(result.contains("$data['email']"));
        assert!(result.contains("$data['age']"));

        // Check toArray method
        assert!(result.contains("'id' => $this->id"));
        assert!(result.contains("'name' => $this->name"));
        assert!(result.contains("'email' => $this->email"));
        assert!(result.contains("'age' => $this->age"));
    }

    #[test]
    fn test_dto_generation_ddd_structure() {
        let generator = DtoGenerator;
        let model = create_test_model();
        let config = create_test_config(true);

        let result = generator.generate(&model, &config).unwrap();

        // Check DDD namespace
        assert!(result.contains("namespace App\\Domain\\User\\DTOs;"));

        // Check class name
        assert!(result.contains("class UserDTO {"));
    }

    #[test]
    fn test_dto_file_path_traditional() {
        let generator = DtoGenerator;
        let model = create_test_model();
        let config = create_test_config(false);

        let path = generator.get_file_path(&model, &config);
        assert_eq!(path, "/tmp/test/app/DTOs/UserDTO.php");
    }

    #[test]
    fn test_dto_file_path_ddd() {
        let generator = DtoGenerator;
        let model = create_test_model();
        let config = create_test_config(true);

        let path = generator.get_file_path(&model, &config);
        assert_eq!(path, "/tmp/test/app/Domain/User/DTOs/UserDTO.php");
    }

    #[test]
    fn test_constructor_fields_generation() {
        let generator = DtoGenerator;
        let model = create_test_model();

        let result = generator.generate_constructor_fields(&model).unwrap();

        assert!(result.contains("public int $id"));
        assert!(result.contains("public string $name"));
        assert!(result.contains("public string $email"));
        assert!(result.contains("public ?int $age"));
        assert!(result.contains("public ?string $created_at"));
        assert!(result.contains("public ?string $updated_at"));
    }

    #[test]
    fn test_from_array_fields_generation() {
        let generator = DtoGenerator;
        let model = create_test_model();

        let result = generator.generate_from_array_fields(&model).unwrap();

        assert!(result.contains("$data['id']"));
        assert!(result.contains("$data['name']"));
        assert!(result.contains("$data['email']"));
        assert!(result.contains("$data['age']"));
        assert!(result.contains("$data['created_at']"));
        assert!(result.contains("$data['updated_at']"));
    }

    #[test]
    fn test_to_array_fields_generation() {
        let generator = DtoGenerator;
        let model = create_test_model();

        let result = generator.generate_to_array_fields(&model).unwrap();

        assert!(result.contains("'id' => $this->id"));
        assert!(result.contains("'name' => $this->name"));
        assert!(result.contains("'email' => $this->email"));
        assert!(result.contains("'age' => $this->age"));
        assert!(result.contains("'created_at' => $this->created_at"));
        assert!(result.contains("'updated_at' => $this->updated_at"));
    }

    #[test]
    fn test_soft_deletes_support() {
        let generator = DtoGenerator;
        let mut model = create_test_model();
        model.soft_deletes = true;

        let constructor_result = generator.generate_constructor_fields(&model).unwrap();
        let from_array_result = generator.generate_from_array_fields(&model).unwrap();
        let to_array_result = generator.generate_to_array_fields(&model).unwrap();

        assert!(constructor_result.contains("public ?string $deleted_at"));
        assert!(from_array_result.contains("$data['deleted_at']"));
        assert!(to_array_result.contains("'deleted_at' => $this->deleted_at"));
    }

    #[test]
    fn test_no_timestamps() {
        let generator = DtoGenerator;
        let mut model = create_test_model();
        model.timestamps = false;

        let constructor_result = generator.generate_constructor_fields(&model).unwrap();
        let from_array_result = generator.generate_from_array_fields(&model).unwrap();
        let to_array_result = generator.generate_to_array_fields(&model).unwrap();

        assert!(!constructor_result.contains("created_at"));
        assert!(!constructor_result.contains("updated_at"));
        assert!(!from_array_result.contains("created_at"));
        assert!(!from_array_result.contains("updated_at"));
        assert!(!to_array_result.contains("created_at"));
        assert!(!to_array_result.contains("updated_at"));
    }

    #[test]
    fn test_validation_error_invalid_model_name() {
        let generator = DtoGenerator;
        let mut model = create_test_model();
        model.name = "123InvalidName".to_string(); // Invalid: starts with number
        let config = create_test_config(false);

        let result = generator.generate(&model, &config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("must start with a letter"));
    }

    #[test]
    fn test_validation_error_empty_model_name() {
        let generator = DtoGenerator;
        let mut model = create_test_model();
        model.name = "".to_string(); // Invalid: empty name
        let config = create_test_config(false);

        let result = generator.generate(&model, &config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_validation_error_invalid_field_name() {
        let generator = DtoGenerator;
        let mut model = create_test_model();
        model.fields[0].name = "invalid-field".to_string(); // Invalid: contains hyphen
        let config = create_test_config(false);

        let result = generator.generate(&model, &config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("invalid characters"));
    }

    #[test]
    fn test_validation_error_php_reserved_word() {
        let generator = DtoGenerator;
        let mut model = create_test_model();
        model.name = "Class".to_string(); // Invalid: PHP reserved word
        let config = create_test_config(false);

        let result = generator.generate(&model, &config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("PHP reserved word"));
    }

    #[test]
    fn test_validation_error_empty_output_dir() {
        let generator = DtoGenerator;
        let model = create_test_model();
        let mut config = create_test_config(false);
        config.output_dir = "".to_string(); // Invalid: empty output directory

        let result = generator.generate(&model, &config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Output directory cannot be empty"));
    }

    #[test]
    fn test_validation_error_model_with_no_fields_no_timestamps() {
        let generator = DtoGenerator;
        let mut model = create_test_model();
        model.fields = vec![]; // No fields
        model.timestamps = false; // No timestamps
        let config = create_test_config(false);

        let result = generator.generate(&model, &config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("must have at least one field or timestamps"));
    }

    #[test]
    fn test_validation_error_duplicate_field_names() {
        let generator = DtoGenerator;
        let mut model = create_test_model();
        // Add duplicate field name
        let duplicate_field = model.fields[0].clone();
        model.fields.push(duplicate_field);
        let config = create_test_config(false);

        let result = generator.generate(&model, &config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Duplicate field name"));
    }

    #[test]
    fn test_validation_error_manual_id_field() {
        let generator = DtoGenerator;
        let mut model = create_test_model();
        // Add manual ID field (should be auto-generated)
        let mut id_field = model.fields[0].clone();
        id_field.name = "id".to_string();
        model.fields.push(id_field);
        let config = create_test_config(false);

        let result = generator.generate(&model, &config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("should not manually define 'id' field"));
    }
}