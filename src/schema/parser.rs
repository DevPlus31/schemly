use pest::Parser;
use pest::iterators::Pair;
use crate::schema::ast::*;

#[derive(pest_derive::Parser)]
#[grammar = "grammar/schemly.pest"]
pub struct SchemlyParser;

pub fn parse_schema(input: &str) -> Result<Schema, String> {
    let pairs = SchemlyParser::parse(Rule::schema, input)
        .map_err(|e| format!("Parse error: {}", e))?;

    let mut schema = Schema::new();

    for pair in pairs {
        if pair.as_rule() == Rule::schema {
            for inner_pair in pair.into_inner() {
                match inner_pair.as_rule() {
                    Rule::generator_block => {
                        let generator = parse_generator(inner_pair)?;
                        schema.generators.push(generator);
                    }
                    Rule::datasource_block => {
                        let datasource = parse_datasource(inner_pair)?;
                        schema.datasources.push(datasource);
                    }
                    Rule::config_block => {
                        // Parse config block (we'll handle this later if needed)
                    }
                    Rule::model_block => {
                        let model = parse_model(inner_pair)?;
                        schema.add_model(model);
                    }
                    Rule::enum_block => {
                        let enum_def = parse_enum(inner_pair)?;
                        schema.add_enum(enum_def);
                    }
                    Rule::EOI => {}
                    _ => {}
                }
            }
        }
    }

    Ok(schema)
}

fn parse_model(pair: Pair<Rule>) -> Result<Model, String> {
    let mut inner = pair.into_inner();
    
    // First identifier is the model name
    let name = inner.next()
        .ok_or("Missing model name")?
        .as_str()
        .to_string();
    
    let mut model = Model::new(name);
    
    // Parse model contents (fields and attributes)
    for content_pair in inner {
        if content_pair.as_rule() == Rule::model_content {
            for item in content_pair.into_inner() {
                match item.as_rule() {
                    Rule::field_decl => {
                        let field = parse_field(item)?;
                        model.add_field(field);
                    }
                    Rule::attr_block => {
                        let attribute = parse_model_attribute(item)?;
                        model.add_attribute(attribute);
                    }
                    _ => {}
                }
            }
        }
    }
    
    Ok(model)
}

fn parse_field(pair: Pair<Rule>) -> Result<Field, String> {
    let mut inner = pair.into_inner();
    
    // Field name
    let name = inner.next()
        .ok_or("Missing field name")?
        .as_str()
        .to_string();
    
    // Field type
    let type_str = inner.next()
        .ok_or("Missing field type")?
        .as_str();
    
    let field_type = parse_field_type(type_str)?;
    
    let mut field = Field::new(name, field_type);
    
    // Parse modifiers and attributes
    for item in inner {
        match item.as_rule() {
            Rule::modifier_array => {
                field = field.with_list(true);
            }
            Rule::modifier_nullable => {
                field = field.with_optional(true);
            }
            Rule::attr_field => {
                let attribute = parse_field_attribute(item)?;
                field.add_attribute(attribute);
            }
            _ => {}
        }
    }
    
    Ok(field)
}

fn parse_field_type(type_str: &str) -> Result<FieldType, String> {
    match type_str {
        "String" => Ok(FieldType::String),
        "Int" => Ok(FieldType::Int),
        "BigInt" => Ok(FieldType::BigInt),
        "Float" => Ok(FieldType::Float),
        "Decimal" => Ok(FieldType::Decimal),
        "Boolean" => Ok(FieldType::Boolean),
        "DateTime" => Ok(FieldType::DateTime),
        "Json" => Ok(FieldType::Json),
        "Bytes" => Ok(FieldType::Bytes),
        _ => {
            // Check if it's a custom type (Model or Enum reference)
            if type_str.chars().next().unwrap_or('a').is_uppercase() {
                Ok(FieldType::Model(type_str.to_string()))
            } else {
                Ok(FieldType::Unsupported(type_str.to_string()))
            }
        }
    }
}

