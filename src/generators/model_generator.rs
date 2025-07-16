use crate::error::Result;
use crate::generators::Generator;
use crate::types::{Config, ModelDefinition, Relationship, RelationshipType};

pub struct ModelGenerator;

impl Generator for ModelGenerator {
    fn generate(&self, model: &ModelDefinition, config: &Config) -> Result<String> {
        let mut content = String::new();

        // PHP opening tag and namespace
        content.push_str("<?php\n\n");
        content.push_str(&format!("namespace {};\n\n", config.namespace));

        // Imports
        content.push_str("use Illuminate\\Database\\Eloquent\\Model;\n");
        if model.soft_deletes {
            content.push_str("use Illuminate\\Database\\Eloquent\\SoftDeletes;\n");
        }
        if config.generate_factories {
            content.push_str("use Illuminate\\Database\\Eloquent\\Factories\\HasFactory;\n");
        }
        content.push_str("\n");

        // Class declaration
        content.push_str(&format!("class {} extends Model\n{{\n", model.name));

        // Traits
        let mut traits = Vec::new();
        if config.generate_factories {
            traits.push("HasFactory");
        }
        if model.soft_deletes {
            traits.push("SoftDeletes");
        }
        if !traits.is_empty() {
            content.push_str(&format!("    use {};\n\n", traits.join(", ")));
        }

        // Table name
        content.push_str(&format!("    protected $table = '{}';\n\n", model.table));

        // Timestamps
        if !model.timestamps {
            content.push_str("    public $timestamps = false;\n\n");
        }

        // Fillable fields
        let fillable_fields: Vec<String> = model.fields
            .iter()
            .filter(|f| f.name != "id")
            .map(|f| format!("'{}'", f.name))
            .collect();

        if !fillable_fields.is_empty() {
            content.push_str("    protected $fillable = [\n");
            content.push_str(&format!("        {}\n", fillable_fields.join(",\n        ")));
            content.push_str("    ];\n\n");
        }

        // Casts
        let casts = self.build_casts(model);
        if !casts.is_empty() {
            content.push_str("    protected $casts = [\n");
            content.push_str(&casts);
            content.push_str("    ];\n\n");
        }

        // Relationships
        for relationship in &model.relationships {
            content.push_str(&self.build_relationship_method(relationship, config));
        }

        content.push_str("}\n");
        Ok(content)
    }

    fn get_file_path(&self, model: &ModelDefinition, config: &Config) -> String {
        format!("{}/app/Models/{}.php", config.output_dir, model.name)
    }
}

impl ModelGenerator {
    fn get_relationship_method_name(&self, relationship: &Relationship) -> String {
        match relationship.relationship_type {
            RelationshipType::BelongsTo => {
                // Convert StudlyCase to camelCase for the method name
                let first_char = relationship.model.chars().next().unwrap().to_lowercase().to_string();
                let rest = &relationship.model[1..];
                format!("{}{}", first_char, rest)
            },
            RelationshipType::HasMany | RelationshipType::BelongsToMany | RelationshipType::MorphMany | RelationshipType::MorphToMany => {
                // Pluralize the model name for the method name
                self.pluralize_model_name(&relationship.model)
            },
            RelationshipType::HasOne | RelationshipType::MorphOne => {
                // Convert StudlyCase to camelCase for the method name
                let first_char = relationship.model.chars().next().unwrap().to_lowercase().to_string();
                let rest = &relationship.model[1..];
                format!("{}{}", first_char, rest)
            },
            RelationshipType::MorphTo => {
                relationship.morph_name.clone().unwrap_or_else(|| "morphable".to_string())
            },
        }
    }

    fn pluralize_model_name(&self, model_name: &str) -> String {
        // Convert StudlyCase to camelCase and pluralize
        let first_char = model_name.chars().next().unwrap().to_lowercase().to_string();
        let rest = &model_name[1..];
        let singular = format!("{}{}", first_char, rest);

        if singular.ends_with('y') {
            format!("{}ies", &singular[..singular.len()-1])
        } else if singular.ends_with('s') {
            format!("{}es", singular)
        } else {
            format!("{}s", singular)
        }
    }

