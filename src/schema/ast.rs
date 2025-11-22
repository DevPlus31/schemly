use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Schema {
    pub generators: Vec<Generator>,
    pub datasources: Vec<Datasource>,
    pub models: Vec<Model>,
    pub enums: Vec<Enum>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Generator {
    pub name: String,
    pub properties: HashMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Datasource {
    pub name: String,
    pub properties: HashMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Model {
    pub name: String,
    pub fields: Vec<Field>,
    pub attributes: Vec<ModelAttribute>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub name: String,
    pub field_type: FieldType,
    pub optional: bool,
    pub list: bool,
    pub attributes: Vec<FieldAttribute>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FieldType {
    // Scalar types
    String,
    Int,
    BigInt,
    Float,
    Decimal,
    Boolean,
    DateTime,
    Json,
    Bytes,
    
    // Custom types
    Model(String),
    Enum(String),
    
    // Unsupported (for future extension)
    Unsupported(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FieldAttribute {
    pub name: String,
    pub args: Vec<AttributeArg>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModelAttribute {
    pub name: String,
    pub args: Vec<AttributeArg>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AttributeArg {
    Named { name: String, value: Value },
    Positional(Value),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Array(Vec<Value>),
    Function { name: String, args: Vec<Value> },
    FieldReference(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Enum {
    pub name: String,
    pub values: Vec<EnumValue>,
    pub attributes: Vec<ModelAttribute>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumValue {
    pub name: String,
    pub attributes: Vec<FieldAttribute>,
}

impl Schema {
    pub fn new() -> Self {
        Self {
            generators: Vec::new(),
            datasources: Vec::new(),
            models: Vec::new(),
            enums: Vec::new(),
        }
    }

    pub fn add_generator(&mut self, generator: Generator) {
        self.generators.push(generator);
    }

    pub fn add_datasource(&mut self, datasource: Datasource) {
        self.datasources.push(datasource);
    }

    pub fn add_model(&mut self, model: Model) {
        self.models.push(model);
    }

    pub fn add_enum(&mut self, enum_def: Enum) {
        self.enums.push(enum_def);
    }

    pub fn find_model(&self, name: &str) -> Option<&Model> {
        self.models.iter().find(|m| m.name == name)
    }

    pub fn find_enum(&self, name: &str) -> Option<&Enum> {
        self.enums.iter().find(|e| e.name == name)
    }
}

impl Model {
    pub fn new(name: String) -> Self {
        Self {
            name,
            fields: Vec::new(),
            attributes: Vec::new(),
        }
    }

    pub fn add_field(&mut self, field: Field) {
        self.fields.push(field);
    }

    pub fn add_attribute(&mut self, attribute: ModelAttribute) {
        self.attributes.push(attribute);
    }

    pub fn find_field(&self, name: &str) -> Option<&Field> {
        self.fields.iter().find(|f| f.name == name)
    }

    pub fn get_attribute(&self, name: &str) -> Option<&ModelAttribute> {
        self.attributes.iter().find(|a| a.name == name)
    }

    pub fn get_table_name(&self) -> String {
        if let Some(map_attr) = self.get_attribute("map") {
            if let Some(AttributeArg::Positional(Value::String(table_name))) = map_attr.args.first() {
                return table_name.clone();
            }
        }
        // Default to snake_case of model name
        to_snake_case(&self.name)
    }

    pub fn has_timestamps(&self) -> bool {
        self.fields.iter().any(|f| f.name == "createdAt" || f.name == "created_at") &&
        self.fields.iter().any(|f| f.name == "updatedAt" || f.name == "updated_at")
    }

    pub fn has_soft_deletes(&self) -> bool {
        self.fields.iter().any(|f| f.name == "deletedAt" || f.name == "deleted_at")
    }

    pub fn get_traits(&self) -> Vec<String> {
        if let Some(traits_attr) = self.get_attribute("traits") {
            if let Some(AttributeArg::Positional(Value::Array(traits))) = traits_attr.args.first() {
                return traits.iter()
                    .filter_map(|v| match v {
                        Value::String(s) => Some(s.clone()),
                        _ => None,
                    })
                    .collect();
            }
        }
        Vec::new()
    }

    pub fn get_fillable(&self) -> Vec<String> {
        if let Some(fillable_attr) = self.get_attribute("fillable") {
            if let Some(AttributeArg::Positional(Value::Array(fields))) = fillable_attr.args.first() {
                return fields.iter()
                    .filter_map(|v| match v {
                        Value::String(s) => Some(s.clone()),
                        _ => None,
                    })
                    .collect();
            }
        }
        Vec::new()
    }
}

impl Field {
    pub fn new(name: String, field_type: FieldType) -> Self {
        Self {
            name,
            field_type,
            optional: false,
            list: false,
            attributes: Vec::new(),
        }
    }

    pub fn with_optional(mut self, optional: bool) -> Self {
        self.optional = optional;
        self
    }

    pub fn with_list(mut self, list: bool) -> Self {
        self.list = list;
        self
    }

    pub fn add_attribute(&mut self, attribute: FieldAttribute) {
        self.attributes.push(attribute);
    }

    pub fn get_attribute(&self, name: &str) -> Option<&FieldAttribute> {
        self.attributes.iter().find(|a| a.name == name)
    }

    pub fn is_id(&self) -> bool {
        self.get_attribute("id").is_some()
    }

    pub fn is_unique(&self) -> bool {
        self.get_attribute("unique").is_some()
    }

    pub fn get_default(&self) -> Option<&Value> {
        if let Some(default_attr) = self.get_attribute("default") {
            return default_attr.args.first().and_then(|arg| match arg {
                AttributeArg::Positional(value) => Some(value),
                _ => None,
            });
        }
        None
    }

    pub fn get_db_type(&self) -> Option<String> {
        if let Some(db_attr) = self.get_attribute("db") {
            if let Some(AttributeArg::Positional(Value::Function { name, args })) = db_attr.args.first() {
                if args.is_empty() {
                    return Some(name.clone());
                } else {
                    // Handle VarChar(255), Decimal(8,2), etc.
                    let args_str = args.iter()
                        .map(|v| match v {
                            Value::Integer(i) => i.to_string(),
                            Value::String(s) => s.clone(),
                            _ => "".to_string(),
                        })
                        .collect::<Vec<_>>()
                        .join(",");
                    return Some(format!("{}({})", name, args_str));
                }
            }
        }
        None
    }

    pub fn get_validation_rules(&self) -> Option<String> {
        if let Some(validate_attr) = self.get_attribute("validate") {
            if let Some(AttributeArg::Positional(Value::String(rules))) = validate_attr.args.first() {
                return Some(rules.clone());
            }
        }
        None
    }

    pub fn get_map_name(&self) -> String {
        if let Some(map_attr) = self.get_attribute("map") {
            if let Some(AttributeArg::Positional(Value::String(column_name))) = map_attr.args.first() {
                return column_name.clone();
            }
        }
        // Default to snake_case of field name
        to_snake_case(&self.name)
    }
}

impl FieldAttribute {
    pub fn new(name: String) -> Self {
        Self {
            name,
            args: Vec::new(),
        }
    }

    pub fn with_args(mut self, args: Vec<AttributeArg>) -> Self {
        self.args = args;
        self
    }
}

impl ModelAttribute {
    pub fn new(name: String) -> Self {
        Self {
            name,
            args: Vec::new(),
        }
    }

    pub fn with_args(mut self, args: Vec<AttributeArg>) -> Self {
        self.args = args;
        self
    }
}

// Utility function to convert PascalCase to snake_case
fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch.is_uppercase() && !result.is_empty() {
            result.push('_');
        }
        result.push(ch.to_lowercase().next().unwrap());
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("User"), "user");
        assert_eq!(to_snake_case("UserProfile"), "user_profile");
        assert_eq!(to_snake_case("XMLHttpRequest"), "x_m_l_http_request");
    }

    #[test]
    fn test_model_creation() {
        let mut model = Model::new("User".to_string());
        
        let field = Field::new("name".to_string(), FieldType::String)
            .with_optional(false);
        model.add_field(field);
        
        let attr = ModelAttribute::new("map".to_string())
            .with_args(vec![AttributeArg::Positional(Value::String("users".to_string()))]);
        model.add_attribute(attr);
        
        assert_eq!(model.name, "User");
        assert_eq!(model.fields.len(), 1);
        assert_eq!(model.get_table_name(), "users");
    }

    #[test]
    fn test_field_attributes() {
        let mut field = Field::new("id".to_string(), FieldType::Int);
        
        field.add_attribute(FieldAttribute::new("id".to_string()));
        field.add_attribute(FieldAttribute::new("default".to_string())
            .with_args(vec![AttributeArg::Positional(Value::Function {
                name: "autoincrement".to_string(),
                args: vec![],
            })]));
        
        assert!(field.is_id());
        assert!(field.get_default().is_some());
    }
}