#[allow(clippy::collapsible_if)]
fn parse_field_attribute(pair: Pair<Rule>) -> Result<FieldAttribute, String> {
    let mut inner = pair.into_inner();

    let name = inner.next()
        .ok_or("Missing attribute name")?
        .as_str()
        .to_string();

    let mut args = Vec::new();

    // Parse arguments if present
    if let Some(args_pair) = inner.next() {
        if args_pair.as_rule() == Rule::args {
            for arg_pair in args_pair.into_inner() {
                if arg_pair.as_rule() == Rule::arg {
                    let mut arg_inner = arg_pair.into_inner();
                    let first = arg_inner.next().ok_or("Missing argument")?;

                    match first.as_rule() {
                        Rule::named_arg => {
                            let mut named_inner = first.into_inner();
                            let arg_name = named_inner.next()
                                .ok_or("Missing named argument name")?
                                .as_str()
                                .to_string();
                            let arg_value = parse_value(named_inner.next()
                                .ok_or("Missing named argument value")?)?;
                            args.push(AttributeArg::Named {
                                name: arg_name,
                                value: arg_value,
                            });
                        }
                        Rule::value => {
                            let value = parse_value(first)?;
                            args.push(AttributeArg::Positional(value));
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    Ok(FieldAttribute::new(name).with_args(args))
}

#[allow(clippy::collapsible_if)]
fn parse_model_attribute(pair: Pair<Rule>) -> Result<ModelAttribute, String> {
    let mut inner = pair.into_inner();

    let name = inner.next()
        .ok_or("Missing attribute name")?
        .as_str()
        .to_string();

    let mut args = Vec::new();

    // Parse arguments if present
    if let Some(args_pair) = inner.next() {
        if args_pair.as_rule() == Rule::args {
            for arg_pair in args_pair.into_inner() {
                if arg_pair.as_rule() == Rule::arg {
                    let mut arg_inner = arg_pair.into_inner();
                    let first = arg_inner.next().ok_or("Missing argument")?;

                    match first.as_rule() {
                        Rule::named_arg => {
                            let mut named_inner = first.into_inner();
                            let arg_name = named_inner.next()
                                .ok_or("Missing named argument name")?
                                .as_str()
                                .to_string();
                            let arg_value = parse_value(named_inner.next()
                                .ok_or("Missing named argument value")?)?;
                            args.push(AttributeArg::Named {
                                name: arg_name,
                                value: arg_value,
                            });
                        }
                        Rule::value => {
                            let value = parse_value(first)?;
                            args.push(AttributeArg::Positional(value));
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    Ok(ModelAttribute::new(name).with_args(args))
}

fn parse_value(pair: Pair<Rule>) -> Result<Value, String> {
    let inner = pair.into_inner().next()
        .ok_or("Empty value")?;
    
    match inner.as_rule() {
        Rule::array_lit => {
            let mut elements = Vec::new();
            for elem_pair in inner.into_inner() {
                if elem_pair.as_rule() == Rule::value {
                    elements.push(parse_value(elem_pair)?);
                }
            }
            Ok(Value::Array(elements))
        }
        Rule::function_call => {
            let mut func_inner = inner.into_inner();
            let func_name = func_inner.next()
                .ok_or("Missing function name")?
                .as_str()
                .to_string();

            let mut args = Vec::new();
            for arg_pair in func_inner {
                if arg_pair.as_rule() == Rule::value {
                    args.push(parse_value(arg_pair)?);
                }
            }

            Ok(Value::Function {
                name: func_name,
                args,
            })
        }
        Rule::string_lit => {
            let s = inner.as_str();
            // Remove quotes
            let unquoted = &s[1..s.len()-1];
            Ok(Value::String(unquoted.to_string()))
        }
        Rule::int_lit => {
            let i = inner.as_str().parse::<i64>()
                .map_err(|e| format!("Invalid integer: {}", e))?;
            Ok(Value::Integer(i))
        }
        Rule::bool_lit => {
            let b = inner.as_str() == "true";
            Ok(Value::Boolean(b))
        }
        Rule::identifier => {
            let id = inner.as_str().to_string();
            Ok(Value::FieldReference(id))
        }
        _ => Err(format!("Unexpected value type: {:?}", inner.as_rule()))
    }
}

fn parse_enum(pair: Pair<Rule>) -> Result<Enum, String> {
    let mut inner = pair.into_inner();

    let name = inner.next()
        .ok_or("Missing enum name")?
        .as_str()
        .to_string();

    let mut values = Vec::new();

    for value_pair in inner {
        if value_pair.as_rule() == Rule::enum_val {
            let value_name = value_pair.into_inner().next()
                .ok_or("Missing enum value")?
                .as_str()
                .to_string();

            values.push(EnumValue {
                name: value_name,
                attributes: Vec::new(),
            });
        }
    }

    Ok(Enum {
        name,
        values,
        attributes: Vec::new(),
    })
}

fn parse_generator(pair: Pair<Rule>) -> Result<Generator, String> {
    let mut inner = pair.into_inner();

    let name = inner.next()
        .ok_or("Missing generator name")?
        .as_str()
        .to_string();

    let mut properties = std::collections::HashMap::new();

    for prop_pair in inner {
        if prop_pair.as_rule() == Rule::generator_pair {
            let mut prop_inner = prop_pair.into_inner();
            let key = prop_inner.next()
                .ok_or("Missing generator property key")?
                .as_str()
                .to_string();
            let value_pair = prop_inner.next()
                .ok_or("Missing generator property value")?;

            let value = parse_value(value_pair)?;
            properties.insert(key, value);
        }
    }

    Ok(Generator {
        name,
        properties,
    })
}

fn parse_datasource(pair: Pair<Rule>) -> Result<Datasource, String> {
    let mut inner = pair.into_inner();

    let name = inner.next()
        .ok_or("Missing datasource name")?
        .as_str()
        .to_string();

    let mut properties = std::collections::HashMap::new();

    for prop_pair in inner {
        if prop_pair.as_rule() == Rule::datasource_pair {
            let mut prop_inner = prop_pair.into_inner();
            let key = prop_inner.next()
                .ok_or("Missing datasource property key")?
                .as_str()
                .to_string();
            let value_pair = prop_inner.next()
                .ok_or("Missing datasource property value")?;

            let value = parse_value(value_pair)?;
            properties.insert(key, value);
        }
    }

    Ok(Datasource {
        name,
        properties,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_model() {
        let input = r#"
model User {
    id Int @id
    name String
}
        "#;

        let schema = parse_schema(input).unwrap();
        assert_eq!(schema.models.len(), 1);
        assert_eq!(schema.models[0].name, "User");
        assert_eq!(schema.models[0].fields.len(), 2);
    }

    #[test]
    fn test_parse_enum() {
        let input = r#"
enum Role {
    ADMIN
    USER
}
        "#;

        let schema = parse_schema(input).unwrap();
        assert_eq!(schema.enums.len(), 1);
        assert_eq!(schema.enums[0].name, "Role");
        assert_eq!(schema.enums[0].values.len(), 2);
    }
}