    fn model_name_to_table(&self, model_name: &str) -> String {
        let snake_case = model_name
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if i > 0 && c.is_uppercase() {
                    format!("_{}", c.to_lowercase())
                } else {
                    c.to_lowercase().to_string()
                }
            })
            .collect::<String>();

        if snake_case.ends_with('y') {
            format!("{}ies", &snake_case[..snake_case.len()-1])
        } else if snake_case.ends_with('s') {
            format!("{}es", snake_case)
        } else {
            format!("{}s", snake_case)
        }
    }
    fn build_casts(&self, model: &ModelDefinition) -> String {
        let mut casts = String::new();

        for field in &model.fields {
            if let Some(cast) = field.field_type.to_cast_type() {
                casts.push_str(&format!("        '{}' => '{}',\n", field.name, cast));
            }
        }

        casts
    }

    fn build_relationship_method(&self, relationship: &crate::types::Relationship, _config: &Config) -> String {
        let method_name = self.get_relationship_method_name(relationship);


        match relationship.relationship_type {
            RelationshipType::BelongsTo => {
                if let Some(foreign_key) = &relationship.foreign_key {
                    format!("    public function {}()\n    {{\n        return $this->belongsTo({}::class, '{}');\n    }}\n\n",
                            method_name, relationship.model, foreign_key)
                } else {
                    format!("    public function {}()\n    {{\n        return $this->belongsTo({}::class);\n    }}\n\n",
                            method_name, relationship.model)
                }
            },
            RelationshipType::HasMany => {
                if let Some(foreign_key) = &relationship.foreign_key {
                    format!("    public function {}()\n    {{\n        return $this->hasMany({}::class, '{}');\n    }}\n\n",
                            method_name, relationship.model, foreign_key)
                } else {
                    format!("    public function {}()\n    {{\n        return $this->hasMany({}::class);\n    }}\n\n",
                            method_name, relationship.model)
                }
            },
            RelationshipType::HasOne => {
                if let Some(foreign_key) = &relationship.foreign_key {
                    format!("    public function {}()\n    {{\n        return $this->hasOne({}::class, '{}');\n    }}\n\n",
                            method_name, relationship.model, foreign_key)
                } else {
                    format!("    public function {}()\n    {{\n        return $this->hasOne({}::class);\n    }}\n\n",
                            method_name, relationship.model)
                }
            },
            RelationshipType::BelongsToMany => {
                if let Some(pivot_table) = &relationship.pivot_table {
                    format!("    public function {}()\n    {{\n        return $this->belongsToMany({}::class, '{}');\n    }}\n\n",
                            method_name, relationship.model, pivot_table)
                } else {
                    format!("    public function {}()\n    {{\n        return $this->belongsToMany({}::class);\n    }}\n\n",
                            method_name, relationship.model)
                }
            },
            RelationshipType::MorphTo => {
                let morph_name = relationship.morph_name.as_deref().unwrap_or("morphable");
                format!("    public function {}()\n    {{\n        return $this->morphTo('{}');\n    }}\n\n",
                        method_name, morph_name)
            },
            RelationshipType::MorphOne => {
                let morph_name = relationship.morph_name.as_deref().unwrap_or("morphable");
                format!("    public function {}()\n    {{\n        return $this->morphOne({}::class, '{}');\n    }}\n\n",
                        method_name, relationship.model, morph_name)
            },
            RelationshipType::MorphMany => {
                let morph_name = relationship.morph_name.as_deref().unwrap_or("morphable");
                format!("    public function {}()\n    {{\n        return $this->morphMany({}::class, '{}');\n    }}\n\n",
                        method_name, relationship.model, morph_name)
            },
            RelationshipType::MorphToMany => {
                let morph_name = relationship.morph_name.as_deref().unwrap_or("morphable");
                if let Some(pivot_table) = &relationship.pivot_table {
                    format!("    public function {}()\n    {{\n        return $this->morphToMany({}::class, '{}', '{}');\n    }}\n\n",
                            method_name, relationship.model, morph_name, pivot_table)
                } else {
                    format!("    public function {}()\n    {{\n        return $this->morphToMany({}::class, '{}');\n    }}\n\n",
                            method_name, relationship.model, morph_name)
                }
            },
        }
    }
    
    fn to_camel_case(&self, s: &str) -> String {
        let mut result = String::new();
        let mut capitalize_next = false;

        for c in s.chars() {
            if c == '_' {
                capitalize_next = true;
            } else if capitalize_next {
                result.push(c.to_uppercase().next().unwrap());
                capitalize_next = false;
            } else {
                result.push(c.to_lowercase().next().unwrap());
            }
        }

        result
    }

    fn to_plural_camel_case(&self, s: &str) -> String {
        let camel = self.to_camel_case(s);
        if camel.ends_with("y") {
            format!("{}ies", &camel[..camel.len()-1])
        } else if camel.ends_with("s") {
            format!("{}es", camel)
        } else {
            format!("{}s", camel)
        }
    }
}