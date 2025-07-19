use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum FieldType {
    String,
    Text,
    Integer,
    BigInteger,
    Float,
    Decimal,
    Boolean,
    Date,
    DateTime,
    Timestamp,
    Json,
    Uuid,
    Enum,
    TinyInteger,
    SmallInteger,
    MediumInteger,
    LongText,
    MediumText,
    Binary,
    Inet,
}

impl FieldType {
    pub fn to_migration_type(&self) -> &'static str {
        match self {
            FieldType::String => "string",
            FieldType::Text => "text",
            FieldType::Integer => "integer",
            FieldType::BigInteger => "bigInteger",
            FieldType::Float => "float",
            FieldType::Decimal => "decimal",
            FieldType::Boolean => "boolean",
            FieldType::Date => "date",
            FieldType::DateTime => "dateTime",
            FieldType::Timestamp => "timestamp",
            FieldType::Json => "json",
            FieldType::Uuid => "uuid",
            FieldType::Enum => "enum",
            FieldType::TinyInteger => "tinyInteger",
            FieldType::SmallInteger => "smallInteger",
            FieldType::MediumInteger => "mediumInteger",
            FieldType::LongText => "longText",
            FieldType::MediumText => "mediumText",
            FieldType::Binary => "binary",
            FieldType::Inet => "ipAddress",
        }
    }

    pub fn to_cast_type(&self) -> Option<&'static str> {
        match self {
            FieldType::Boolean => Some("boolean"),
            FieldType::Integer | FieldType::BigInteger | FieldType::TinyInteger | FieldType::SmallInteger | FieldType::MediumInteger => Some("integer"),
            FieldType::Float | FieldType::Decimal => Some("float"),
            FieldType::Json => Some("array"),
            FieldType::DateTime | FieldType::Timestamp => Some("datetime"),
            FieldType::Date => Some("date"),
            _ => None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum RelationshipType {
    BelongsTo,
    HasMany,
    HasOne,
    BelongsToMany,
    MorphTo,
    MorphOne,
    MorphMany,
    MorphToMany,
}

// Standard relationship for most relationship types
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StandardRelationship {
    pub model: String,
    #[serde(default)]
    pub foreign_key: Option<String>,
    #[serde(default)]
    pub local_key: Option<String>,
    #[serde(default)]
    pub pivot_table: Option<String>,
    #[serde(default)]
    pub pivot_fields: Vec<String>,
    #[serde(default)]
    pub on_delete: Option<String>,
    #[serde(default)]
    pub on_update: Option<String>,
    #[serde(default)]
    pub with_timestamps: bool,
}

// Polymorphic relationship for morphTo (no model field needed)
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PolymorphicRelationship {
    pub morph_name: String,
    #[serde(default)]
    pub foreign_key: Option<String>,
    #[serde(default)]
    pub local_key: Option<String>,
}

// Polymorphic relationship for morphOne, morphMany, morphToMany
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PolymorphicWithModelRelationship {
    pub model: String,
    pub morph_name: String,
    #[serde(default)]
    pub foreign_key: Option<String>,
    #[serde(default)]
    pub local_key: Option<String>,
    #[serde(default)]
    pub pivot_table: Option<String>,
    #[serde(default)]
    pub pivot_fields: Vec<String>,
    #[serde(default)]
    pub on_delete: Option<String>,
    #[serde(default)]
    pub on_update: Option<String>,
    #[serde(default)]
    pub with_timestamps: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EnumValue {
    pub value: String,
    pub label: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DecimalPrecision {
    pub precision: u8,
    pub scale: u8,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ValidationRule {
    pub rule: String,
    pub parameters: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ModelDefinition {
    pub name: String,
    pub table: String,
    pub fields: Vec<Field>,
    #[serde(default)]
    pub timestamps: bool,
    #[serde(default)]
    pub soft_deletes: bool,
    #[serde(default)]
    pub relationships: Vec<Relationship>,
    #[serde(default)]
    pub pivot_tables: Vec<PivotTable>,
    #[serde(default)]
    pub validation_rules: Vec<ValidationRule>,
    #[serde(default)]
    pub traits: Vec<String>,
    #[serde(default)]
    pub fillable_guarded: FillableGuarded,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum FillableGuarded {
    Fillable(Vec<String>),
    Guarded(Vec<String>),
    All,
}

impl Default for FillableGuarded {
    fn default() -> Self {
        FillableGuarded::All
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: FieldType,
    #[serde(default)]
    pub nullable: bool,
    #[serde(default)]
    pub unique: bool,
    #[serde(default)]
    pub default: Option<String>,
    #[serde(default)]
    pub length: Option<u32>,
    #[serde(default)]
    pub index: bool,
    #[serde(default)]
    pub enum_values: Vec<EnumValue>,
    #[serde(default)]
    pub decimal_precision: Option<DecimalPrecision>,
    #[serde(default)]
    pub unsigned: bool,
    #[serde(default)]
    pub auto_increment: bool,
    #[serde(default)]
    pub primary: bool,
    #[serde(default)]
    pub comment: Option<String>,
    #[serde(default)]
    pub validation_rules: Vec<ValidationRule>,
    #[serde(default)]
    pub cast_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Relationship {
    BelongsTo(StandardRelationship),
    HasMany(StandardRelationship),
    HasOne(StandardRelationship),
    BelongsToMany(StandardRelationship),
    MorphTo(PolymorphicRelationship),
    MorphOne(PolymorphicWithModelRelationship),
    MorphMany(PolymorphicWithModelRelationship),
    MorphToMany(PolymorphicWithModelRelationship),
}

impl Relationship {
    pub fn get_model(&self) -> Option<&str> {
        match self {
            Relationship::BelongsTo(rel) => Some(&rel.model),
            Relationship::HasMany(rel) => Some(&rel.model),
            Relationship::HasOne(rel) => Some(&rel.model),
            Relationship::BelongsToMany(rel) => Some(&rel.model),
            Relationship::MorphTo(_) => None, // No model for morphTo
            Relationship::MorphOne(rel) => Some(&rel.model),
            Relationship::MorphMany(rel) => Some(&rel.model),
            Relationship::MorphToMany(rel) => Some(&rel.model),
        }
    }

    pub fn get_foreign_key(&self) -> Option<&str> {
        match self {
            Relationship::BelongsTo(rel) => rel.foreign_key.as_deref(),
            Relationship::HasMany(rel) => rel.foreign_key.as_deref(),
            Relationship::HasOne(rel) => rel.foreign_key.as_deref(),
            Relationship::BelongsToMany(rel) => rel.foreign_key.as_deref(),
            Relationship::MorphTo(rel) => rel.foreign_key.as_deref(),
            Relationship::MorphOne(rel) => rel.foreign_key.as_deref(),
            Relationship::MorphMany(rel) => rel.foreign_key.as_deref(),
            Relationship::MorphToMany(rel) => rel.foreign_key.as_deref(),
        }
    }

    pub fn get_local_key(&self) -> Option<&str> {
        match self {
            Relationship::BelongsTo(rel) => rel.local_key.as_deref(),
            Relationship::HasMany(rel) => rel.local_key.as_deref(),
            Relationship::HasOne(rel) => rel.local_key.as_deref(),
            Relationship::BelongsToMany(rel) => rel.local_key.as_deref(),
            Relationship::MorphTo(rel) => rel.local_key.as_deref(),
            Relationship::MorphOne(rel) => rel.local_key.as_deref(),
            Relationship::MorphMany(rel) => rel.local_key.as_deref(),
            Relationship::MorphToMany(rel) => rel.local_key.as_deref(),
        }
    }

    pub fn get_pivot_table(&self) -> Option<&str> {
        match self {
            Relationship::BelongsToMany(rel) => rel.pivot_table.as_deref(),
            Relationship::MorphToMany(rel) => rel.pivot_table.as_deref(),
            _ => None,
        }
    }

    pub fn get_morph_name(&self) -> Option<&str> {
        match self {
            Relationship::MorphTo(rel) => Some(&rel.morph_name),
            Relationship::MorphOne(rel) => Some(&rel.morph_name),
            Relationship::MorphMany(rel) => Some(&rel.morph_name),
            Relationship::MorphToMany(rel) => Some(&rel.morph_name),
            _ => None,
        }
    }

    pub fn get_on_delete(&self) -> Option<&str> {
        match self {
            Relationship::BelongsTo(rel) => rel.on_delete.as_deref(),
            Relationship::HasMany(rel) => rel.on_delete.as_deref(),
            Relationship::HasOne(rel) => rel.on_delete.as_deref(),
            Relationship::BelongsToMany(rel) => rel.on_delete.as_deref(),
            Relationship::MorphOne(rel) => rel.on_delete.as_deref(),
            Relationship::MorphMany(rel) => rel.on_delete.as_deref(),
            Relationship::MorphToMany(rel) => rel.on_delete.as_deref(),
            _ => None,
        }
    }

    pub fn get_on_update(&self) -> Option<&str> {
        match self {
            Relationship::BelongsTo(rel) => rel.on_update.as_deref(),
            Relationship::HasMany(rel) => rel.on_update.as_deref(),
            Relationship::HasOne(rel) => rel.on_update.as_deref(),
            Relationship::BelongsToMany(rel) => rel.on_update.as_deref(),
            Relationship::MorphOne(rel) => rel.on_update.as_deref(),
            Relationship::MorphMany(rel) => rel.on_update.as_deref(),
            Relationship::MorphToMany(rel) => rel.on_update.as_deref(),
            _ => None,
        }
    }

    pub fn get_with_timestamps(&self) -> bool {
        match self {
            Relationship::BelongsTo(rel) => rel.with_timestamps,
            Relationship::HasMany(rel) => rel.with_timestamps,
            Relationship::HasOne(rel) => rel.with_timestamps,
            Relationship::BelongsToMany(rel) => rel.with_timestamps,
            Relationship::MorphOne(rel) => rel.with_timestamps,
            Relationship::MorphMany(rel) => rel.with_timestamps,
            Relationship::MorphToMany(rel) => rel.with_timestamps,
            _ => false,
        }
    }

    pub fn get_relationship_type(&self) -> RelationshipType {
        match self {
            Relationship::BelongsTo(_) => RelationshipType::BelongsTo,
            Relationship::HasMany(_) => RelationshipType::HasMany,
            Relationship::HasOne(_) => RelationshipType::HasOne,
            Relationship::BelongsToMany(_) => RelationshipType::BelongsToMany,
            Relationship::MorphTo(_) => RelationshipType::MorphTo,
            Relationship::MorphOne(_) => RelationshipType::MorphOne,
            Relationship::MorphMany(_) => RelationshipType::MorphMany,
            Relationship::MorphToMany(_) => RelationshipType::MorphToMany,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PivotTable {
    pub name: String,
    pub model1: String,
    pub model2: String,
    pub foreign_key1: String,
    pub foreign_key2: String,
    #[serde(default)]
    pub additional_fields: Vec<Field>,
    #[serde(default)]
    pub timestamps: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub models: Vec<ModelDefinition>,
    #[serde(default)]
    pub output_dir: String,
    #[serde(default)]
    pub namespace: String,
    #[serde(default)]
    pub generate_models: bool,
    #[serde(default)]
    pub generate_controllers: bool,
    #[serde(default)]
    pub generate_resources: bool,
    #[serde(default)]
    pub generate_factories: bool,
    #[serde(default)]
    pub generate_migrations: bool,
    #[serde(default)]
    pub generate_pivot_tables: bool,
    #[serde(default)]
    pub generate_validation_rules: bool,
    #[serde(default)]
    pub generate_dto: bool,
    #[serde(default)]
    pub use_ddd_structure: bool,
    #[serde(default)]
    pub database_engine: String,
    #[serde(default)]
    pub force_overwrite: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            models: Vec::new(),
            output_dir: ".".to_string(),
            namespace: "App\\Models".to_string(),
            generate_models: true,
            generate_controllers: true,
            generate_resources: true,
            generate_factories: true,
            generate_migrations: true,
            generate_pivot_tables: true,
            generate_validation_rules: true,
            generate_dto: false,
            use_ddd_structure: false,
            database_engine: "mysql".to_string(),
            force_overwrite: false,
        }
    }
}