use crate::error::Result;
use crate::generators::Generator;
use crate::generators::shared::{PathResolver, NamespaceResolver};
use crate::types::{Config, ModelDefinition, Relationship};

pub struct ModelGenerator;

impl Generator for ModelGenerator {
    fn generate(&self, model: &ModelDefinition, config: &Config) -> Result<String> {
        let mut content = String::new();

        // PHP opening tag and namespace
        content.push_str("<?php\n\n");
        let namespace = NamespaceResolver::get_model_namespace(model, config);
        content.push_str(&format!("namespace {};\n\n", namespace));

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
        PathResolver::get_model_path(model, config)
    }
}

impl ModelGenerator {
    fn get_relationship_method_name(&self, relationship: &Relationship) -> String {
        match relationship {
            Relationship::BelongsTo(rel) => {
                // Convert StudlyCase to camelCase for the method name
                let first_char = rel.model.chars().next().unwrap().to_lowercase().to_string();
                let rest = &rel.model[1..];
                format!("{}{}", first_char, rest)
            },
            Relationship::HasMany(rel) | Relationship::BelongsToMany(rel) => {
                // Pluralize the model name for the method name
                self.pluralize_model_name(&rel.model)
            },
            Relationship::MorphMany(rel) | Relationship::MorphToMany(rel) => {
                // Pluralize the model name for the method name
                self.pluralize_model_name(&rel.model)
            },
            Relationship::HasOne(rel) => {
                // Convert StudlyCase to camelCase for the method name
                let first_char = rel.model.chars().next().unwrap().to_lowercase().to_string();
                let rest = &rel.model[1..];
                format!("{}{}", first_char, rest)
            },
            Relationship::MorphOne(rel) => {
                // Convert StudlyCase to camelCase for the method name
                let first_char = rel.model.chars().next().unwrap().to_lowercase().to_string();
                let rest = &rel.model[1..];
                format!("{}{}", first_char, rest)
            },
            Relationship::MorphTo(rel) => {
                rel.morph_name.clone()
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


        match relationship {
            Relationship::BelongsTo(rel) => {
                if let Some(foreign_key) = &rel.foreign_key {
                    format!("    public function {}()\n    {{\n        return $this->belongsTo({}::class, '{}');\n    }}\n\n",
                            method_name, rel.model, foreign_key)
                } else {
                    format!("    public function {}()\n    {{\n        return $this->belongsTo({}::class);\n    }}\n\n",
                            method_name, rel.model)
                }
            },
            Relationship::HasMany(rel) => {
                if let Some(foreign_key) = &rel.foreign_key {
                    format!("    public function {}()\n    {{\n        return $this->hasMany({}::class, '{}');\n    }}\n\n",
                            method_name, rel.model, foreign_key)
                } else {
                    format!("    public function {}()\n    {{\n        return $this->hasMany({}::class);\n    }}\n\n",
                            method_name, rel.model)
                }
            },
            Relationship::HasOne(rel) => {
                if let Some(foreign_key) = &rel.foreign_key {
                    format!("    public function {}()\n    {{\n        return $this->hasOne({}::class, '{}');\n    }}\n\n",
                            method_name, rel.model, foreign_key)
                } else {
                    format!("    public function {}()\n    {{\n        return $this->hasOne({}::class);\n    }}\n\n",
                            method_name, rel.model)
                }
            },
            Relationship::BelongsToMany(rel) => {
                if let Some(pivot_table) = &rel.pivot_table {
                    format!("    public function {}()\n    {{\n        return $this->belongsToMany({}::class, '{}');\n    }}\n\n",
                            method_name, rel.model, pivot_table)
                } else {
                    format!("    public function {}()\n    {{\n        return $this->belongsToMany({}::class);\n    }}\n\n",
                            method_name, rel.model)
                }
            },
            Relationship::MorphTo(_rel) => {
                format!("    public function {}()\n    {{\n        return $this->morphTo();\n    }}\n\n", method_name)
            },
            Relationship::MorphOne(rel) => {
                format!("    public function {}()\n    {{\n        return $this->morphOne({}::class, '{}');\n    }}\n\n",
                        method_name, rel.model, rel.morph_name)
            },
            Relationship::MorphMany(rel) => {
                format!("    public function {}()\n    {{\n        return $this->morphMany({}::class, '{}');\n    }}\n\n",
                        method_name, rel.model, rel.morph_name)
            },
            Relationship::MorphToMany(rel) => {
                if let Some(pivot_table) = &rel.pivot_table {
                    format!("    public function {}()\n    {{\n        return $this->morphToMany({}::class, '{}', '{}');\n    }}\n\n",
                            method_name, rel.model, rel.morph_name, pivot_table)
                } else {
                    format!("    public function {}()\n    {{\n        return $this->morphToMany({}::class, '{}');\n    }}\n\n",
                            method_name, rel.model, rel.morph_name)
                }
            },
        }
    }

}